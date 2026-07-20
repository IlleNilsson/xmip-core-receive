#![forbid(unsafe_code)]

use std::error::Error;
use std::fmt;
use xmip_core::{ArtifactId, PartyId};
use xmip_stream::Stream;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReceiveLocationType {
    Composite,
    DataTransfer,
    BatchLoad,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReceiveLocation {
    pub artifact_id: ArtifactId,
    pub name: String,
    pub uri: String,
    pub transport: String,
    pub location_type: ReceiveLocationType,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReceivePort {
    pub artifact_id: ArtifactId,
    pub name: String,
    pub version: String,
}

#[derive(Clone, Debug)]
pub struct ReceivedStream {
    pub stream: Stream,
    pub source_uri: String,
    pub identified_party: Option<PartyId>,
    pub transport_properties: Vec<(String, String)>,
}

#[derive(Debug)]
pub struct ReceiveError {
    pub message: String,
}

impl fmt::Display for ReceiveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { f.write_str(&self.message) }
}
impl Error for ReceiveError {}

pub trait ReceiveTransport: Send + Sync {
    fn technology(&self) -> &'static str;
    fn receive(&self, location: &ReceiveLocation) -> Result<Option<ReceivedStream>, ReceiveError>;
}

pub trait ReceivePublisher: Send + Sync {
    fn publish(&self, port: &ReceivePort, location: &ReceiveLocation, received: ReceivedStream) -> Result<(), ReceiveError>;
}
