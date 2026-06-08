//! PhyCloud adapter for [`UdmEventStore`].
//!
//! **Status — stub crate (v0.0.3).** The trait is implemented but every
//! method returns [`Error::Unsupported`] pending the HTTP client
//! implementation. The intent is that the wire-level integration
//! lands once the OSS schemas + analysis layer are merged into the
//! PhyWare monorepo (tracked as PhyWare#307 for the Rust SDK and
//! PhyWare#308 for the Python SDK).
//!
//! # Why ship a stub?
//!
//! 1. **Locks in the dependency graph.** The CLI (#301) and MCP server
//!    (#302) can wire feature flags against a real crate name instead
//!    of a `phycloud` module that may or may not exist.
//! 2. **Forces the public surface.** Anyone designing the PhyCloud
//!    HTTP API has a concrete trait to implement; nothing is left
//!    ambiguous about pagination, time-range semantics, or
//!    capabilities negotiation.
//! 3. **Documents the contract.** This file is the human-readable
//!    record of "what the adapter must do" until the full
//!    implementation lands.
//!
//! # Forward plan
//!
//! - PhyWare#307 — `phytrace-rust-sdk` consumes the OSS schemas + this
//!   trait; full HTTP client implementation lives here.
//! - PhyWare#308 — `phytrace-python-sdk` mirrors the Rust surface.
//! - The conformance suite in
//!   [`udm_eventstore::conformance::run_full_suite`] is the
//!   ground-truth acceptance test once the HTTP client is wired up.

#![deny(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]

use async_trait::async_trait;

use udm_eventstore::{
    AggregateQuery, AggregateResult, Error, EventId, EventPage, EventQuery, EventStream,
    GetEventOptions, Result, SourceId, StoreCapabilities, TimeRange, UdmEvent, UdmEventStore,
};

/// Connection configuration for the PhyCloud adapter.
///
/// All fields are placeholders pending the full HTTP client
/// implementation. The shape is intended to match a typical
/// `phycloud://` connection-string parse so it can be deserialised
/// from CLI / MCP configuration without further migration.
#[derive(Debug, Clone)]
pub struct PhyCloudConfig {
    /// Base URL of the PhyCloud HTTP API (e.g.
    /// `https://api.phycloud.example.com`).
    pub endpoint: String,
    /// API key or bearer token. Required for any tenant-scoped query.
    pub api_key: String,
    /// Optional tenant override for multi-tenant deployments.
    pub tenant_id: Option<String>,
    /// Request timeout in seconds (default 30).
    pub timeout_secs: u64,
}

impl PhyCloudConfig {
    /// Build a config from explicit endpoint + api_key. Tenant defaults
    /// to "from token"; timeout defaults to 30 s.
    pub fn new(endpoint: impl Into<String>, api_key: impl Into<String>) -> Self {
        Self {
            endpoint: endpoint.into(),
            api_key: api_key.into(),
            tenant_id: None,
            timeout_secs: 30,
        }
    }
}

/// PhyCloud-backed implementation of [`UdmEventStore`].
///
/// **Stub.** Every method returns [`Error::Unsupported`] until the HTTP
/// client lands. The capabilities struct accurately advertises the
/// adapter as not yet implemented so the CLI / MCP server can refuse
/// to start with a clear error message rather than appearing to work
/// and then failing on every query.
pub struct PhyCloudStore {
    config: PhyCloudConfig,
    capabilities: StoreCapabilities,
}

impl PhyCloudStore {
    /// Construct a PhyCloud store from the given config.
    ///
    /// Does no network I/O — the connection is opened lazily on the
    /// first request. In the stub, no request will ever succeed.
    pub fn new(config: PhyCloudConfig) -> Self {
        let capabilities = StoreCapabilities {
            backend: "phycloud".to_owned(),
            version: format!("{}-stub", env!("CARGO_PKG_VERSION")),
            // The eventual implementation supports all of these
            // natively; the stub reports them as false so consumers
            // know not to expect anything yet.
            supports_aggregation: false,
            supports_full_text: false,
            supports_ordered_streaming: false,
            max_page_size: 1000,
            indexed_paths: vec![
                "event_id".to_owned(),
                "event_type".to_owned(),
                "source_id".to_owned(),
                "source_type".to_owned(),
                "captured_at".to_owned(),
            ],
            cursor_scheme: "opaque".to_owned(),
        };
        Self {
            config,
            capabilities,
        }
    }

    /// Borrow the configured endpoint + tenant for diagnostics.
    pub fn config(&self) -> &PhyCloudConfig {
        &self.config
    }
}

fn unimpl(method: &str) -> Error {
    Error::Unsupported(format!(
        "phycloud adapter is a v0.0.3 stub; {method} not yet implemented. \
         Track PhyWare#307 / PhyWare#308 for the full HTTP client."
    ))
}

#[async_trait]
impl UdmEventStore for PhyCloudStore {
    async fn get_event(&self, _id: &EventId, _opts: &GetEventOptions) -> Result<Option<UdmEvent>> {
        Err(unimpl("get_event"))
    }

    async fn query_events(&self, _q: &EventQuery) -> Result<EventPage> {
        Err(unimpl("query_events"))
    }

    async fn timeline(&self, _source_id: &SourceId, _range: TimeRange) -> Result<EventStream<'_>> {
        Err(unimpl("timeline"))
    }

    async fn aggregate(&self, _agg: &AggregateQuery) -> Result<AggregateResult> {
        Err(unimpl("aggregate"))
    }

    fn capabilities(&self) -> &StoreCapabilities {
        &self.capabilities
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn stub_reports_not_implemented() {
        let store = PhyCloudStore::new(PhyCloudConfig::new("https://example", "k"));
        let id = "any".to_string();
        let err = store
            .get_event(&id, &GetEventOptions::default())
            .await
            .unwrap_err();
        assert!(matches!(err, Error::Unsupported(_)));
        assert_eq!(store.capabilities().backend, "phycloud");
        assert!(store.capabilities().version.ends_with("-stub"));
    }
}
