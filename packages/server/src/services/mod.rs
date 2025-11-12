mod error;

pub mod action;
pub mod stream;
pub mod view;

pub use error::ServiceError;

type AggregateId = String;
