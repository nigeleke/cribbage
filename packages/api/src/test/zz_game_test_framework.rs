use crate::domain::{Game, GameId};

pub struct GameTestFramework;

impl GameTestFramework {
    pub fn new(id: GameId, game: Game) -> Self {
        let inner = TestFramework::new(id, game);
        Self { inner }
    }

    pub fn assert_entity(mut self, f: impl Fn(&Game)) -> Self {
        self.inner = self.inner.assert_entity(f);
        self
    }

    pub fn entity(&self) -> &Game {
        self.inner.entity()
    }

    pub fn execute_using_result<C>(mut self, command: C, f: impl Fn(&C::Reply)) -> Self
    where
        C: Command<Game>,
        C::Error: std::fmt::Debug,
    {
        let effect = command.handle_command(self.entity().id(), self.entity());
        match effect {
            CommandEffect::EmitAndReply(event, reply) => {
                let updated = self.given(vec![event]);
                let entity = updated.entity();
                let reply = reply(&entity);
                f(&reply);
                updated
            }
            CommandEffect::Reply(reply) => {
                f(&reply);
                self
            }
            CommandEffect::Reject(error) => panic!("failed to execute command - {error:?}"),
        }
    }

    pub fn execute<C>(self, command: C) -> Self
    where
        C: Command<Game>,
        C::Error: std::fmt::Debug,
    {
        self.execute_using_result(command, |_| {})
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
