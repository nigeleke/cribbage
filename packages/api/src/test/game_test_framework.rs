use crate::{
    CardCutForDealReactor, CardsDiscardedToCribReactor, Event, Game, GameId, ScoringReactor,
};
use eventsourced::Command;
use eventsourced_ext::{TestFramework, TestFrameworkResult};

pub struct GameTestFramework {
    inner: TestFramework<Game>,
}

impl GameTestFramework {
    pub fn new(id: GameId, entity: Game) -> Self {
        let inner = TestFramework::new(id, entity).with_reactors(vec![
            Box::new(CardCutForDealReactor),
            Box::new(CardsDiscardedToCribReactor),
            Box::new(ScoringReactor),
        ]);
        Self { inner }
    }

    pub fn entity(&self) -> &Game {
        self.inner.entity()
    }

    pub fn given(mut self, events: Vec<Event>) -> Self {
        self.inner = self.inner.given(events);
        self
    }

    pub fn when<R, ER>(
        self,
        command: impl Command<Game, Reply = R, Error = ER>,
    ) -> TestFrameworkResult<Game, R, ER>
    where
        R: PartialEq + std::fmt::Debug,
        ER: PartialEq + std::fmt::Debug,
    {
        self.inner.when(command)
    }
}
