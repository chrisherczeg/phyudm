# udm-eventstore-phycloud

[![Crate](https://img.shields.io/crates/v/udm-eventstore-phycloud.svg)](https://crates.io/crates/udm-eventstore-phycloud)
[![Docs](https://docs.rs/udm-eventstore-phycloud/badge.svg)](https://docs.rs/udm-eventstore-phycloud)

Reference deployment [`UdmEventStore`](../udm-eventstore/) adapter for
**PhyCloud** — the multi-tenant, Postgres-backed telemetry store that
ships as the sole commercially-supported deployment path for the OSS
UDM analysis tooling.

## Status — v0.0.3 STUB

This crate compiles, exposes the public surface (`PhyCloudConfig`,
`PhyCloudStore`), and implements the `UdmEventStore` trait — but
**every method currently returns
`Error::Unsupported`**. The full HTTP client implementation lands
once the OSS schemas + analysis layer are integrated into the
PhyWare monorepo:

- **PhyWare#307** — `phytrace-rust-sdk` consumes the OSS schemas
  and this trait; the HTTP client implementation lives here.
- **PhyWare#308** — `phytrace-python-sdk` mirrors the Rust surface.

Until then, downstream tooling (CLI #301, MCP server #302) that
selects the `phycloud` backend will surface a clear "not yet
implemented" error referencing those issues, instead of silently
failing on every query.

## Why ship a stub?

1. **Locks in the dependency graph.** The CLI and MCP server can wire
   feature flags + crate names against a real artifact rather than a
   forward reference that may or may not exist.
2. **Forces the public surface.** Anyone designing the PhyCloud HTTP
   API has a concrete trait to implement; nothing is ambiguous about
   pagination, time-range semantics, capability negotiation.
3. **Documents the contract** in source.

## License

Apache-2.0.
