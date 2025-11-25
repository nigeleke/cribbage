use crate::domain::Game;
use crate::projections::GameView;
use cqrs_es::persist::GenericQuery;
use postgres_es::PostgresViewRepository;

pub type GameQuery = GenericQuery<PostgresViewRepository<GameView, Game>, GameView, Game>;
