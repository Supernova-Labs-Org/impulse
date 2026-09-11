use std::{net::SocketAddr, sync::Arc, time::Instant};

use http::{HeaderMap, Method};

/// HTTP request data normalized by an ingress adapter before policy evaluation.
///
/// The fields deliberately describe HTTP semantics only. Connection, stream,
/// QUIC, and Hyper state remain owned by their respective protocol adapters.
#[derive(Clone, Debug)]
pub(crate) struct NormalizedRequestMetadata {
    pub(crate) method: Method,
    pub(crate) scheme: Option<Arc<str>>,
    pub(crate) authority: Option<Arc<str>>,
    pub(crate) path: Arc<str>,
    pub(crate) headers: HeaderMap,
}

/// The route selected by the shared routing service before backend selection.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ResolvedRouteTarget {
    pub(crate) upstream_name: Arc<str>,
    pub(crate) matched_path_len: usize,
    pub(crate) host_specific: bool,
    pub(crate) method_specific: bool,
}

/// Request-scoped identity and timing data shared across request services.
///
/// Trace identifiers are propagated as opaque values. Protocol adapters remain
/// responsible for parsing protocol-specific tracing headers.
#[derive(Clone, Debug)]
pub(crate) struct SharedRequestContext {
    pub(crate) request_id: u64,
    pub(crate) peer_addr: SocketAddr,
    pub(crate) trace_id: Option<Arc<str>>,
    pub(crate) span_id: Option<Arc<str>>,
    pub(crate) traceparent: Option<Arc<str>>,
    pub(crate) started_at: Instant,
    pub(crate) deadline: Instant,
}
