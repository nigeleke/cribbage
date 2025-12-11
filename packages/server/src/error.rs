use anyhow::Error as AnyhowError;
use thiserror::*;

/// Represents possible errors that can occur in the server.
#[derive(Debug, Error)]
pub enum ServerError {
    /// The user's request is not permitted.
    #[error("forbidden request: {0}")]
    Forbidden(String),

    /// The resource, e.g. game, cannot be found.
    #[error("not found")]
    NotFound,

    /// An error occurred within the domain usage.
    #[error(transparent)]
    Domain(#[from] crate::domain::DomainError),

    /// An unexpected error occurred; this wraps infrastrucutre errors as
    /// well as internal defects.
    #[error("internal server error: {0}")]
    Internal(
        #[from]
        #[source]
        AnyhowError,
    ),
}

macro_rules! bug_inner {
    ($msg:expr) => {{
        let location = std::panic::Location::caller();
        let message = format!("BUG at {}:{}:{} - {}",
            location.file(), location.line(), location.column(), $msg);

        #[cfg(feature = "backtrace")]
        {
            let backtrace = std::backtrace::Backtrace::capture();
            if backtrace.status() == std::backtrace::BacktraceStatus::Captured {
                use std::fmt::Write;
                let _ = writeln!(message.clone(), "\n\nBacktrace:\n{backtrace:?}");
            }
        }

        ServerError::Internal(anyhow::anyhow!(message))
    }};
    ($fmt:expr, $($arg:tt)*) => { bug_inner!(format_args!($fmt, $($arg)*)) };
}

macro_rules! bug {
    () => {
        |error| $crate::error::bug_inner!(error)
    };

    ($($msg:tt)*) => {{
        |e| $crate::error::bug_inner!(format_args!("{}: {}", format_args!($($msg)*), e))
    }};
}

pub(crate) use bug;
pub(crate) use bug_inner;
