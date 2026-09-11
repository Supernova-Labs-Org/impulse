use std::net::SocketAddr;

use impulse_errors::ProxyError;

use super::resolve::ForwardTargetResolution;
use crate::{
    quic_listener::{
        LbHeaderLookup,
        admission::{AdmissionPolicyDecision, RequestAdmissionService},
    },
    request_pipeline::{RequestPolicyService, ResolvedRequestPolicy},
    resilience::runtime::RuntimeResilience,
};

/// Shared route-policy-admission pipeline used by protocol ingress adapters.
///
/// Protocol adapters supply their route/selection result and retain ownership
/// of parsing, response writing, and connection lifecycle state.
#[derive(Clone, Copy)]
pub(super) struct ForwardingRequestPipeline<'a> {
    resilience: &'a RuntimeResilience,
}

pub(super) struct PipelineRequest<'a> {
    pub(super) method: &'a str,
    pub(super) path: &'a str,
    pub(super) authority: Option<&'a str>,
    pub(super) peer_address: SocketAddr,
    pub(super) header_lookup: Option<&'a LbHeaderLookup<'a>>,
}

pub(super) struct PipelineResolution {
    pub(super) target: ForwardTargetResolution,
    pub(super) policy: ResolvedRequestPolicy,
    pub(super) admission: AdmissionPolicyDecision,
}

impl<'a> ForwardingRequestPipeline<'a> {
    pub(super) const fn new(resilience: &'a RuntimeResilience) -> Self {
        Self { resilience }
    }

    pub(super) fn resolve(
        self,
        target: Result<ForwardTargetResolution, ProxyError>,
        request: PipelineRequest<'_>,
    ) -> Result<PipelineResolution, ProxyError> {
        let target = target?;
        let policy = RequestPolicyService::resolve(&target.upstream_policy);
        let admission = RequestAdmissionService::new(self.resilience).evaluate_pre_auth(
            &policy.local_auth_policy,
            request.header_lookup,
            &target.upstream_name,
            request.method,
            request.path,
            request.authority,
            request.peer_address,
        );

        Ok(PipelineResolution {
            target,
            policy,
            admission,
        })
    }
}
