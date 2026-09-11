//! Protocol-neutral contracts shared by ingress adapters and request services.

mod types;

pub(crate) use types::{
    NormalizedRequestMetadata, ResolvedRouteTarget, SharedRequestContext,
};
