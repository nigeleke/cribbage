use crate::domain::{
    Discarding, Finished, HasCrib, HasHands, HasRoles, HasScoreboard, HasStarterCut, Phase,
    Playing, ScoringCrib, ScoringDealer, ScoringPone, Starting,
};

pub trait Wrap {
    fn wrap(self) -> Phase;
}

macro_rules! impl_wrap {
    ($($ty:ident => $variant:ident),* $(,)?) => {
        $(
            impl Wrap for $ty {
                fn wrap(self) -> Phase {
                    Phase::$variant(self)
                }
            }
        )*
    };
}

impl_wrap! {
    Starting      => Starting,
    Discarding    => Discarding,
    Playing       => Playing,
    ScoringPone   => ScoringPone,
    ScoringDealer => ScoringDealer,
    ScoringCrib   => ScoringCrib,
    Finished      => Finished,
}

pub trait WrapOrFinished {
    fn wrap_or_finished(self) -> Phase;
}

impl<T> WrapOrFinished for T
where
    T: HasScoreboard + HasRoles + HasHands + HasCrib + HasStarterCut + Wrap,
{
    fn wrap_or_finished(self) -> Phase {
        if let Some(winner) = self.scoreboard().winner() {
            let finished = Finished::new(
                winner,
                self.scoreboard().clone(),
                *self.roles(),
                self.hands().clone(),
                self.crib().clone(),
                *self.starter_cut(),
            );
            finished.wrap()
        } else {
            self.wrap()
        }
    }
}
