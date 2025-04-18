use super::cards::{
    Card, Crib, Cut, Cuts, Deck, Hand, Hands, HasCrib, HasCut, HasCuts, HasDeck, HasHands,
};
use super::players::{HasPlayers, HasRoles, Player, Players, Roles, RolesError};
use super::plays::PlayState;
use super::scorers::{CribScorer, CurrentPlayScorer, CutScorer, EndOfPlayScorer, Scorer};
use super::scoring::{HasScores, Pegging, ScoreReasons, Scores};
use super::state::{
    Discarding, DiscardingState, Finished, Playing, ScoringCrib, ScoringDealer, ScoringPone,
    Starting,
};
use super::{HandScorer, HasPlayState, Peggings};

use crate::constants::*;

use serde::{Deserialize, Serialize};
use thiserror::*;

use std::collections::HashMap;

#[derive(Debug, Error, PartialEq)]
pub enum GameError {
    #[error("internal error")]
    InternalError(String),

    #[error("incorrect number of players: {0} given, 2 required")]
    IncorrectNumberOfPlayers(usize),

    #[error("player {0} not in game")]
    PlayerNotInGame(Player),

    #[error("cannot redraw for start of game as cut for dealer was decisive")]
    CutForStartDecided,

    #[error("error determining roles: {0}")]
    CannotDetermineRoles(#[from] RolesError),

    #[error("player {0} is not a participant of the current game")]
    InvalidPlayer(Player),

    #[error("player does not own card {0}")]
    InvalidCard(Card),

    #[error("player does not own all cards")]
    InvalidCards,

    #[error("not this player's turn to play")]
    CannotPlay,

    #[error("not this player's turn to pass")]
    CannotPass,

    //------------------
    #[error("an action was attempted which is not permitted in the current game state")]
    ActionNotPermitted,

    #[error("cannot play the desired card")]
    CannotPlayCard,

    #[error("cannot score pone as it is still possible to play cards")]
    CannotScorePone,

    #[error("only two cards can be discarded to the crib")]
    TooManyDiscards,
}

type Result<T> = std::result::Result<T, GameError>;

/// The game state, waiting for opponent, discarding, playing, scoring, finished.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Game<T> {
    state: T,
    _marker: std::marker::PhantomData<T>,
}

impl<T> Game<T> {
    pub fn new(state: T) -> Self {
        Self {
            state,
            _marker: std::marker::PhantomData::<_>,
        }
    }
}

impl<T: HasPlayers> HasPlayers for Game<T> {
    fn players(&self) -> Players {
        self.state.players()
    }
}

impl<T: HasDeck> HasDeck for Game<T> {
    fn deck(&self) -> &Deck {
        self.state.deck()
    }

    fn deck_mut(&mut self) -> &mut Deck {
        self.state.deck_mut()
    }
}

impl<T: HasCuts> HasCuts for Game<T> {
    fn cuts(&self) -> &Cuts {
        self.state.cuts()
    }
}

impl<T: HasRoles> HasRoles for Game<T> {
    fn roles(&self) -> &Roles {
        self.state.roles()
    }
}

impl<T: HasHands> HasHands for Game<T> {
    fn hands(&self) -> &Hands {
        self.state.hands()
    }

    fn hands_mut(&mut self) -> &mut Hands {
        self.state.hands_mut()
    }
}

impl<T: HasScores> HasScores for Game<T> {
    fn scores(&self) -> &Scores {
        self.state.scores()
    }

    fn scores_mut(&mut self) -> &mut Scores {
        self.state.scores_mut()
    }
}

impl<T: HasCrib> HasCrib for Game<T> {
    fn crib(&self) -> &Crib {
        self.state.crib()
    }

    fn crib_mut(&mut self) -> &mut Crib {
        self.state.crib_mut()
    }
}

impl<T: HasPlayState> HasPlayState for Game<T> {
    fn play_state(&self) -> &PlayState {
        self.state.play_state()
    }

    fn play_state_mut(&mut self) -> &mut PlayState {
        self.state.play_state_mut()
    }
}

impl<T: HasCut> HasCut for Game<T> {
    fn cut(&self) -> Cut {
        self.state.cut()
    }
}

impl<T: HasPlayers> Game<T> {
    fn validate_player(&self, player: Player) -> Result<()> {
        if self.players().contains(&player) {
            Ok(())
        } else {
            Err(GameError::InvalidPlayer(player))
        }
    }

    pub fn player_1_2(&self) -> Result<(Player, Player)> {
        let players = self.state.players();
        let players_count = players.len();
        (players_count == NUMBER_OF_PLAYERS_IN_GAME)
            .then_some(players.players_1_2())
            .ok_or(GameError::IncorrectNumberOfPlayers(players_count))
    }

    pub fn opponent(&self, player: Player) -> Result<Player> {
        self.validate_player(player)?;
        Ok(self.state.players().opponent(player))
    }
}

impl<T: HasCuts> Game<T> {
    pub fn cut(&self, player: Player) -> Result<Cut> {
        self.state
            .cuts()
            .get(&player)
            .copied()
            .ok_or(GameError::InvalidPlayer(player))
    }
}

impl<T: HasHands> Game<T> {
    fn validate_player_card(&self, player: Player, card: Card) -> Result<()> {
        if let Some(hand) = self.hands().get(&player) {
            if hand.contains(&card) {
                Ok(())
            } else {
                Err(GameError::InvalidCard(card))
            }
        } else {
            Err(GameError::InvalidPlayer(player))
        }
    }

    fn validate_player_cards(&self, player: Player, cards: &[Card]) -> Result<()> {
        if let Some(hand) = self.hands().get(&player) {
            if hand.contains_all(cards) {
                Ok(())
            } else {
                Err(GameError::InvalidCards)
            }
        } else {
            Err(GameError::InvalidPlayer(player))
        }
    }

    fn validate_player_discards(&self, player: Player, discards: &[Card]) -> Result<()> {
        self.validate_player_cards(player, discards)?;
        let hand = self.hand(player)?;
        if hand.len() - discards.len() >= CARDS_KEPT_PER_HAND {
            Ok(())
        } else {
            Err(GameError::TooManyDiscards)
        }
    }

    pub fn hand(&self, player: Player) -> Result<&Hand> {
        self.hands()
            .get(&player)
            .ok_or(GameError::InvalidPlayer(player))
    }

    pub fn hand_mut(&mut self, player: Player) -> Result<&mut Hand> {
        self.hands_mut()
            .get_mut(&player)
            .ok_or(GameError::InvalidPlayer(player))
    }
}

impl<T: HasScores> Game<T> {
    pub fn pegging(&self, player: Player) -> Result<&Pegging> {
        self.peggings()
            .get(&player)
            .ok_or(GameError::InvalidPlayer(player))
    }

    // pub fn score(&self, player: Player) -> Result<()> {
    //     self.pegging().
    // }
}

impl<T: HasPlayState> Game<T> {
    fn validate_next_to_play(&self, player: Player) -> Result<()> {
        if self.play_state().next_to_play() == player {
            Ok(())
        } else {
            Err(GameError::CannotPlay)
        }
    }

    fn validate_can_play(&self, player: Player, card: Card) -> Result<()> {
        self.validate_next_to_play(player)?;

        let legal_plays = self.play_state().legal_plays(player);

        if legal_plays.contains(&card) {
            Ok(())
        } else {
            Err(GameError::CannotPlayCard)
        }
    }

    fn validate_can_pass(&self, player: Player) -> Result<()> {
        self.validate_next_to_play(player)?;

        let legal_plays = self.play_state().legal_plays(player);

        if legal_plays.is_empty() {
            Ok(())
        } else {
            Err(GameError::CannotPass)
        }
    }
}

//     fn winner(&self) -> Option<Player> {
//         let peggings = self.peggings();
//         peggings
//             .iter()
//             .filter_map(|(player, pegging)| {
//                 (pegging.points() >= WINNING_SCORE.into()).then_some(*player)
//             })
//             .next()
//     }

//     pub fn redraw(&self) -> Result<Self> {
//         match self {
//             Game::Starting(cuts, _) => {
//                 let players = self.players();
//                 verify::players(&players)?;
//                 verify::same_cuts(cuts)?;
//                 Ok(Game::new(&self.players())?)
//             }
//             _ => Err(GameError::ActionNotPermitted),
//         }
//     }

//     pub fn dealer(&self) -> Player {
//         match self {
//             Game::Starting(_, _) => unreachable!(),
//             Game::Discarding(_, dealer, _, _, _) => *dealer,
//             Game::Playing(_, dealer, _, _, _, _) => *dealer,
//             Game::ScoringPone(_, dealer, _, _, _) => *dealer,
//             Game::ScoringDealer(_, dealer, _, _, _) => *dealer,
//             Game::ScoringCrib(_, dealer, _, _, _) => *dealer,
//             Game::Finished(_, _) => unreachable!(),
//         }
//     }

//     pub fn pone(&self) -> Player {
//         let (player1, player2) = self.player_1_2();
//         if self.dealer() == player1 {
//             player2
//         } else {
//             player1
//         }
//     }

//     fn score(&self, player: Player, reasons: &ScoreReasons) -> Result<Self> {
//         let update = |scores: &mut Scores| {
//             scores.add(player, reasons);
//         };

//         let mut game = self.clone();
//         match game {
//             Game::Starting(_, _) => unreachable!(),
//             Game::Discarding(ref mut scores, _, _, _, _) => update(scores),
//             Game::Playing(ref mut scores, _, _, _, _, _) => update(scores),
//             Game::ScoringPone(ref mut scores, _, _, _, _) => update(scores),
//             Game::ScoringDealer(ref mut scores, _, _, _, _) => update(scores),
//             Game::ScoringCrib(ref mut scores, _, _, _, _) => update(scores),
//             Game::Finished(_, _) => {}
//         };

//         if let Some(winner) = game.winner() {
//             let scores = game.peggings();
//             game = Game::Finished(winner, scores)
//         }

//         Ok(game)
//     }

//     pub .unwrap()&self, player: Player) -> Result<Game> {
//     }

//     pub fn score_pone(&self) -> Result<Game> {
//         let mut game = self.clone();
//         let pone = game.pone();

//         match game {
//             Game::Playing(ref mut scores, dealer, _, ref mut play_state, cut, crib) => {
//                 verify::ready_to_score_pone(play_state)?;

//                 let hands = play_state.finish_plays();
//                 game = Game::ScoringPone(scores.clone(), dealer, hands.clone(), cut, crib.clone());
//                 let score = HandScorer::new(&hands[&pone], cut).score();
//                 game.score(pone, &score)
//             }
//             _ => Err(GameError::ActionNotPermitted),
//         }
//     }

//     pub fn score_dealer(&self) -> Result<Game> {
//         let mut game = self.clone();
//         match game {
//             Game::ScoringPone(ref mut scores, dealer, hands, cut, crib) => {
//                 game =
//                     Game::ScoringDealer(scores.clone(), dealer, hands.clone(), cut, crib.clone());
//                 let score = HandScorer::new(&hands[&dealer], cut).score();
//                 game.score(dealer, &score)
//             }
//             _ => Err(GameError::ActionNotPermitted),
//         }
//     }

//     pub fn score_crib(&self) -> Result<Game> {
//         let mut game = self.clone();
//         match game {
//             Game::ScoringDealer(ref mut scores, dealer, hands, cut, crib) => {
//                 game = Game::ScoringCrib(scores.clone(), dealer, hands.clone(), cut, crib.clone());
//                 let score = CribScorer::new(&crib, cut).score();
//                 game.score(dealer, &score)
//             }
//             _ => Err(GameError::ActionNotPermitted),
//         }
//     }

//     pub fn deal_next_hands(&self) -> Result<Game> {
//         match self {
//             Game::ScoringCrib(scores, _, _, _, _) => {
//                 let players = self.players();
//                 let deck = Deck::shuffled_pack();
//                 let (hands, deck) = deck.deal(&players);
//                 let crib = Crib::default();
//                 Ok(Game::Discarding(
//                     scores.clone(),
//                     self.pone(),
//                     hands,
//                     crib,
//                     deck,
//                 ))
//             }
//             _ => Err(GameError::ActionNotPermitted),
//         }
//     }
// }

// impl std::fmt::Display for Game {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Game::Starting(cuts, deck) => write!(
//                 f,
//                 "Starting(Cuts({}), Deck({}))",
//                 format_hashmap(cuts),
//                 deck
//             ),

//             Game::Discarding(scores, dealer, hands, crib, deck) => write!(
//                 f,
//                 "Discarding(Scores({}), Dealer({}), Hands({}), Crib({}), Deck({}))",
//                 scores,
//                 dealer,
//                 format_hashmap(hands),
//                 crib,
//                 deck
//             ),

//             Game::Playing(scores, dealer, hands, play_state, cut, crib) => write!(
//                 f,
//                 "Playing(Scores({}), Dealer({}), Hands({}), PlayState({}), Cut({})), Crib({}))",
//                 scores,
//                 dealer,
//                 format_hashmap(hands),
//                 play_state,
//                 cut,
//                 crib
//             ),

//             Game::ScoringPone(scores, dealer, hands, cut, crib) => write!(
//                 f,
//                 "ScoringPone(Scores({}), Dealer({}), Hands({}), Cut({})), Crib({}))",
//                 scores,
//                 dealer,
//                 format_hashmap(hands),
//                 cut,
//                 crib
//             ),

//             Game::ScoringDealer(scores, dealer, hands, cut, crib) => write!(
//                 f,
//                 "ScoringDealer(Scores({}), Dealer({}), Hands({}), Cut({})), Crib({}))",
//                 scores,
//                 dealer,
//                 format_hashmap(hands),
//                 cut,
//                 crib
//             ),

//             Game::ScoringCrib(scores, dealer, hands, cut, crib) => write!(
//                 f,
//                 "ScoringCrib(Scores({}), Dealer({}), Hands({}), Cut({})), Crib({}))",
//                 scores,
//                 dealer,
//                 format_hashmap(hands),
//                 cut,
//                 crib
//             ),

//             Game::Finished(winner, peggings) => write!(
//                 f,
//                 "Finished(Winner({}), Peggings({}))",
//                 winner,
//                 format_hashmap(peggings)
//             ),
//         }
//     }
// }

// mod verify {
//     use super::*;

//     pub fn players(players: &Players) -> Result<()> {
//         if players.len() != NUMBER_OF_PLAYERS_IN_GAME {
//             Err(GameError::NotEnoughPlayers)
//         } else {
//             Ok(())
//         }
//     }

//     pub fn player(player: Player, players: &Players) -> Result<()> {
//         if !players.contains(&player) {
//             Err(GameError::InvalidPlayer(player))
//         } else {
//             Ok(())
//         }
//     }

//     pub fn different_cuts(cuts: &Cuts) -> Result<()> {
//         let cuts: HashSet<Rank> = HashSet::from_iter(cuts.values().map(|c| c.rank()));
//         if cuts.len() != NUMBER_OF_PLAYERS_IN_GAME {
//             Err(GameError::CutForStartUndecided)
//         } else {
//             Ok(())
//         }
//     }

//     pub fn same_cuts(cuts: &Cuts) -> Result<()> {
//         let cuts: HashSet<Rank> = HashSet::from_iter(cuts.values().map(|c| c.rank()));
//         if cuts.len() == NUMBER_OF_PLAYERS_IN_GAME {
//             Err(GameError::CutForStartDecided)
//         } else {
//             Ok(())
//         }
//     }

//     pub fn discards(discards: &[Card], hand: &Hand) -> Result<()> {
//         for discard in discards {
//             verify::card(*discard, hand.as_ref())?
//         }

//         if hand.len() - discards.len() < CARDS_KEPT_PER_HAND {
//             Err(GameError::TooManyDiscards)
//         } else {
//             Ok(())
//         }
//     }

//     pub fn card(card: Card, cards: &[Card]) -> Result<()> {
//         if !cards.contains(&card) {
//             Err(GameError::InvalidCard(card))
//         } else {
//             Ok(())
//         }.
//     }

//     pub fn no_legal_plays(cards: &[Card]) -> Result<()> {
//         if cards.is_empty() {
//             Ok(())
//         } else {
//             Err(GameError::CannotPass)
//         }
//     }

//     pub fn ready_to_score_pone(play_state: &PlayState) -> Result<()> {
//         if play_state.all_are_cards_played() {
//             Ok(())
//         } else {
//             Err(GameError::CannotScorePone)
//         }
//     }

// impl<T> From<T> for Game<T> {
//     fn from(value: T) -> Self {
//         Game {
//             state: value,
//             _marker: std::marker::PhantomData::<T>,
//         }
//     }
// }

impl Game<Starting> {
    pub fn try_new(value: &Players) -> Result<Game<Starting>> {
        if value.len() == NUMBER_OF_PLAYERS_IN_GAME {
            let mut deck = Deck::shuffled_pack();
            let make_cut = |mut cuts: Cuts, player: &Player| {
                let cut = deck.cut();
                cuts.insert(*player, cut);
                cuts
            };
            let cuts = value.iter().fold(HashMap::new(), make_cut);
            Ok(Game::<_>::new(Starting::new(cuts, deck)))
        } else {
            Err(GameError::IncorrectNumberOfPlayers(value.len()))
        }
    }

    pub fn start(self) -> Result<Game<Discarding>> {
        let initial_discarding = |roles: Roles| {
            let mut deck = Deck::shuffled_pack();
            let players = self.players();
            let scores = Scores::new(&players);
            let hands = deck.deal(&players);
            let crib = Crib::default();

            let discarding = Discarding::new(scores, roles, hands, crib, deck);

            Game::<_>::new(discarding)
        };

        Roles::try_from(self.cuts())
            .map(initial_discarding)
            .map_err(GameError::from)
    }

    pub fn redraw(self) -> Result<Game<Starting>> {
        match Roles::try_from(self.cuts()) {
            Ok(_) => Err(GameError::CutForStartDecided),
            Err(_) => Game::<_>::try_new(&self.players()),
        }
    }
}

#[derive(Debug)]
pub enum DiscardResult {
    Discarding(Game<Discarding>),
    Playing(Game<Playing>),
    Finished(Game<Finished>),
}

impl Game<Discarding> {
    pub fn discard(mut self, player: Player, discards: &[Card]) -> Result<DiscardResult> {
        self.validate_player(player)?;
        self.validate_player_discards(player, discards)?;

        let discard_state = self.state.discard(player, discards);
        let (mut scores, roles, hands, crib, mut deck) = self.state.into_parts();
        let result = match discard_state {
            DiscardingState::StillDiscarding => {
                let discarding_state = Discarding::new(scores, roles, hands, crib, deck);
                DiscardResult::Discarding(Game::<_>::new(discarding_state))
            }
            DiscardingState::ReadyToCut => {
                let cut = deck.cut();
                let score = CutScorer::new(cut).score();
                scores.score_points(roles.dealer(), &score);
                if let Some(winner) = scores.winner() {
                    let finished_state = Finished::new(winner, scores.peggings().clone());
                    DiscardResult::Finished(Game::<_>::new(finished_state))
                } else {
                    let pone = roles.pone();
                    let play_state = PlayState::new(pone, &hands);
                    let playing_state = Playing::new(scores, roles, hands, play_state, cut, crib);
                    DiscardResult::Playing(Game::<_>::new(playing_state))
                }
            }
        };

        Ok(result)
    }
}

#[derive(Debug)]
pub enum PlayResult {
    Playing(Game<Playing>),
    Scoring(Game<ScoringPone>),
    Finished(Game<Finished>),
}

#[derive(Debug)]
pub enum PassResult {
    Playing(Game<Playing>),
    Scoring(Game<ScoringPone>),
    Finished(Game<Finished>),
}

impl Game<Playing> {
    pub fn play(self, player: Player, card: Card) -> Result<PlayResult> {
        self.validate_player_card(player, card)?;
        self.validate_can_play(player, card)?;

        let (mut scores, roles, mut hands, mut play_state, cut, crib) = self.state.into_parts();
        let hand = hands.get_mut(&player).unwrap();
        hand.remove(card);

        play_state.play(card);

        let score_current_play = CurrentPlayScorer::new(&play_state).score();
        let score_end_of_play = EndOfPlayScorer::new(&play_state).score();

        let all_cards_are_played = play_state.all_are_cards_played();
        let end_of_play = play_state.target_reached() || all_cards_are_played;

        if end_of_play {
            play_state.start_new_play();
        }

        if all_cards_are_played {
            hands = play_state.finish_plays();
        }

        scores.score_points(player, &score_current_play);
        scores.score_points(player, &score_end_of_play);

        let result = if let Some(winner) = scores.winner() {
            let finished_state = Finished::new(winner, scores.peggings().clone());
            PlayResult::Finished(Game::<_>::new(finished_state))
        } else if all_cards_are_played {
            let scoring_state = ScoringPone::new(scores, roles, hands, cut, crib);
            PlayResult::Scoring(Game::<_>::new(scoring_state))
        } else {
            let playing_state = Playing::new(scores, roles, hands, play_state, cut, crib);
            PlayResult::Playing(Game::<_>::new(playing_state))
        };

        Ok(result)
    }

    pub fn pass(self, player: Player) -> Result<PassResult> {
        println!("pass: player: {} self: {}", player, self);
        self.validate_player(player)?;
        println!("pass2");
        self.validate_can_pass(player)?;
        println!("pass3");

        let (mut scores, roles, hands, mut play_state, cut, crib) = self.state.into_parts();

        play_state.pass();

        let all_cards_are_played = play_state.all_are_cards_played();

        let mut reasons = ScoreReasons::default();

        if play_state.pass_count() == NUMBER_OF_PLAYERS_IN_GAME {
            reasons = EndOfPlayScorer::new(&play_state).score();
            play_state.start_new_play();
        }

        scores.score_points(player, &reasons);

        let result = if let Some(winner) = scores.winner() {
            let finished_state = Finished::new(winner, scores.peggings().clone());
            PassResult::Finished(Game::<_>::new(finished_state))
        } else if all_cards_are_played {
            let scoring_state = ScoringPone::new(scores, roles, hands, cut, crib);
            PassResult::Scoring(Game::<_>::new(scoring_state))
        } else {
            let playing_state = Playing::new(scores, roles, hands, play_state, cut, crib);
            PassResult::Playing(Game::<_>::new(playing_state))
        };

        Ok(result)
    }
}

pub enum ScorePoneResult {
    Scoring(Game<ScoringDealer>),
    Finished(Game<Finished>),
}

impl Game<ScoringPone> {
    pub fn reasons(&self) -> Result<ScoreReasons> {
        let hand = self.hand(self.pone())?;
        Ok(HandScorer::new(hand, self.cut()).score())
    }

    pub fn score_hand(self) -> Result<ScorePoneResult> {
        let reasons = self.reasons()?;

        let (mut scores, roles, hands, cut, crib) = self.state.into_parts();

        scores.score_points(roles.pone(), &reasons);

        let result = if let Some(winner) = scores.winner() {
            let finished_state = Finished::new(winner, scores.peggings().clone());
            ScorePoneResult::Finished(Game::<_>::new(finished_state))
        } else {
            let scoring_state = ScoringDealer::new(scores, roles, hands, cut, crib);
            ScorePoneResult::Scoring(Game::<_>::new(scoring_state))
        };

        Ok(result)
    }
}

pub enum ScoreDealerResult {
    Scoring(Game<ScoringCrib>),
    Finished(Game<Finished>),
}

impl Game<ScoringDealer> {
    pub fn reasons(&self) -> Result<ScoreReasons> {
        let hand = self.hand(self.dealer())?;
        Ok(HandScorer::new(hand, self.cut()).score())
    }

    pub fn score_hand(self) -> Result<ScoreDealerResult> {
        let reasons = self.reasons()?;

        let (mut scores, roles, hands, cut, crib) = self.state.into_parts();

        scores.score_points(roles.dealer(), &reasons);

        let result = if let Some(winner) = scores.winner() {
            let finished_state = Finished::new(winner, scores.peggings().clone());
            ScoreDealerResult::Finished(Game::<_>::new(finished_state))
        } else {
            let scoring_state = ScoringCrib::new(scores, roles, hands, cut, crib);
            ScoreDealerResult::Scoring(Game::<_>::new(scoring_state))
        };

        Ok(result)
    }
}

pub enum ScoreCribResult {
    Discarding(Game<Discarding>),
    Finished(Game<Finished>),
}

impl Game<ScoringCrib> {
    pub fn reasons(&self) -> Result<ScoreReasons> {
        let crib = self.crib();
        Ok(CribScorer::new(crib, self.cut()).score())
    }

    pub fn score_crib(self) -> Result<ScoreCribResult> {
        let reasons = self.reasons()?;
        let players = self.players();

        let (mut scores, roles, _, _, _) = self.state.into_parts();

        scores.score_points(roles.dealer(), &reasons);

        let result = if let Some(winner) = scores.winner() {
            let finished_state = Finished::new(winner, scores.peggings().clone());
            ScoreCribResult::Finished(Game::<_>::new(finished_state))
        } else {
            let mut deck = Deck::shuffled_pack();
            let hands = deck.deal(&players);
            let crib = Crib::default();
            let discarding_state = Discarding::new(scores, roles, hands, crib, deck);
            ScoreCribResult::Discarding(Game::<_>::new(discarding_state))
        };

        Ok(result)
    }
}

impl Game<Finished> {
    pub fn winner(&self) -> Player {
        self.state.winner()
    }

    pub fn peggings(&self) -> &Peggings {
        self.state.peggings()
    }
}

impl<T: std::fmt::Display> std::fmt::Display for Game<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.state.fmt(f)
    }
}

// #[cfg(test)]
// mod test {
//     use super::*;

//     #[test]
//     fn redeal_after_crib_scored() {
//         let game0 = Builder::new(2)
//             .with_peggings(0, 0)
//             .with_cut("4H")
//             .with_hands("7H8CAC2C", "JCKS5HTH")
//             .with_crib("AHADASTD")
//             .as_scoring_crib();
//         let pone0 = game0.pone();
//         let Game::ScoringCrib(scores0, dealer0, _, _, _) = game0.clone() else {
//             panic!("Unexpected state")
//         };

//         let game1 = game0.deal_next_hands().unwrap;
//         let pone1 = game1.pone();
//         let Game::Discarding(scores1, dealer1, hands1, crib1, deck1) = game1.clone() else {
//             panic!("Unexpected state")
//         };

//         assert_eq!(scores1, scores0);
//         assert_eq!(dealer1, pone0);
//         assert_eq!(pone1, dealer0);
//         assert_eq!(hands1[&dealer1].len(), 6);
//         assert_eq!(hands1[&pone1].len(), 6);
//         assert!(crib1.is_empty());
//         assert_eq!(deck1.len(), 40);
//     }

//     #[test]
//     fn fail_redeal_when_crib_not_scored() {
//         let game0 = Builder::new(2)
//             .with_peggings(0, 0)
//             .with_cut("4H")
//             .with_hands("7H8CAC2C", "JCKS5HTH")
//             .with_crib("AHADASTD")
//             .as_scoring_dealer();

//         let error = game0.deal_next_hands().err().unwrap();
//         assert_eq!(error, GameError::ActionNotPermitted);
//     }
