use cqrs_es::persist::GenericQuery;
use postgres_es::PostgresViewRepository;

use crate::{domain::Game, projections::GameView};

pub type GameQuery = GenericQuery<PostgresViewRepository<GameView, Game>, GameView, Game>;
