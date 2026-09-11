use std::sync::Arc;

use crate::{request_pipeline::ResolvedRouteTarget, routing::RouteIndex};

/// Shared route-index lookup service for protocol adapters.
#[derive(Clone, Copy)]
pub(crate) struct RouteResolutionService<'a> {
    routing_index: &'a RouteIndex,
}

impl<'a> RouteResolutionService<'a> {
    pub(crate) const fn new(routing_index: &'a RouteIndex) -> Self {
        Self { routing_index }
    }

    /// Resolves a validated HTTP method, path, and authority to an upstream route.
    pub(crate) fn resolve(
        self,
        method: &str,
        path: &str,
        authority: Option<&str>,
    ) -> Result<ResolvedRouteTarget, RouteResolutionError> {
        if method.is_empty() || path.is_empty() {
            return Err(RouteResolutionError::EmptyMethodOrPath);
        }

        let route = self
            .routing_index
            .lookup_with_decision_for_method(path, authority, Some(method))
            .ok_or(RouteResolutionError::NoRoute)?;

        Ok(ResolvedRouteTarget {
            upstream_name: Arc::from(route.upstream),
            matched_path_len: route.matched_path_len,
            host_specific: route.host_specific,
            method_specific: route.method_specific,
            reason: route.reason,
        })
    }
}

/// Route-lookup failures that ingress adapters map to their existing responses.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum RouteResolutionError {
    EmptyMethodOrPath,
    NoRoute,
}
