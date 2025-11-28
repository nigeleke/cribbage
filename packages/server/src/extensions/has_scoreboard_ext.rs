use crate::domain::{GameEvent, HasScoreboard, Player, Points, ScoreBreakdown};

pub trait HasScoreboardExt: HasScoreboard {
    fn award_points(&self, player: Player, reasons: ScoreBreakdown) -> Vec<GameEvent> {
        let mut events = Vec::default();

        if reasons.points() != Points::from(0) {
            let winner = self.scoreboard().clone().peg(player, &reasons);
            events.push(GameEvent::PointsScored { player, reasons });

            if let Some(player) = winner {
                events.push(GameEvent::WinnerDeclared { player });
            }
        }

        events
    }
}

impl<T: HasScoreboard> HasScoreboardExt for T {}
