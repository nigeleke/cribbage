use crate::card::Card;
use crate::cards::Crib;
use crate::game::{Hands, Playing};
use crate::players::Roles;
use crate::plays::PlayState;

impl Playing {
    pub(crate) fn new(roles: Roles, hands: Hands, crib: Crib, starter: Card) -> Self {
        let play_state = PlayState::new(roles.pone().player(), &hands);

        Self {
            roles,
            hands,
            crib,
            starter,
            play_state,
        }
    }
}

impl std::fmt::Debug for Playing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            r#"playing(
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
    use crate::game::tests::GameFixture;

    #[test]
    fn has_debug_text() {
        let given = format!("{:?}", GameFixture::default().as_playing());
        insta::assert_snapshot!(given, @r"
        game(playing(
          roles(dealer(player-0), pone(player-1))
          [Hand([]), Hand([])]
          Crib([])
          AS
          play_state: (next: player-1, go: not-called, current: [], previous: [])
        )

          score(player-0: 0->0 player-1: 0->0) <- [])
        ");
    }
}
