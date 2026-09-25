use crate::{Card, Crib, Hands, PlayState, Roles};

use crate::game::Finished;

impl Finished {
    pub fn new(roles: Roles, hands: Hands, crib: Crib, starter: Card) -> Self {
        Self {
            roles,
            hands,
            crib,
            starter,
            play_state: None,
        }
    }

    pub fn with_play_state(mut self, play_state: PlayState) -> Self {
        self.play_state = Some(play_state);
        self
    }
}

impl std::fmt::Debug for Finished {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            r#"finished(
  {:?}
  {:?}
  {:?}
  {:?}
  {:?}
)"#,
            self.roles, self.hands, self.crib, self.starter, self.play_state
        )
    }
}
