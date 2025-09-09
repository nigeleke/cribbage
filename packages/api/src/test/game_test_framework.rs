use crate::{
    AdvanceScoring, CheckForWinner, CutStarterCardAfterDiscards, Event, Game, GameId,
    RedrawOrStartGame,
};
use eventsourced::Command;
use eventsourced_ext::{TestFramework, TestFrameworkResult};

pub struct GameTestFramework {
    inner: TestFramework<Game>,
}

impl GameTestFramework {
    pub fn new(id: GameId, entity: Game) -> Self {
        let inner = TestFramework::new(id, entity).with_reactors(vec![
            Box::new(RedrawOrStartGame),
            Box::new(CutStarterCardAfterDiscards),
            Box::new(AdvanceScoring),
            Box::new(CheckForWinner),
        ]);
        Self { inner }
    }

    pub fn assert_entity(mut self, f: impl Fn(&Game)) -> Self {
        self.inner = self.inner.assert_entity(f);
        self
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
