use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, RwLock},
    time::Duration,
};

use impulse_config::runtime::RuntimeUpstreamPolicy;
use impulse_lb::upstream_pool::UpstreamPool;
use tokio::sync::Semaphore;

use super::{
    AdmissionPolicyDecision, LbHeaderLookup, PendingForward, PostAuthAdmissionExecution,
    RuntimeResilience, evaluate_forwarding_pre_admission_policy,
    execute_forwarding_post_auth_admission,
};

/// Protocol-neutral admission boundary for an already resolved request.
///
/// It owns the ordering of pre-auth and post-auth admission checks while the
/// protocol adapter retains metrics observation and response serialization.
#[derive(Clone, Copy)]
pub(super) struct RequestAdmissionService<'a> {
    resilience: &'a RuntimeResilience,
}

impl<'a> RequestAdmissionService<'a> {
    pub(super) const fn new(resilience: &'a RuntimeResilience) -> Self {
        Self { resilience }
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) fn evaluate_pre_auth(
        self,
        policy: &RuntimeUpstreamPolicy,
        header_lookup: Option<&LbHeaderLookup<'_>>,
        route: &str,
        method: &str,
        path: &str,
        authority: Option<&str>,
        client_addr: SocketAddr,
    ) -> AdmissionPolicyDecision {
        evaluate_forwarding_pre_admission_policy(
            policy,
            header_lookup,
            &self.resilience.brownout,
            self.resilience.adaptive_admission.inflight_percent(),
            route,
            method,
            path,
            authority,
            client_addr,
            self.resilience.shed_retry_after_seconds,
            &self.resilience.scoped_rate_limits,
        )
    }

    pub(super) fn execute_post_auth(
        self,
        pending_forward: &PendingForward,
        upstream_pool: Option<&Arc<RwLock<UpstreamPool>>>,
        backend_index: Option<usize>,
        upstream_inflight: &HashMap<String, Arc<Semaphore>>,
        global_inflight: Arc<Semaphore>,
        inflight_acquire_wait: Duration,
    ) -> PostAuthAdmissionExecution {
        execute_forwarding_post_auth_admission(
            self.resilience,
            pending_forward,
            upstream_pool,
            backend_index,
            upstream_inflight,
            global_inflight,
            inflight_acquire_wait,
        )
    }
}
