//! Protocol-neutral contracts shared by ingress adapters and request services.

mod outcome;
mod policy;
mod routing;
mod types;

pub(crate) use outcome::{
    AdmissionOutcome, AdmissionRejectionReason, ForwardingFailureReason, ForwardingOutcome,
    ResponseAbortReason, ResponseOutcome,
};
pub(crate) use policy::{ExternalAuthPlan, RequestPolicyService, ResolvedRequestPolicy};
pub(crate) use routing::{RouteResolutionError, RouteResolutionService};
pub(crate) use types::{
    NormalizedRequestMetadata, ResolvedRouteTarget, SharedRequestContext,
};
