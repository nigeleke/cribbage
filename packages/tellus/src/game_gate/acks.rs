use std::collections::HashSet;

use cribbage_domain::prelude::Player;
use tellus::{Effect, EventSourced};

use crate::{game::Users, user::UserId};

#[derive(Debug)]
pub struct Acks(HashSet<UserId>);

impl Acks {
    pub(crate) fn ack_effect<T: EventSourced>(&self, user: UserId, effect: Effect<T>) -> Effect<T> {
        if self.acked(user) {
            Effect::none()
        } else {
            effect
        }
    }

    pub(crate) fn ack(&mut self, user: UserId) {
        self.0.remove(&user);
    }

    pub(crate) fn acked(&self, user: UserId) -> bool {
        !self.0.contains(&user)
    }

    pub(crate) fn all_acked(&self) -> bool {
        self.0.is_empty()
    }
}

impl From<&Users> for Acks {
    fn from(value: &Users) -> Self {
        Self(HashSet::from_iter([
            value[Player::Player0],
            value[Player::Player1],
        ]))
    }
}
