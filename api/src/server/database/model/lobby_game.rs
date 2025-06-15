use chrono::{DateTime, Utc};
use goofy_animals::GoofyAnimals;
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::sync::LazyLock;
use uuid::Uuid;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct LobbyGameRow {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
}

impl LobbyGameRow {
    pub fn new(owner_id: Uuid) -> Self {
        let id = Uuid::new_v4();
        let name = generate_name();
        let created_at = Utc::now();
        Self {
            id,
            owner_id,
            name,
            created_at,
        }
    }
}

pub fn generate_name() -> String {
    let mut rng = ChaCha20Rng::from_entropy();
    GOOFY_ANIMALS.generate_name(&mut rng)
}

static ANIMALS_TXT: &str = include_str!("../../../data/animals.txt");
static ADJECTIVES_TXT: &str = include_str!("../../../data/adjectives.txt");

static ANIMALS: LazyLock<Vec<&'static str>> = LazyLock::new(|| ANIMALS_TXT.lines().collect());

static ADJECTIVES: LazyLock<Vec<&'static str>> = LazyLock::new(|| ADJECTIVES_TXT.lines().collect());

static GOOFY_ANIMALS: LazyLock<GoofyAnimals> =
    LazyLock::new(|| GoofyAnimals::new(ANIMALS.as_slice(), ADJECTIVES.as_slice()));

#[cfg(feature = "server")]
impl From<LobbyGameRow> for LobbyGame {
    fn from(value: LobbyGameRow) -> Self {
        Self {
            id: GameId::from(value.id),
            name: value.name,
        }
    }
}
