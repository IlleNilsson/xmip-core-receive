# xmip-core-receive

Receive Ports, Receive Locations, and what arrives at them: the
`ReceiveLocation` with its transport, its address and its identity policy,
the `ReceivePort` that binds arrivals into the topology, and the
`ReceivedStream` a transport hands over.

Receive owns inbound orchestration and nothing about how bytes move: a
transport technology moves the Stream, this crate decides what is done with
it. It does not create the Message — a Contract accepts a Stream — and it
does not send. A Receive Location declares a closed set of mechanisms and
Parties it accepts; anything else is refused at authentication (ADR-0019).

`doc/architecture/runtime-model.md` sections 5 and 6 govern the receive
chain, ADR-0010 the boundary with transport; `architecture.toml` carries the
maturity.
