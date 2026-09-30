use cribbage_derive_id::Id;

#[derive(Clone, Copy, PartialEq, Eq, Id)]
pub struct UserId(uuid::Uuid);
