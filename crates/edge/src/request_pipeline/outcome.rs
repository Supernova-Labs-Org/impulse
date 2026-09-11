use http::StatusCode;

use crate::AdmissionOverloadCause;

/// Protocol-neutral result of request admission.
///
/// Adapters translate a rejection into their existing HTTP or H3 response only
/// after shared admission has completed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum AdmissionOutcome {
    Admitted,
    Rejected {
        reason: AdmissionRejectionReason,
        overload_cause: Option<AdmissionOverloadCause>,
    },
}

/// Semantic reason a request was not admitted.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum AdmissionRejectionReason {
    AuthDenied,
    AuthUnavailable,
    RateLimited,
    QuotaDenied,
    Overloaded,
    ValidationRejected,
    PolicyRejected,
    RequestBodyNotAllowed,
    RequestBodyTooLarge,
}

/// Protocol-neutral result of forwarding an admitted request.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ForwardingOutcome {
    Completed { status: StatusCode },
    Failed { reason: ForwardingFailureReason },
}

/// Semantic class of a failed upstream forwarding attempt.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ForwardingFailureReason {
    DispatchUnavailable,
    UpstreamResultDropped,
    Timeout,
    Transport,
    Protocol,
    Tls,
    Bridge,
}

/// Protocol-neutral state of the downstream response lifecycle.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ResponseOutcome {
    Pending,
    Completed { status: StatusCode },
    Aborted { reason: ResponseAbortReason },
}

/// Semantic reason a downstream response could not complete.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ResponseAbortReason {
    ClientDisconnected,
    WriteFailed,
    StreamAborted,
    TimedOut,
}
