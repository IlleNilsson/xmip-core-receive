#![forbid(unsafe_code)]

//! Receive Ports, and what arrives at a Receive Location before any gate has
//! run.
//!
//! A Receive Location is configured once, as `xmip-core-configure`'s
//! `ConfiguredLocation` — its transport, its address, its settings and the
//! closed set it accepts (ADR-0019 clause 1) — and the runtime builds the
//! Location's transport from it through `xmip-core-transport`, the one trait
//! every protocol implements in both directions (ADR-0010). What is here is
//! what neither of those holds: the alignment policy a Receive Location keeps
//! between the two identity layers, and the Stream as it comes off the
//! transport — its body still to be read — with how it got there and what
//! the transport observed.

use std::fmt;
use std::io::Read;

use context::{Alignment, OnMisalignment};
use identify::Presented;
use xcore::{Arriving, ArtifactId};

/// What a Receive Location does when the two identity layers disagree.
///
/// ADR-0019 clause 7. `none` and `accept` are the defaults because a default of
/// `strict` would refuse every relayed integration on the first day, and the
/// failure would present as a routing bug rather than a policy decision.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IdentityPolicy {
    pub alignment: Alignment,
    pub on_misalignment: OnMisalignment,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReceivePort {
    pub artifact_id: ArtifactId,
    pub name: String,
    pub version: String,
}

/// A Stream coming off a transport — its body a reader, not yet read — and
/// the credential the transport observed.
///
/// **The body is read once the transport gates have passed**, into the
/// Ledger a chunk at a time (`runtime-model.md` section 5): nothing holds
/// the whole Stream where the far end streams it, and a refused sender's
/// content is never read.
///
/// **Nothing here is authenticated.** The transport extracts what it can see —
/// a client certificate, an `Authorization` header, the path and permissions of
/// a drop folder — and the gate runs afterwards. This previously carried a
/// resolved `PartyId`, which presumed the answer to a question that had not yet
/// been asked.
pub struct ReceivedStream {
    /// The content, read as it is asked for.
    pub body: Box<dyn Read + Send>,

    /// How it got here: pushed, detected or scheduled.
    ///
    /// Set by the transport, which is the only thing that knows. A Receive
    /// Location watching a folder produces [`Arriving::Detected`]; the same
    /// folder polled on a timer produces [`Arriving::Scheduled`]; an HTTP
    /// endpoint produces [`Arriving::Pushed`]. Defaults to pushed because that
    /// is the case with a caller to answer to.
    pub arriving: Arriving,

    pub source_uri: String,
    /// What the transport observed. `None` only where the technology offers
    /// nothing at all to observe.
    pub presented: Option<Presented>,
    pub transport_properties: Vec<(String, String)>,
}

impl ReceivedStream {
    #[must_use]
    pub fn new(body: impl Read + Send + 'static, source_uri: impl Into<String>) -> Self {
        Self {
            body: Box::new(body),
            arriving: Arriving::Pushed,
            source_uri: source_uri.into(),
            presented: None,
            transport_properties: Vec::new(),
        }
    }

    /// Xmip was watching and it appeared. Nobody connected.
    #[must_use]
    pub const fn detected(mut self) -> Self {
        self.arriving = Arriving::Detected;
        self
    }

    /// A timer fired and Xmip went and fetched it. Xmip is the client, so any
    /// credential in play is Xmip's own and proves nothing about the source.
    #[must_use]
    pub const fn scheduled(mut self) -> Self {
        self.arriving = Arriving::Scheduled;
        self
    }

    #[must_use]
    pub fn presenting(mut self, presented: Presented) -> Self {
        self.presented = Some(presented);
        self
    }

    #[must_use]
    pub fn with_property(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.transport_properties.push((name.into(), value.into()));
        self
    }
}

impl fmt::Debug for ReceivedStream {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ReceivedStream")
            .field("arriving", &self.arriving)
            .field("source_uri", &self.source_uri)
            .field("presented", &self.presented)
            .field("transport_properties", &self.transport_properties)
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use xcore::mechanism;

    #[test]
    fn the_default_policy_never_compares_the_two_layers() {
        // The relaying case. One authenticated connection carrying traffic for
        // many Parties is ordinary, not a fault.
        let policy = IdentityPolicy::default();

        assert_eq!(policy.alignment, Alignment::None);
        assert_eq!(policy.on_misalignment, OnMisalignment::Accept);
    }

    #[test]
    fn what_arrives_is_not_yet_authenticated() {
        let received =
            ReceivedStream::new(&b"<order/>"[..], "https://xmip.example/in/party-x").presenting(
                Presented::passed(mechanism::mutual_tls(), "CN=party-x.example"),
            );

        // A credential was observed. Whether it holds is the gate's question,
        // and there is nowhere here to record an answer to it.
        assert!(received.presented.is_some());
    }

    #[test]
    fn a_technology_with_nothing_to_observe_presents_nothing() {
        // Modbus, CAN bus, a raw TCP socket. The circumstance becomes the
        // identity later; the transport itself saw no credential.
        let received = ReceivedStream::new(&b"\x01\x02"[..], "tcp://10.0.0.4:502");

        assert!(received.presented.is_none());
        assert_eq!(received.arriving, Arriving::Pushed);
    }
}
