use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum PendingDTO {
    #[default]
    Nobody,
    User,
    Opponent,
}

#[cfg(feature = "server")]
mod server_only {
    use super::*;
    use server::domain::{Pending, Player};

    impl PendingDTO {
        pub fn new(player: Player, pending: &Pending) -> Self {
            let opponent = player.opponent();
            match (pending.waiting_on(player), pending.waiting_on(opponent)) {
                (true, _) => PendingDTO::User,
                (false, true) => PendingDTO::Opponent,
                _ => PendingDTO::Nobody,
            }
        }
    }
}
