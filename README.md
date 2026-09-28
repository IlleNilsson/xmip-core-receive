# xmip-core-receive

Receive Ports, a Receive Location's identity policy, and what arrives at a
Receive Location before any gate has run: the `ReceivePort` that binds
arrivals into the topology, the `IdentityPolicy` a Receive Location keeps
between the two identity layers (ADR-0019 clause 7), and the
`ReceivedStream` — the Stream off a transport, how it got there and what the
transport observed.

A Receive Location is configured once, as `xmip-core-configure`'s
`ConfiguredLocation`, with the closed set of mechanisms it accepts
(`accept`, ADR-0019 clause 1); the runtime builds its transport from that
through `xmip-core-transport`'s one trait, which every protocol implements
in both directions (ADR-0010).

It does not create the Message — the runtime's arrival does, once the
transport gates have passed — and it does not send.

`doc/architecture/runtime-model.md` sections 5 and 6 govern the receive
chain, ADR-0010 the boundary with transport; `architecture.toml` carries the
maturity.
