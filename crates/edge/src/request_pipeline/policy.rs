use impulse_config::{
    config::{ForwardedHeaderPolicy, UpstreamHostPolicy},
    runtime::{RuntimeExternalAuth, RuntimeUpstreamPolicy},
};

use crate::runtime::connection::auth::{ExternalAuthFailureDisposition, ExternalAuthTaskConfig};

/// Resolves the policies selected by route resolution independently of the
/// ingress protocol.
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct RequestPolicyService;

impl RequestPolicyService {
    pub(crate) fn resolve(upstream_policy: &RuntimeUpstreamPolicy) -> ResolvedRequestPolicy {
        ResolvedRequestPolicy {
            local_auth_policy: upstream_policy.clone(),
            host_policy: upstream_policy.host.0.clone(),
            forwarded_header_policy: upstream_policy.forwarded_headers.0.clone(),
            external_auth: upstream_policy
                .upstream_auth
                .external_auth
                .clone()
                .map(ExternalAuthPlan::new),
        }
    }
}

/// Protocol-neutral policies applied to a request after its route resolves.
#[derive(Clone, Debug)]
pub(crate) struct ResolvedRequestPolicy {
    /// Local API-key and JWT/JWKS policy evaluated before external auth.
    pub(crate) local_auth_policy: RuntimeUpstreamPolicy,
    pub(crate) host_policy: UpstreamHostPolicy,
    pub(crate) forwarded_header_policy: ForwardedHeaderPolicy,
    pub(crate) external_auth: Option<ExternalAuthPlan>,
}

/// External-auth work selected by the resolved request policy.
#[derive(Clone, Debug)]
pub(crate) struct ExternalAuthPlan {
    pub(crate) policy: RuntimeExternalAuth,
    pub(crate) disposition: ExternalAuthFailureDisposition,
}

impl ExternalAuthPlan {
    fn new(policy: RuntimeExternalAuth) -> Self {
        Self {
            disposition: ExternalAuthTaskConfig::from_external_auth(&policy).disposition,
            policy,
        }
    }
}
