//! Protocol-neutral contracts shared by ingress adapters and request services.

mod outcome;
mod types;

pub(crate) use outcome::{
    AdmissionOutcome, AdmissionRejectionReason, ForwardingFailureReason, ForwardingOutcome,
    ResponseAbortReason, ResponseOutcome,
};
pub(crate) use types::{
    NormalizedRequestMetadata, ResolvedRouteTarget, SharedRequestContext,
};
