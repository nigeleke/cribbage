use crate::{Card, Crib, Finished, Hands, PlayState, Roles};

impl Finished {
    pub(crate) fn new(roles: Roles, hands: Hands, crib: Crib, starter: Card) -> Self {
        Self {
            roles,
            hands,
            crib,
            starter,
            play_state: None,
        }
    }

    pub(crate) fn with_play_state(mut self, play_state: PlayState) -> Self {
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

#[cfg(test)]
#[coverage(off)]
mod tests {
    use crate::tests::GameFixture;

    #[test]
    fn has_debug_text() {
        let given = format!("{:?}", GameFixture::default().as_finished());
        insta::assert_snapshot!(given, @r"
        game(finished(
          roles(dealer(player-0), pone(player-1))
          [Hand([]), Hand([])]
          Crib([])
          AS
          None
        )

          score(player-0: 0->0 player-1: 0->0) <- [])
        ");
    }
}
