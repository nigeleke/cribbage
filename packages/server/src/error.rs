use anyhow::Error as AnyhowError;
use thiserror::*;

#[derive(Debug, Error)]
pub enum ServerError {
    #[error("forbidden request: {0}")]
    Forbidden(String),

    #[error("not found")]
    NotFound,

    #[error(transparent)]
    Domain(#[from] crate::domain::DomainError),

    #[error("internal server error: {0}")]
    Internal(
        #[from]
        #[source]
        AnyhowError,
    ),
}

#[macro_export]
macro_rules! bug_inner {
    ($msg:expr) => {{
        let location = std::panic::Location::caller();
        let mut msg = format!("BUG at {}:{}:{} — {}",
            location.file(), location.line(), location.column(), $msg);

        #[cfg(feature = "backtrace")]
        {
            let bt = std::backtrace::Backtrace::capture();
            if bt.status() == std::backtrace::BacktraceStatus::Captured {
                use std::fmt::Write;
                let _ = writeln!(msg, "\n\nBacktrace:\n{bt:?}");
            }
        }

        ServerError::Internal(anyhow::anyhow!(msg))
    }};
    ($fmt:expr, $($arg:tt)*) => { bug_inner!(format_args!($fmt, $($arg)*)) };
}

#[macro_export]
macro_rules! bug {
    // Case 1: bug!(original_error) → just lift the error with location
    () => {
        |error| $crate::bug_inner!(error)
    };

    // Case 2: bug!("custom message {}", vars...) → add context first
    ($($msg:tt)*) => {{
        |e| $crate::bug_inner!(format_args!("{}: {}", format_args!($($msg)*), e))
    }};
}

impl ServerError {
    #[track_caller]
    pub fn bug(msg: impl std::fmt::Display) -> Self {
        let location = std::panic::Location::caller();
        ServerError::Internal(anyhow::anyhow!(
            "BUG at {} {} {}: {msg}",
            location.file(),
            location.line(),
            location.column()
        ))
    }

    //     #[track_caller]
    //     pub fn bug_fmt(msg: impl std::fmt::Display, args: std::fmt::Arguments<'_>) -> Self {
    //         let location = std::panic::Location::caller();
    //         ServerError::Internal(anyhow::anyhow!(
    //             "BUG at {} {} {}: {msg}{args}",
    //             location.file(),
    //             location.line(),
    //             location.column()
    //         ))
    //     }
}
