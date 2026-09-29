mod call;
mod event;
mod pegs;
mod points;

pub(crate) use call::*;
pub(crate) use event::*;
pub(crate) use pegs::*;
pub(crate) use points::*;

// ------------------------------------
use crate::Player;
use crate::constants::WINNING_SCORE;

/// The scoreboard for a game, retaining its complete scoring history.
///
/// The current score and peg positions are derived from the recorded
/// scoring history rather than stored separately.
#[derive(Clone, Default, PartialEq, Eq)]
pub struct Scoreboard {
    history: Vec<Event>,
}

impl Scoreboard {
    /// Records a scoring event for a player.
    ///
    /// The event records the scoring phase and the individual calls that
    /// account for the points awarded. If no event actually occurred, no
    /// record is made; i.e. scoring zero points is not recorded.
    pub fn record(&mut self, maybe_event: Option<Event>) {
        if let Some(event) = maybe_event {
            self.history.push(event);
        }
    }

    /// Returns the current score for a player.
    pub fn points(&self, player: Player) -> Points {
        self.pegs(player).front_peg()
    }

    /// Returns the current positions of a player's two pegs.
    pub fn pegs(&self, player: Player) -> Pegs {
        self.history
            .iter()
            .filter(|e| e.player() == player)
            .fold(Pegs::default(), |mut ps, e| {
                ps.record(e.points());
                ps
            })
    }

    /// Returns the winning player, if any.
    /// If there is a winner, there will only be one, so returning the "first" here is okay.
    pub fn winner(&self) -> Option<Player> {
        use strum::IntoEnumIterator;
        Player::iter().find(|p| self.points(*p) >= Points::from(WINNING_SCORE))
    }

    /// Create a Scoreboard pending win for player.
    #[cfg(test)]
    pub fn at_120(player: Player) -> Self {
        use crate::{card, hand};

        let perfect_29 = Event::try_pone_hand(player, &hand!("JH5C5D5S"), card!("5H"));
        let starter = Event::try_starter(player, card!("JH"));

        let mut scoreboard = Scoreboard::default();
        scoreboard.record(perfect_29.clone()); // 29
        scoreboard.record(perfect_29.clone()); // 58
        scoreboard.record(perfect_29.clone()); // 87
        scoreboard.record(perfect_29); // 116
        scoreboard.record(starter.clone()); // 118
        scoreboard.record(starter); // 120

        scoreboard
    }

    /// Create a Scoreboard with preset pegging.
    #[cfg(test)]
    pub fn with_calls(mut self, player: Player, calls: &[Call]) -> Self {
        let event = Event::new(player, calls);
        self.history.push(event);
        self
    }
}

impl std::fmt::Debug for Scoreboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "score({:?}: {:?} {:?}: {:?}) <- {:#?}",
            Player::Player0,
            self.pegs(Player::Player0),
            Player::Player1,
            self.pegs(Player::Player1),
            self.history
        )
    }
}

#[cfg(test)]
mod tests;
