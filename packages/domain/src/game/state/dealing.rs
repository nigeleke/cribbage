use crate::game::Dealing;
use crate::players::Roles;

impl From<Roles> for Dealing {
    fn from(roles: Roles) -> Self {
        Self { roles }
    }
}

impl std::fmt::Debug for Dealing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "dealing(
  {:?}
)",
            self.roles
        )
    }
}

#[cfg(test)]
#[coverage(off)]
mod tests {
    use crate::game::tests::GameFixture;

    #[test]
    fn has_debug_text() {
        let given = format!("{:?}", GameFixture::default().as_dealing());
        insta::assert_snapshot!(given, @r"
        game(dealing(
          roles(dealer(player-0), pone(player-1))
        )

          score(player-0: 0->0 player-1: 0->0) <- [])
        ");
    }
}
