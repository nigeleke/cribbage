use std::sync::LazyLock;

// use goofy_animals::GoofyAnimals;
// use rand::{Rng, SeedableRng};
// use rand_chacha::ChaCha20Rng;

pub fn generate_game_name() -> String {
    "x".into()
    // let mut rng = ChaCha20Rng::from_os_rng();
    // GOOFY_ANIMALS.generate_name(&mut rng)
}

static ANIMALS_TXT: &str = include_str!("../data/animals.txt");
static ADJECTIVES_TXT: &str = include_str!("../data/adjectives.txt");

static ANIMALS: LazyLock<Vec<&'static str>> = LazyLock::new(|| ANIMALS_TXT.lines().collect());

static ADJECTIVES: LazyLock<Vec<&'static str>> = LazyLock::new(|| ADJECTIVES_TXT.lines().collect());

// static GOOFY_ANIMALS: LazyLock<GoofyAnimals<'static>> =
//     LazyLock::new(|| GoofyAnimals::new(ANIMALS.as_slice(), ADJECTIVES.as_slice()));
