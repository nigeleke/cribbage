#![forbid(clippy::expect_used)]

use std::collections::HashMap;

use thiserror::*;

use super::{
    HandScorer, HasPlayState, Peggings,
    cards::{
        Card, Crib, Cut, Cuts, Deck, Hand, Hands, HasCrib, HasCut, HasCuts, HasDeck, HasHands,
    },
    players::{HasPlayers, HasRoles, Player, Players, Roles, RolesError},
    plays::PlayState,
    scorers::{CribScorer, CurrentPlayScorer, CutScorer, EndOfPlayScorer, Scorer},
    scoring::{HasScores, Pegging, ScoreReasons, Scores},
    state::{
        Discarding, DiscardingState, Finished, Playing, ScoringCrib, ScoringDealer, ScoringPone,
        Starting,
    },
};
use crate::constants::*;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum GameError {
    #[error("internal error")]
    InternalError(String),

    #[error("incorrect number of players: {0} given, 2 required")]
    IncorrectNumberOfPlayers(usize),

    #[error("player {0} not in game")]
    PlayerNotInGame(Player),

    #[error("only two cards can be discarded to the crib")]
    TooManyDiscards,

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
    PlayOrPassNotPermittedByPlayer,

    #[error("cannot play the desired card")]
    CannotPlayCard,

    #[error("not this player's turn to pass")]
    CannotPass,
}

type Result<T> = std::result::Result<T, GameError>;

/// The game state, waiting for opponent, discarding, playing, scoring, finished.
#[derive(Clone, Debug)]
pub struct Game<T> {
    state: T,
    _marker: std::marker::PhantomData<T>,
}

impl<T> Game<T> {
    pub const fn new(state: T) -> Self {
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
}

impl<T: HasScores> HasScores for Game<T> {
    fn scores(&self) -> &Scores {
        self.state.scores()
    }
}

impl<T: HasCrib> HasCrib for Game<T> {
    fn crib(&self) -> &Crib {
        self.state.crib()
    }
}

impl<T: HasPlayState> HasPlayState for Game<T> {
    fn play_state(&self) -> &PlayState {
        self.state.play_state()
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
        self.hands()
            .get(&player)
            .map_or(Err(GameError::InvalidPlayer(player)), |hand| {
                if hand.contains(&card) {
                    Ok(())
                } else {
                    Err(GameError::InvalidCard(card))
                }
            })
    }

    fn validate_player_cards(&self, player: Player, cards: &[Card]) -> Result<()> {
        self.hands()
            .get(&player)
            .map_or(Err(GameError::InvalidPlayer(player)), |hand| {
                if hand.contains_all(cards) {
                    Ok(())
                } else {
                    Err(GameError::InvalidCards)
                }
            })
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
}

impl<T: HasScores> Game<T> {
    pub fn pegging(&self, player: Player) -> Result<&Pegging> {
        self.peggings()
            .get(&player)
            .ok_or(GameError::InvalidPlayer(player))
    }
}

impl<T: HasPlayState> Game<T> {
    fn validate_next_to_play(&self, player: Player) -> Result<()> {
        if self.play_state().next_to_play() == player {
            Ok(())
        } else {
            Err(GameError::PlayOrPassNotPermittedByPlayer)
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

impl Game<Starting> {
    pub fn try_new(value: &Players) -> Result<Self> {
        if value.len() == NUMBER_OF_PLAYERS_IN_GAME {
            let mut deck = Deck::shuffled_pack();
            let make_cut = |mut cuts: Cuts, player: &Player| {
                let cut = deck.cut();
                cuts.insert(*player, cut);
                cuts
            };
            let cuts = value.iter().fold(HashMap::new(), make_cut);
            Ok(Self::new(Starting::new(cuts, deck)))
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

    pub fn redraw(self) -> Result<Self> {
        match Roles::try_from(self.cuts()) {
            Ok(_) => Err(GameError::CutForStartDecided),
            Err(_) => Self::try_new(&self.players()),
        }
    }
}

#[derive(Debug)]
pub enum DiscardResult {
    Discarding(Box<Game<Discarding>>),
    Playing(Box<Game<Playing>>),
    Finished(Box<Game<Finished>>),
}

impl Game<Discarding> {
    pub fn discard(mut self, player: Player, discards: &[Card]) -> Result<DiscardResult> {
        self.validate_player_discards(player, discards)?;

        let discard_state = self.state.discard(player, discards);
        let (mut scores, roles, hands, crib, mut deck) = self.state.into_parts();
        let result = match discard_state {
            DiscardingState::StillDiscarding => {
                let discarding_state = Discarding::new(scores, roles, hands, crib, deck);
                DiscardResult::Discarding(Box::new(Self::new(discarding_state)))
            }
            DiscardingState::ReadyToCut => {
                let cut = deck.cut();
                let score = CutScorer::new(cut).score();
                scores.score_points(roles.dealer(), &score);
                if let Some(winner) = scores.winner() {
                    let finished_state = Finished::new(winner, scores.peggings().clone(), cut);
                    DiscardResult::Finished(Box::new(Game::<_>::new(finished_state)))
                } else {
                    let pone = roles.pone();
                    let play_state = PlayState::new(pone, &hands);
                    let playing_state = Playing::new(scores, roles, hands, play_state, cut, crib);
                    DiscardResult::Playing(Box::new(Game::<_>::new(playing_state)))
                }
            }
        };

        Ok(result)
    }
}

#[derive(Debug)]
pub enum PlayResult {
    Playing(Box<Game<Playing>>),
    Scoring(Box<Game<ScoringPone>>),
    Finished(Box<Game<Finished>>),
}

#[derive(Debug)]
pub enum PassResult {
    Playing(Box<Game<Playing>>),
    Finished(Box<Game<Finished>>),
}

impl Game<Playing> {
    pub fn play(self, player: Player, card: Card) -> Result<PlayResult> {
        self.validate_player_card(player, card)?;
        self.validate_can_play(player, card)?;

        let (mut scores, roles, mut hands, mut play_state, cut, crib) = self.state.into_parts();
        let hand = hands
            .get_mut(&player)
            .ok_or(GameError::InvalidPlayer(player))?;
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
            let finished_state = Finished::new(winner, scores.peggings().clone(), cut);
            PlayResult::Finished(Box::new(Game::<_>::new(finished_state)))
        } else if all_cards_are_played {
            let scoring_state = ScoringPone::new(scores, roles, hands, cut, crib);
            PlayResult::Scoring(Box::new(Game::<_>::new(scoring_state)))
        } else {
            let playing_state = Playing::new(scores, roles, hands, play_state, cut, crib);
            PlayResult::Playing(Box::new(Self::new(playing_state)))
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

        let mut reasons = ScoreReasons::default();

        if play_state.pass_count() == NUMBER_OF_PLAYERS_IN_GAME {
            reasons = EndOfPlayScorer::new(&play_state).score();
            play_state.start_new_play();
        }

        scores.score_points(player, &reasons);

        let result = if let Some(winner) = scores.winner() {
            let finished_state = Finished::new(winner, scores.peggings().clone(), cut);
            PassResult::Finished(Box::new(Game::<_>::new(finished_state)))
        } else {
            let playing_state = Playing::new(scores, roles, hands, play_state, cut, crib);
            PassResult::Playing(Box::new(Self::new(playing_state)))
        };

        Ok(result)
    }
}

pub enum ScorePoneResult {
    Scoring(Box<Game<ScoringDealer>>),
    Finished(Box<Game<Finished>>),
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
            let finished_state = Finished::new(winner, scores.peggings().clone(), cut);
            ScorePoneResult::Finished(Box::new(Game::<_>::new(finished_state)))
        } else {
            let scoring_state = ScoringDealer::new(scores, roles, hands, cut, crib);
            ScorePoneResult::Scoring(Box::new(Game::<_>::new(scoring_state)))
        };

        Ok(result)
    }
}

pub enum ScoreDealerResult {
    Scoring(Box<Game<ScoringCrib>>),
    Finished(Box<Game<Finished>>),
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
            let finished_state = Finished::new(winner, scores.peggings().clone(), cut);
            ScoreDealerResult::Finished(Box::new(Game::<_>::new(finished_state)))
        } else {
            let scoring_state = ScoringCrib::new(scores, roles, hands, cut, crib);
            ScoreDealerResult::Scoring(Box::new(Game::<_>::new(scoring_state)))
        };

        Ok(result)
    }
}

pub enum ScoreCribResult {
    Discarding(Box<Game<Discarding>>),
    Finished(Box<Game<Finished>>),
}

impl Game<ScoringCrib> {
    pub fn reasons(&self) -> Result<ScoreReasons> {
        let crib = self.crib();
        Ok(CribScorer::new(crib, self.cut()).score())
    }

    pub fn score_crib(self) -> Result<ScoreCribResult> {
        let reasons = self.reasons()?;
        let players = self.players();

        let (mut scores, mut roles, _, cut, _) = self.state.into_parts();

        scores.score_points(roles.dealer(), &reasons);

        let result = if let Some(winner) = scores.winner() {
            let finished_state = Finished::new(winner, scores.peggings().clone(), cut);
            ScoreCribResult::Finished(Box::new(Game::<_>::new(finished_state)))
        } else {
            let mut deck = Deck::shuffled_pack();
            let hands = deck.deal(&players);
            let crib = Crib::default();
            roles.swap();
            let discarding_state = Discarding::new(scores, roles, hands, crib, deck);
            ScoreCribResult::Discarding(Box::new(Game::<_>::new(discarding_state)))
        };

        Ok(result)
    }
}

impl Game<Finished> {
    pub const fn winner(&self) -> Player {
        self.state.winner()
    }

    pub const fn peggings(&self) -> &Peggings {
        self.state.peggings()
    }

    pub const fn cut(&self) -> Cut {
        self.state.cut()
    }
}

impl<T: std::fmt::Display> std::fmt::Display for Game<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.state.fmt(f)
    }
}

#[cfg(test)]
mod test {
    /// # [Cribbage Rules](https://www.officialgamerules.org/cribbage)
    use crate::prelude::*;
    use crate::test_modules::*;

    /// ## Number of Players
    ///
    /// Two or three people can play. Or four people can play two against two as partners. But
    /// Cribbage is basically best played by two people, and the rules that follow are for that
    /// number.
    mod players {
        use super::*;

        #[test]
        fn constructed_with_two_players() {
            let builder = GameBuilder::default();
            let players = builder.players();
            let game = builder.into_new().expect("valid into_new");
            assert_eq!(game.players().len(), 2);
            for player in players.into_iter() {
                assert!(game.players().contains(&player))
            }
        }

        #[test]
        fn fails_constuction_without_two_players() {
            [0, 1, 3, 4].into_iter().for_each(|n| {
                let error = GameBuilder::new(n)
                    .into_new()
                    .expect_err("invalid into_new");
                assert!(matches!(error, GameError::IncorrectNumberOfPlayers(_)));
            });
        }

        #[test]
        fn provide_opponent_to_existing_player() {
            let game = GameBuilder::default().into_new().expect("valid into_new");
            let (player, opponent) = game.player_1_2().expect("valid player_1_2");

            assert_eq!(game.opponent(player).expect("valid opponent"), opponent);
            assert_eq!(game.opponent(opponent).expect("valid opponent"), player);
        }

        #[test]
        fn fail_to_provide_opponent_for_invalid_player() {
            let game = GameBuilder::default().into_new().expect("valid into_new");
            let error = game.opponent(Player::new()).expect_err("invalid opponent");
            assert!(matches!(error, GameError::InvalidPlayer(_)))
        }
    }

    /// ## The Pack
    ///
    /// The standard 52-card pack is used.
    ///
    /// Rank of Cards: K (high), Q, J, 10, 9, 8, 7, 6, 5, 4, 3, 2, A.
    mod deck {
        use super::*;

        #[test]
        fn use_a_standard_pack_of_cards() {
            let builder = GameBuilder::default();
            let game = builder.into_new().expect("valid into_new");
            let deck = game.deck();
            assert_eq!(deck.len(), 50);
            let (player1, player2) = game.player_1_2().expect("valid player_1_2");
            assert!(!deck.contains(&game.cut(player1).expect("valid cut")));
            assert!(!deck.contains(&game.cut(player2).expect("valid cut")));
        }
    }

    /// ## The Draw, Shuffle and Cut
    ///
    /// From a shuffled pack face down, each player cuts a card, leaving at least four cards at
    /// either end of the pack.
    ///
    /// If both players cut cards of the same rank, each draws again. The player with the lower card
    /// deals the first hand. Thereafter, the turn to deal alternates between the two players,
    /// except that the loser of the game deals first if another game is played. The dealer has the
    /// right to shuffle last, and he presents the cards to the non-dealer for the cut prior to the
    /// deal. (In some games, there is no cut at this time.)
    mod deal_cut {
        use super::*;

        #[test]
        fn start_game_with_lowest_cut_as_dealer() {
            for (expected_dealer, cuts) in [(0, "ASKS"), (1, "KSAS")] {
                let builder = GameBuilder::default().with_cuts(cuts);
                let players = builder.players();
                let cuts = builder.cuts();
                let game = builder.into_starting();
                let game = game.start().expect("valid start");
                let roles = Roles::try_from(&cuts).expect("valid try_from");

                assert_eq!(game.dealer(), players[expected_dealer]);
                assert_eq!(game.pone(), players[1 - expected_dealer]);
                assert_eq!(game.dealer(), roles.dealer());
                assert_eq!(game.pone(), roles.pone());
            }
        }

        #[test]
        fn fail_to_start_game_if_cuts_are_the_same_value() {
            let game = GameBuilder::default().with_cuts("ASAC").into_starting();
            let error = game.start().expect_err("invalid start");
            assert!(matches!(error, GameError::CannotDetermineRoles(_)));
        }

        #[test]
        fn redraw_if_cuts_are_same_value() {
            use std::any::{Any, TypeId};
            let game0 = GameBuilder::default().with_cuts("ASAC").into_starting();
            let game0_players = game0.players();
            let game1 = game0.redraw().expect("valid redraw");
            assert_eq!(game1.type_id(), TypeId::of::<Game<Starting>>());
            assert_eq!(game1.players(), game0_players);
        }

        #[test]
        fn fail_to_redraw_if_cuts_are_not_the_same_value() {
            let game = GameBuilder::default().with_cuts("ASKS").into_starting();
            let error = game.redraw().expect_err("invalid redraw");
            assert_eq!(error, GameError::CutForStartDecided);
        }

        #[test]
        fn cannot_get_cut_when_player_not_participating() {
            let game = GameBuilder::default().with_cuts("AHAS").into_starting();
            let non_player = Player::new();
            let error = game.cut(non_player).expect_err("invalid cut");
            assert_eq!(error, GameError::InvalidPlayer(non_player));
        }
    }

    /// ## The Deal
    ///
    /// The dealer distributes six cards face down to his opponent and himself, beginning with the
    /// opponent.
    mod deal {
        use super::*;

        #[test]
        fn deal_six_cards_per_player() {
            let game = GameBuilder::default().with_cuts("ASKS").into_starting();

            let game = game.start().expect("valid start");
            let players = game.players();
            assert_eq!(players.len(), 2);

            players.into_iter().for_each(|p| {
                assert_eq!(
                    game.hand(p).expect("valid hand").len(),
                    CARDS_DEALT_PER_HAND
                )
            });
        }

        #[test]
        fn deal_when_draw_decided() {
            let game = GameBuilder::default().with_cuts("ASKS").into_starting();
            let game = game.start().expect("valid start");
            let players = game.players();

            players.iter().for_each(|p| {
                assert_eq!(
                    *game.pegging(*p).expect("valid pegging").back_peg().points(),
                    0
                );
                assert_eq!(
                    *game
                        .pegging(*p)
                        .expect("valid pegging")
                        .front_peg()
                        .points(),
                    0
                );
            });

            players.iter().for_each(|p| {
                assert_eq!(
                    game.hand(*p).expect("valid hand").len(),
                    CARDS_DEALT_PER_HAND
                );
            });

            assert_eq!(game.crib().len(), 0);
            assert_eq!(
                52 - (NUMBER_OF_PLAYERS_IN_GAME * CARDS_DEALT_PER_HAND),
                game.deck().len(),
            );
        }
    }

    //   ## Object of the Game

    //   The goal is to be the first player to score 121 points. (Some games are to 61 points.)
    //   Players earn points during play and for making various card combinations.
    mod object_of_the_game {}

    /// ## The Crib
    ///
    /// Each player looks at his six cards and "lays away" (discards) two of them face down to
    /// reduce the hand to four. The four cards laid away together constitute "the crib". The crib
    /// belongs to the dealer, but these cards are not exposed or used until after the hands have
    /// been played.
    mod the_crib {
        use super::*;

        #[test]
        fn player_can_discard_one_held_card_to_the_crib() {
            let game0 = GameBuilder::default()
                .with_peggings(0, 0)
                .with_hands("AH2H3H4H5H6H", "AC2C3C4C5C6C")
                .into_discarding();

            let (player0, opponent0) = game0.player_1_2().expect("valid player_1_2");

            let player_hand0 = game0.hand(player0).expect("valid hand");
            let player_discard = player_hand0.get(&[0]);

            let opponent_hand0 = game0.hand(opponent0).expect("valid hand").clone();
            let scores0 = game0.scores().clone();
            let dealer0 = game0.dealer();
            let deck0 = game0.deck().clone();

            let DiscardResult::Discarding(game1) = game0
                .discard(player0, &player_discard)
                .expect("valid discard")
            else {
                panic!("unexpected state")
            };

            let player_hand1 = game1.hand(player0).expect("valid hand");
            let opponent_hand1 = game1.hand(opponent0).expect("valid hand");

            assert_eq!(game1.scores(), &scores0);
            assert_eq!(game1.dealer(), dealer0);
            assert!(player_hand1.contains_none(&player_discard));
            assert!(game1.crib().contains_all(&player_discard));
            assert_eq!(opponent_hand1, &opponent_hand0);
            assert_eq!(game1.deck(), &deck0);
        }

        #[test]
        fn player_can_discard_two_held_cards_to_the_crib() {
            let game0 = GameBuilder::default()
                .with_peggings(0, 0)
                .with_hands("AH2H3H4H5H6H", "AC2C3C4C5C6C")
                .into_discarding();

            let (player0, opponent0) = game0.player_1_2().expect("valid player_1_2");

            let scores0 = game0.scores().clone();
            let dealer0 = game0.dealer();
            let hands0 = game0.hands().clone();
            let deck0 = game0.deck().clone();
            let player_hand0 = hands0[&player0].clone();
            let player_discard = player_hand0.get(&[0, 1]);

            let opponent_hand0 = hands0[&opponent0].clone();

            let DiscardResult::Discarding(game1) = game0
                .discard(player0, &player_discard)
                .expect("valid discard")
            else {
                panic!("unexpected state")
            };

            let scores1 = game1.scores();
            let dealer1 = game1.dealer();
            let hands1 = game1.hands();
            let crib1 = game1.crib();
            let deck1 = game1.deck();

            let player_hand1 = hands1[&player0].clone();
            let opponent_hand1 = hands1[&opponent0].clone();

            assert_eq!(scores1, &scores0);
            assert_eq!(dealer1, dealer0);
            assert!(player_hand1.contains_none(&player_discard));
            assert!(crib1.contains_all(&player_discard));
            assert_eq!(opponent_hand1, opponent_hand0);
            assert_eq!(deck1, &deck0);
        }

        #[test]
        fn player_cannot_discard_more_then_two_held_cards_to_the_crib() {
            let game0 = GameBuilder::default()
                .with_peggings(0, 0)
                .with_hands("AH2H3H4H5H6H", "AC2C3C4C5C6C")
                .into_discarding();

            let (player0, _) = game0.player_1_2().expect("valid player_1_2");

            let hands0 = game0.hands();
            let hand0 = hands0[&player0].clone();
            let discards0 = hand0.get(&[0, 1, 2]);

            let error = game0
                .discard(player0, &discards0)
                .expect_err("invalid discard");
            assert_eq!(error, GameError::TooManyDiscards);
        }

        #[test]
        fn player_cannot_discard_non_held_cards_to_the_crib() {
            let game0 = GameBuilder::default()
                .with_peggings(0, 0)
                .with_hands("AH2H3H4H5H6H", "AC2C3C4C5C6C")
                .into_discarding();

            let (player0, opponent0) = game0.player_1_2().expect("valid player_1_2");

            let hands0 = game0.hands();
            let hand0 = hands0[&opponent0].clone();
            let discards0 = hand0.get(&[0, 1]);

            let error = game0
                .discard(player0, &discards0)
                .expect_err("invalid discard");
            assert_eq!(error, GameError::InvalidCards);
        }

        #[test]
        fn cannot_discard_when_player_not_participating() {
            let game0 = GameBuilder::default()
                .with_peggings(0, 0)
                .with_hands("AH2H3H4H5H6H", "AC2C3C4C5C6C")
                .into_discarding();

            let (player0, _) = game0.player_1_2().expect("valid player_1_2");

            let hands0 = game0.hands();
            let hand0 = hands0[&player0].clone();
            let discards0 = hand0.get(&[0, 1]);

            let non_player = Player::new();
            let error = game0
                .discard(non_player, &discards0)
                .expect_err("invalid discard");
            assert_eq!(error, GameError::InvalidPlayer(non_player));
        }
    }

    /// ## Before the Play
    ///
    /// After the crib is laid away, the non-dealer cuts the pack. The dealer turns up the top card
    /// of the lower packet and places it face up on top of the pack. This card is the "starter." If
    /// the starter is a jack, it is called "His Heels," and the dealer pegs (scores) 2 points at
    /// once. The starter is not used in the play phase of Cribbage , but is used later for making
    /// svarious card combinations that score points.
    mod before_the_play {
        use super::*;

        fn after_discards_common_tests() -> (Scores, Scores, Cut, Player, Player) {
            let game0 = GameBuilder::default()
                .with_peggings(0, 0)
                .with_hands("AH2H3H4H5H6H", "AC2C3C4C5C6C")
                .into_discarding();

            let (player0, opponent0) = game0.player_1_2().expect("valid player_1_2");

            let scores0 = game0.scores().clone();
            let dealer0 = game0.dealer();
            let hands0 = game0.hands();
            let deck0 = game0.deck().clone();
            let pone0 = game0.pone();

            let player_hand0 = hands0[&player0].clone();
            let player_discard = player_hand0.get(&[0, 1]);

            let opponent_hand0 = hands0[&opponent0].clone();
            let opponent_discard = opponent_hand0.get(&[0, 1]);

            let DiscardResult::Discarding(game1) = game0
                .discard(player0, &player_discard)
                .expect("valid discard")
            else {
                panic!("unexpected state")
            };

            let DiscardResult::Playing(game1) = game1
                .discard(opponent0, &opponent_discard)
                .expect("valid discard")
            else {
                panic!("unexpected state")
            };

            let scores1 = game1.scores();
            let dealer1 = game1.dealer();
            let hands1 = game1.hands();
            let play_state1 = game1.play_state();
            let cut1 = game1.cut();
            let crib1 = game1.crib();

            let hand1 = hands1[&player0].clone();
            let opponent_hand1 = hands1[&opponent0].clone();

            assert_eq!(dealer1, dealer0);
            assert!(hand1.contains_none(&player_discard));
            assert!(crib1.contains_all(&player_discard));
            assert!(opponent_hand1.contains_none(&opponent_discard));
            assert!(crib1.contains_all(&opponent_discard));
            assert!(deck0.contains(&cut1));
            assert_eq!(crib1.len(), CARDS_REQUIRED_IN_CRIB);
            assert_eq!(play_state1.legal_plays(pone0), hands1[&pone0]);
            assert_eq!(play_state1.legal_plays(dealer0), hands1[&dealer0]);
            assert_eq!(play_state1.pass_count(), 0);
            assert_eq!(play_state1.current_plays(), []);
            assert_eq!(play_state1.previous_plays(), []);

            (scores0.clone(), scores1.clone(), cut1, dealer1, pone0)
        }

        #[test]
        fn start_the_play_after_discards() {
            let (scores0, scores1, cut, dealer, pone) = after_discards_common_tests();
            if cut.face() == Face::Jack {
                assert_eq!(
                    scores0.peggings()[&dealer].add(2.into()),
                    scores1.peggings()[&dealer]
                );
                assert_eq!(scores0.peggings()[&pone], scores1.peggings()[&pone]);
            } else {
                assert_eq!(scores0, scores1)
            }
        }

        #[test]
        fn score_his_heels_when_jack_cut_after_discards() {
            loop {
                let (scores0, scores1, cut, dealer, pone) = after_discards_common_tests();
                if cut.face() == Face::Jack {
                    assert_eq!(
                        scores0.peggings()[&dealer].add(2.into()),
                        scores1.peggings()[&dealer]
                    );
                    assert_eq!(scores0.peggings()[&pone], scores1.peggings()[&pone]);
                    break;
                }
            }
        }

        #[test]
        fn finish_game_when_jack_cut_after_discards() {
            loop {
                let game0 = GameBuilder::default()
                    .with_peggings(120, 0)
                    .with_hands("AH2H3H4H5H6H", "AC2C3C4C5C6C")
                    .into_discarding();

                let (player0, opponent0) = game0.player_1_2().expect("valid player_1_2");
                let hands0 = game0.hands();
                let player_hand0 = hands0[&player0].clone();
                let player_discard = player_hand0.get(&[0, 1]);
                let opponent_hand0 = hands0[&opponent0].clone();
                let opponent_discard = opponent_hand0.get(&[0, 1]);
                let peggings0 = game0.peggings().clone();

                let DiscardResult::Discarding(game1) = game0
                    .discard(player0, &player_discard)
                    .expect("valid discard")
                else {
                    panic!("unexpected state")
                };

                let result = game1
                    .discard(opponent0, &opponent_discard)
                    .expect("valid discard");
                match result {
                    DiscardResult::Finished(game1) => {
                        assert_eq!(game1.cut().face(), Face::Jack);
                        let winner1 = game1.winner();
                        let loser1 = if winner1 == player0 {
                            opponent0
                        } else {
                            player0
                        };
                        let peggings1 = game1.peggings();
                        assert_eq!(peggings1[&winner1], peggings0[&winner1].add(2.into()));
                        assert_eq!(peggings1[&loser1], peggings0[&loser1]);
                        break;
                    }
                    DiscardResult::Playing(_) => {}
                    _ => panic!("unexpected state"),
                }
            }
        }
    }

    /// ## The Play
    ///
    /// After the starter is turned, the non-dealer lays one of his cards face up on the table. The
    /// dealer similarly exposes a card, then non-dealer again, and so on - the hands are exposed
    /// card by card, alternately except for a "Go," (Pass) as noted below. Each player keeps his
    /// cards separate from those of his opponent.
    ///
    /// As each person plays, he announces a running total of pips reached by the addition of the
    /// last card to all those previously2 played. (Example: The non-dealer begins with a four,
    /// saying "Four." The dealer plays a nine, saying "Thirteen".) The kings, queens and jacks
    /// count 10 each; every other card counts its pip value (the ace counts one).
    mod the_play {
        use super::*;

        #[test]
        fn accept_valid_play() {
            let game0 = GameBuilder::default()
                .with_peggings(0, 0)
                .with_hands("9S", "4S")
                .with_cut("AS")
                .into_playing(1);
            let pone0 = game0.pone();
            let scores0 = game0.scores();
            let dealer0 = game0.dealer();
            let hands0 = game0.hands();
            let play_state0 = game0.play_state();
            let cut0 = game0.cut();
            let crib0 = game0.crib().clone();
            let dealer_hand0 = hands0[&dealer0].clone();
            let dealer_score0 = scores0.peggings()[&dealer0];
            let pone_score0 = scores0.peggings()[&pone0];

            assert_eq!(play_state0.legal_plays(dealer0), hands0[&dealer0]);
            assert_eq!(play_state0.legal_plays(pone0), valid_hand("4S"));

            let PlayResult::Playing(game1) =
                game0.play(pone0, valid_card("4S")).expect("valid play")
            else {
                panic!("unexpected state")
            };

            let scores1 = game1.scores();
            let dealer1 = game1.dealer();
            let pone1 = game1.pone();
            let hands1 = game1.hands();
            let play_state1 = game1.play_state();
            let cut1 = game1.cut();
            let crib1 = game1.crib();
            let dealer_hand1 = hands1[&dealer1].clone();
            let pone_hand1 = hands1[&pone1].clone();
            let dealer_score1 = scores1.peggings()[&dealer1];
            let pone_score1 = scores1.peggings()[&pone1];

            assert_eq!(dealer_score1, dealer_score0);
            assert_eq!(pone_score1, pone_score0);
            assert_eq!(dealer1, dealer0);
            assert_eq!(dealer_hand1, dealer_hand0);
            assert_eq!(pone_hand1, Hand::default());
            assert_eq!(play_state1.next_to_play(), dealer1);
            assert_eq!(play_state1.legal_plays(dealer1), dealer_hand1);
            assert_eq!(cut1, cut0);
            assert_eq!(crib1, &crib0);
        }

        #[test]
        fn cannot_play_when_player_not_participating() {
            let game0 = GameBuilder::default()
                .with_peggings(0, 0)
                .with_hands("9S", "4S")
                .with_cut("AS")
                .into_playing(1);

            let non_player = Player::new();
            let error = game0
                .play(non_player, valid_card("4S"))
                .expect_err("invalid play");
            assert_eq!(error, GameError::InvalidPlayer(non_player));
        }

        #[test]
        fn cannot_play_when_unheld_card() {
            let game0 = GameBuilder::default()
                .with_peggings(0, 0)
                .with_hands("9S", "4S")
                .with_cut("AS")
                .into_playing(1);
            let pone0 = game0.pone();
            let card = valid_card("9S");
            let error = game0.play(pone0, card).expect_err("invalid play");
            assert_eq!(error, GameError::InvalidCard(card));
        }

        #[test]
        fn cannot_play_when_not_their_turn() {
            let game0 = GameBuilder::default()
                .with_peggings(0, 0)
                .with_hands("9S", "4S")
                .with_cut("AS")
                .into_playing(1);
            let dealer0 = game0.dealer();
            let card = valid_card("9S");
            let error = game0.play(dealer0, card).expect_err("invalid play");
            assert_eq!(error, GameError::PlayOrPassNotPermittedByPlayer);
        }

        #[test]
        fn cannot_play_when_play_exceeds_target() {
            let game0 = GameBuilder::default()
                .with_peggings(0, 0)
                .with_hands("9S", "4S")
                .with_cut("AS")
                .with_current_plays(&[(0, "KH"), (0, "KC"), (0, "KD")])
                .into_playing(1);
            let pone0 = game0.pone();

            let play_state0 = game0.play_state();
            assert_eq!(play_state0.legal_plays(pone0), valid_hand(""));

            let error = game0
                .play(pone0, valid_card("4S"))
                .expect_err("invalid play");
            assert_eq!(error, GameError::CannotPlayCard)
        }

        #[test]
        fn score_play_when_target_not_reached_mid_play() {
            let game0 = GameBuilder::default()
                .with_peggings(0, 0)
                .with_hands("5S", "5H")
                .with_cut("AS")
                .with_current_plays(&[(0, "TH")])
                .into_playing(1);
            let pone0 = game0.pone();
            let scores0 = game0.scores();
            let score0_pone = scores0.peggings()[&pone0];

            let PlayResult::Playing(game1) =
                game0.play(pone0, valid_card("5H")).expect("valid play")
            else {
                panic!("unexpected state")
            };

            let scores1 = game1.scores();
            let dealer1 = game1.dealer();
            let play_state1 = game1.play_state();
            let score1_pone = scores1.peggings()[&pone0];

            assert_eq!(score1_pone, score0_pone.add(2.into()));
            assert_eq!(play_state1.next_to_play(), dealer1);
        }

        #[test]
        fn score_play_when_target_not_reached_end_play() {
            let game0 = GameBuilder::default()
                .with_peggings(0, 0)
                .with_hands("QS", "2H")
                .with_cut("QC")
                .with_current_plays(&[(0, "JH"), (0, "QH")])
                .with_previous_plays(&[(0, "7C"), (1, "6S"), (1, "2S"), (1, "KS")])
                .into_playing(1);

            let scores0 = game0.scores();
            let dealer0 = game0.dealer();
            let pone0 = game0.pone();
            let score0_dealer = scores0.peggings()[&dealer0];
            let score0_pone = scores0.peggings()[&pone0];

            let PlayResult::Playing(game1) =
                game0.play(pone0, valid_card("2H")).expect("valid play")
            else {
                panic!("unexpected state")
            };

            let scores1 = game1.scores();
            let dealer1 = game1.dealer();
            let play_state1 = game1.play_state();
            let score1_pone = scores1.peggings()[&pone0];
            let score1_dealer = scores1.peggings()[&dealer1];

            assert_eq!(score1_pone, score0_pone.add(1.into()));
            assert_eq!(score1_dealer, score0_dealer);
            assert_eq!(play_state1.next_to_play(), dealer1);
            // TODO: assert score_history...
        }

        #[test]
        fn score_play_when_target_not_reached_finished() {
            let game0 = GameBuilder::default()
                .with_peggings(0, 120)
                .with_hands("AH", "5H")
                .with_cut("QC")
                .with_current_plays(&[(0, "JH")])
                .with_previous_plays(&[(0, "9H"), (0, "7C"), (1, "6S"), (1, "2S"), (1, "KS")])
                .into_playing(1);
            let scores0 = game0.scores();
            let pone0 = game0.pone();
            let score0_pone = scores0.peggings()[&pone0];

            let PlayResult::Finished(game1) =
                game0.play(pone0, valid_card("5H")).expect("valid play")
            else {
                panic!("unexpected state")
            };

            let winner1 = game1.winner();
            let peggings1 = game1.peggings();
            let score1_pone = peggings1[&pone0];

            assert_eq!(winner1, pone0);
            assert_eq!(score1_pone, score0_pone.add(2.into()));
        }

        #[test]
        fn score_play_when_target_reached_mid_play() {
            let card = valid_card("AH");
            let game0 = GameBuilder::default()
                .with_peggings(0, 0)
                .with_hands("9H", "AH")
                .with_cut("KC")
                .with_current_plays(&[(0, "TH"), (0, "JH"), (0, "QH")])
                .with_previous_plays(&[(1, "2S"), (1, "QS"), (1, "6S")])
                .into_playing(1);
            let scores0 = game0.scores().clone();
            let dealer0 = game0.dealer();
            let pone0 = game0.pone();
            let play_state0 = game0.play_state().clone();

            let PlayResult::Playing(game1) = game0.play(pone0, card).expect("valid play") else {
                panic!("unexpected event")
            };

            let scores1 = game1.scores();
            let dealer1 = game1.dealer();
            let hands1 = game1.hands();
            let play_state1 = game1.play_state();
            assert_eq!(
                scores1.peggings()[&pone0],
                scores0.peggings()[&pone0].add(2.into())
            );
            assert_eq!(dealer1, dealer0);
            assert!(!hands1[&pone0].contains(&card));
            assert_eq!(play_state1.next_to_play(), dealer0);
            assert!(play_state1.current_plays().is_empty());
            for p in play_state0.current_plays().into_iter() {
                assert!(play_state1.previous_plays().contains(&p))
            }
        }

        #[test]
        fn score_play_when_target_reached_end_play() {
            let card = valid_card("AH");
            let game0 = GameBuilder::default()
                .with_peggings(0, 0)
                .with_hands("QC", "AH")
                .with_cut("KC")
                .with_current_plays(&[(0, "TH"), (0, "JH"), (0, "QH")])
                .with_previous_plays(&[(1, "2S"), (1, "QS"), (1, "6S")])
                .into_playing(1);
            let scores0 = game0.scores().clone();
            let dealer0 = game0.dealer();
            let pone0 = game0.pone();
            let cut0 = game0.cut();
            let crib0 = game0.crib().clone();

            let PlayResult::Playing(game1) = game0.play(pone0, card).expect("valid play") else {
                panic!("unexpected state")
            };

            let scores1 = game1.scores();
            let dealer1 = game1.dealer();
            let play_state1 = game1.play_state();
            let cut1 = game1.cut();
            let crib1 = game1.crib();

            assert_eq!(dealer1, dealer0);
            assert_eq!(
                scores1.peggings()[&pone0],
                scores0.peggings()[&pone0].add(2.into())
            );
            assert_eq!(play_state1.next_to_play(), dealer1);
            assert_eq!(cut1, cut0);
            assert_eq!(crib1, &crib0);
        }

        #[test]
        fn score_play_when_target_reached_finished() {
            let game0 = GameBuilder::default()
                .with_peggings(0, 120)
                .with_hands("QC", "AH")
                .with_cut("KC")
                .with_current_plays(&[(0, "TH"), (1, "JH"), (0, "QH")])
                .with_previous_plays(&[(1, "9H"), (1, "5S"), (0, "6S")])
                .into_playing(1);
            let scores0 = game0.scores().clone();
            let dealer0 = game0.dealer();
            let pone0 = game0.pone();

            let PlayResult::Finished(game1) =
                game0.play(pone0, valid_card("AH")).expect("valid play")
            else {
                panic!("unexpected state")
            };

            let winner1 = game1.winner();
            let peggings1 = game1.peggings();

            assert_eq!(winner1, pone0);
            assert_eq!(peggings1[&pone0], scores0.peggings()[&pone0].add(2.into()));
            assert_eq!(peggings1[&dealer0], scores0.peggings()[&dealer0]);
        }

        #[test]
        fn score_play_when_plays_finished_and_game_not_finished() {
            let game0 = GameBuilder::default()
                .with_peggings(0, 60)
                .with_hands("", "AH")
                .with_cut("KC")
                .with_current_plays(&[(0, "8H"), (1, "JH"), (0, "QH")])
                .with_previous_plays(&[(1, "9H"), (0, "4S"), (1, "5S"), (0, "6S")])
                .into_playing(1);
            let pone0 = game0.pone();
            let scores0 = game0.scores().clone();
            let dealer0 = game0.dealer();

            let PlayResult::Scoring(game1) =
                game0.play(pone0, valid_card("AH")).expect("valid play")
            else {
                panic!("unexpected state")
            };

            let peggings1 = game1.peggings();
            assert_eq!(peggings1[&pone0], scores0.peggings()[&pone0].add(1.into()));
            assert_eq!(peggings1[&dealer0], scores0.peggings()[&dealer0]);
        }

        #[test]
        fn score_play_when_plays_finished_and_game_finished() {
            let game0 = GameBuilder::default()
                .with_peggings(0, 120)
                .with_hands("", "AH")
                .with_cut("KC")
                .with_current_plays(&[(0, "8H"), (1, "JH"), (0, "QH")])
                .with_previous_plays(&[(1, "9H"), (0, "4S"), (1, "5S"), (0, "6S")])
                .into_playing(1);
            let pone0 = game0.pone();
            let scores0 = game0.scores().clone();
            let dealer0 = game0.dealer();

            let PlayResult::Finished(game1) =
                game0.play(pone0, valid_card("AH")).expect("valid play")
            else {
                panic!("unexpected state")
            };

            let winner1 = game1.winner();
            let peggings1 = game1.peggings();

            assert_eq!(winner1, pone0);
            assert_eq!(peggings1[&pone0], scores0.peggings()[&pone0].add(1.into()));
            assert_eq!(peggings1[&dealer0], scores0.peggings()[&dealer0]);
        }

        #[test]
        fn swap_player_after_pone_play() {
            let game0 = GameBuilder::default()
                .with_peggings(0, 0)
                .with_cut("AS")
                .with_hands("7H8H8D9C", "4S5STHJH")
                .into_playing(1);
            let dealer0 = game0.dealer();
            let pone0 = game0.pone();

            let PlayResult::Playing(game1) =
                game0.play(pone0, valid_card("4S")).expect("valid play")
            else {
                panic!("unexpected state")
            };

            let dealer1 = game1.dealer();
            let pone1 = game1.pone();
            let play_state1 = game1.play_state();

            assert_eq!(dealer1, dealer0);
            assert_eq!(pone1, pone0);
            assert_eq!(play_state1.next_to_play(), dealer1);
        }

        #[test]
        fn swap_player_after_dealer_play() {
            let game0 = GameBuilder::default()
                .with_peggings(0, 0)
                .with_cut("AS")
                .with_hands("7H8H8D9C", "5STHJH")
                .with_current_plays(&[(1, "4S")])
                .into_playing(0);
            let dealer0 = game0.dealer();
            let pone0 = game0.pone();

            let PlayResult::Playing(game1) =
                game0.play(dealer0, valid_card("9C")).expect("valid play")
            else {
                panic!("unexpected state")
            };

            let dealer1 = game1.dealer();
            let pone1 = game1.pone();
            let play_state1 = game1.play_state();

            assert_eq!(dealer1, dealer0);
            assert_eq!(pone1, pone0);
            assert_eq!(play_state1.next_to_play(), pone0);
        }

        #[test]
        fn reset_play_after_exact_target_reached() {
            let game0 = GameBuilder::default()
                .with_peggings(0, 0)
                .with_cut("AS")
                .with_hands("7H8H8D", "5STH")
                .with_current_plays(&[(1, "JH"), (0, "9C"), (1, "4S")])
                .into_playing(0);
            let dealer0 = game0.dealer();
            let pone0 = game0.pone();
            let play_state0 = game0.play_state().clone();

            let PlayResult::Playing(game1) =
                game0.play(dealer0, valid_card("8H")).expect("valid play")
            else {
                panic!("unexpected state")
            };

            let dealer1 = game1.dealer();
            let pone1 = game1.pone();
            let play_state1 = game1.play_state();

            let last_play = Play::new(dealer0, valid_card("8H"));
            let mut expected_current_plays = play_state0.current_plays().clone();
            expected_current_plays.push(last_play);

            assert_eq!(dealer1, dealer0);
            assert_eq!(pone1, pone0);
            assert_eq!(play_state1.next_to_play(), pone0);
            assert_eq!(play_state1.previous_plays(), expected_current_plays);
            assert!(play_state1.current_plays().is_empty());
            assert_eq!(play_state1.pass_count(), 0);
        }

        #[test]
        fn score_play_points_for_fifteens() {
            let game0 = GameBuilder::default()
                .with_peggings(0, 0)
                .with_cut("AS")
                .with_hands("KH", "8D")
                .with_current_plays(&[(0, "7D")])
                .into_playing(1);
            let scores0 = game0.scores().clone();
            let dealer0 = game0.dealer();
            let pone0 = game0.pone();

            let PlayResult::Playing(game1) =
                game0.play(pone0, valid_card("8D")).expect("valid play")
            else {
                panic!("unexpected state")
            };

            let scores1 = game1.scores();
            let dealer1 = game1.dealer();
            let pone1 = game1.pone();

            assert_eq!(dealer1, dealer0);
            assert_eq!(pone1, pone0);
            assert_eq!(scores1.peggings()[&dealer1], scores0.peggings()[&dealer0]);
            assert_eq!(
                scores1.peggings()[&pone0],
                scores0.peggings()[&pone0].add(2.into())
            );
        }

        #[test]
        fn score_play_points_for_pair() {
            let game0 = GameBuilder::default()
                .with_peggings(0, 0)
                .with_cut("AS")
                .with_hands("KH", "8D")
                .with_current_plays(&[(0, "8S")])
                .into_playing(1);
            let scores0 = game0.scores().clone();
            let dealer0 = game0.dealer();
            let pone0 = game0.pone();

            let PlayResult::Playing(game1) =
                game0.play(pone0, valid_card("8D")).expect("valid play")
            else {
                panic!("unexpected state")
            };

            let scores1 = game1.scores();
            let dealer1 = game1.dealer();
            let pone1 = game1.pone();

            assert_eq!(dealer1, dealer0);
            assert_eq!(pone1, pone0);
            assert_eq!(scores1.peggings()[&dealer1], scores0.peggings()[&dealer0]);
            assert_eq!(
                scores1.peggings()[&pone0],
                scores0.peggings()[&pone0].add(2.into())
            );
        }

        #[test]
        fn score_play_points_for_triplet() {
            let game0 = GameBuilder::default()
                .with_peggings(0, 0)
                .with_cut("AS")
                .with_hands("KH", "8DAH")
                .with_current_plays(&[(1, "8C"), (0, "8S")])
                .into_playing(1);
            let scores0 = game0.scores().clone();
            let dealer0 = game0.dealer();
            let pone0 = game0.pone();

            let PlayResult::Playing(game1) =
                game0.play(pone0, valid_card("8D")).expect("valid play")
            else {
                panic!("unexpected state")
            };

            let scores1 = game1.scores();
            let dealer1 = game1.dealer();
            let pone1 = game1.pone();

            assert_eq!(dealer1, dealer0);
            assert_eq!(pone1, pone0);
            assert_eq!(scores1.peggings()[&dealer1], scores0.peggings()[&dealer0]);
            assert_eq!(
                scores1.peggings()[&pone0],
                scores0.peggings()[&pone0].add(6.into())
            );
        }

        #[test]
        fn score_play_points_for_quartet() {
            let game0 = GameBuilder::default()
                .with_peggings(0, 0)
                .with_cut("AS")
                .with_hands("KH", "7DAH")
                .with_current_plays(&[(1, "7C"), (0, "7S"), (0, "7H")])
                .into_playing(1);
            let scores0 = game0.scores().clone();
            let dealer0 = game0.dealer();
            let pone0 = game0.pone();

            let PlayResult::Playing(game1) =
                game0.play(pone0, valid_card("7D")).expect("valid play")
            else {
                panic!("unexpected state")
            };

            let scores1 = game1.scores();
            let dealer1 = game1.dealer();
            let pone1 = game1.pone();

            assert_eq!(dealer1, dealer0);
            assert_eq!(pone1, pone0);
            assert_eq!(scores1.peggings()[&dealer1], scores0.peggings()[&dealer0]);
            assert_eq!(
                scores1.peggings()[&pone0],
                scores0.peggings()[&pone0].add(12.into())
            );
        }

        #[test]
        fn score_play_points_for_run() {
            let game0 = GameBuilder::default()
                .with_peggings(0, 0)
                .with_cut("AS")
                .with_hands("KH", "AS")
                .with_current_plays(&[(1, "2D"), (0, "3H")])
                .into_playing(1);
            let scores0 = game0.scores().clone();
            let dealer0 = game0.dealer();
            let pone0 = game0.pone();

            let PlayResult::Playing(game1) =
                game0.play(pone0, valid_card("AS")).expect("valid play")
            else {
                panic!("unexpected state")
            };

            let scores1 = game1.scores();
            let dealer1 = game1.dealer();
            let pone1 = game1.pone();

            assert_eq!(dealer1, dealer0);
            assert_eq!(pone1, pone0);
            assert_eq!(scores1.peggings()[&dealer1], scores0.peggings()[&dealer0]);
            assert_eq!(
                scores1.peggings()[&pone0],
                scores0.peggings()[&pone0].add(3.into())
            );
        }

        #[test]
        fn score_play_points_for_run_edge_case_1() {
            let game0 = GameBuilder::default()
                .with_peggings(0, 0)
                .with_cut("AS")
                .with_hands("5H7H6H", "AH8S7S")
                .into_playing(1);
            let scores0 = game0.scores().clone();
            let dealer0 = game0.dealer();
            let pone0 = game0.pone();

            let PlayResult::Playing(game1) =
                game0.play(pone0, valid_card("8S")).expect("valid play")
            else {
                panic!("unexpected state")
            };

            let PlayResult::Playing(game1) =
                game1.play(dealer0, valid_card("7H")).expect("valid play")
            else {
                panic!("unexpected state")
            };

            let PlayResult::Playing(game1) =
                game1.play(pone0, valid_card("7S")).expect("valid play")
            else {
                panic!("unexpected state")
            };

            let PlayResult::Playing(game1) =
                game1.play(dealer0, valid_card("6H")).expect("valid play")
            else {
                panic!("unexpected state")
            };

            let scores1 = game1.scores();
            let dealer1 = game1.dealer();
            let pone1 = game1.pone();

            assert_eq!(dealer1, dealer0);
            assert_eq!(pone1, pone0);
            assert_eq!(
                scores1.peggings()[&dealer1],
                scores0.peggings()[&dealer0].add(2.into())
            );
            assert_eq!(
                scores1.peggings()[&pone1],
                scores0.peggings()[&pone0].add(2.into())
            );
        }

        #[test]
        fn score_play_points_for_run_edge_case_2() {
            let game0 = GameBuilder::default()
                .with_peggings(0, 0)
                .with_cut("AS")
                .with_hands("5H7H6H", "AH9S8S")
                .into_playing(1);
            let scores0 = game0.scores().clone();
            let dealer0 = game0.dealer();
            let pone0 = game0.pone();

            let PlayResult::Playing(game1) =
                game0.play(pone0, valid_card("9S")).expect("valid play")
            else {
                panic!("unexpected state")
            };

            let PlayResult::Playing(game1) =
                game1.play(dealer0, valid_card("6H")).expect("valid play")
            else {
                panic!("unexpected state")
            };

            let PlayResult::Playing(game1) =
                game1.play(pone0, valid_card("8S")).expect("valid play")
            else {
                panic!("unexpected state")
            };

            let PlayResult::Playing(game1) =
                game1.play(dealer0, valid_card("7H")).expect("valid play")
            else {
                panic!("unexpected state")
            };

            let scores1 = game1.scores();
            let dealer1 = game1.dealer();
            let pone1 = game1.pone();

            assert_eq!(dealer1, dealer0);
            assert_eq!(pone1, pone0);
            assert_eq!(
                scores1.peggings()[&dealer1],
                scores0.peggings()[&dealer0].add(2.into()).add(4.into())
            );
            assert_eq!(scores1.peggings()[&pone1], scores0.peggings()[&pone0]);
        }
    }

    /// ## The Go
    ///
    /// During play, the running total of cards may never be carried beyond 31. If a player cannot
    /// add another card without exceeding 31, he or she says "Go" and the opponent pegs 1. After
    /// gaining the Go, the opponent must first lay down any additional cards he can without
    /// exceeding 31. Besides the point for Go, he may then score any additional points that can be
    /// made through pairs and runs (described later). If a player reaches exactly 31, he pegs two
    /// instead of one for Go.
    ///
    /// The player who called Go leads for the next series of plays, with the count starting at
    /// zero. The lead may not be combined with any cards previously played to form a scoring
    /// combination; the Go has interrupted the sequence.
    ///
    /// The person who plays the last card pegs one for Go, plus one extra if the card brings the
    /// count to exactly 31. The dealer is sure to peg at least one point in every hand, for he will
    /// have a Go on the last card if not earlier.
    mod the_go {
        use super::*;

        #[test]
        fn accept_pass_when_pone_has_no_valid_card() {
            let game0 = GameBuilder::default()
                .with_peggings(0, 0)
                .with_cut("AS")
                .with_hands("AH", "KH")
                .with_current_plays(&[(0, "TH"), (0, "JH"), (0, "QH")])
                .into_playing(1);
            let scores0 = game0.scores().clone();
            let dealer0 = game0.dealer();
            let pone0 = game0.pone();
            let hands0 = game0.hands().clone();
            let play_state0 = game0.play_state().clone();

            let PassResult::Playing(game1) = game0.pass(pone0).expect("valid pass") else {
                panic!("unexpected result")
            };

            let scores1 = game1.scores();
            let dealer1 = game1.dealer();
            let hands1 = game1.hands();
            let play_state1 = game1.play_state();

            assert_eq!(scores1, &scores0);
            assert_eq!(dealer1, dealer0);
            assert_eq!(hands1, &hands0);
            assert_eq!(play_state1.next_to_play(), dealer0);
            assert_eq!(play_state1.pass_count(), 1);
            assert_eq!(play_state1.current_plays(), play_state0.current_plays());
            assert_eq!(play_state1.previous_plays(), play_state0.previous_plays());
        }

        #[test]
        fn accept_pass_when_dealer_has_no_valid_card() {
            let game0 = GameBuilder::default()
                .with_peggings(0, 0)
                .with_cut("AS")
                .with_hands("KH", "KS")
                .with_current_plays(&[(0, "TH"), (0, "JH"), (1, "QH")])
                .into_playing(1);
            let scores0 = game0.scores().clone();
            let dealer0 = game0.dealer();
            let pone0 = game0.pone();
            let hands0 = game0.hands().clone();
            let play_state0 = game0.play_state().clone();

            let PassResult::Playing(game1) = game0.pass(pone0).expect("valid pass") else {
                panic!("unexpected state")
            };

            let PassResult::Playing(game1) = game1.pass(dealer0).expect("valid pass") else {
                panic!("unexpected state")
            };

            let scores1 = game1.scores();
            let dealer1 = game1.dealer();
            let hands1 = game1.hands();
            let play_state1 = game1.play_state();

            assert_eq!(scores1.peggings()[&pone0], scores0.peggings()[&pone0]);
            assert_eq!(
                scores1.peggings()[&dealer1],
                scores0.peggings()[&dealer0].add(1.into())
            );
            assert_eq!(dealer1, dealer0);
            assert_eq!(hands1, &hands0);
            assert_eq!(play_state1.next_to_play(), pone0);
            assert_eq!(play_state1.pass_count(), 0);
            assert!(play_state1.current_plays().is_empty());
            for p in play_state0.current_plays().into_iter() {
                assert!(play_state1.previous_plays().contains(&p))
            }
        }

        #[test]
        fn cannot_pass_when_player_not_participating() {
            let game0 = GameBuilder::default()
                .with_peggings(0, 0)
                .with_cut("AS")
                .with_hands("KH", "KS")
                .with_current_plays(&[(0, "TH"), (0, "JH"), (1, "QH")])
                .into_playing(1);

            let non_player = Player::new();
            let error = game0.pass(non_player).expect_err("invalid pass");
            assert_eq!(error, GameError::InvalidPlayer(non_player));
        }

        #[test]
        fn cannot_pass_when_valid_card_held() {
            let game0 = GameBuilder::default()
                .with_peggings(0, 0)
                .with_cut("AS")
                .with_hands("AH", "AS")
                .with_current_plays(&[(0, "TH"), (0, "JH"), (0, "8H")])
                .into_playing(1);
            let pone0 = game0.pone();

            let error = game0.pass(pone0).expect_err("invalid pass");
            assert_eq!(error, GameError::CannotPass);
        }

        #[test]
        fn score_pass_when_both_players_passed_playing() {
            let game0 = GameBuilder::default()
                .with_peggings(0, 0)
                .with_cut("AS")
                .with_hands("KH", "KS")
                .with_current_plays(&[(0, "TH"), (0, "JH"), (1, "QH")])
                .into_playing(1);
            let scores0 = game0.scores().clone();
            let dealer0 = game0.dealer();
            let pone0 = game0.pone();

            let PassResult::Playing(game1) = game0.pass(pone0).expect("valid pass") else {
                panic!("unexpected state")
            };

            let PassResult::Playing(game1) = game1.pass(dealer0).expect("valid pass") else {
                panic!("unexpectd state")
            };

            let scores1 = game1.scores();
            let dealer1 = game1.dealer();
            let pone1 = game1.pone();

            assert_eq!(scores1.peggings()[&pone1], scores0.peggings()[&pone0]);
            assert_eq!(
                scores1.peggings()[&dealer1],
                scores0.peggings()[&dealer0].add(1.into())
            );
        }

        #[test]
        fn score_pass_when_both_players_passed_finished() {
            let game0 = GameBuilder::default()
                .with_peggings(120, 0)
                .with_cut("AS")
                .with_hands("KH", "KS")
                .with_current_plays(&[(0, "TH"), (0, "JH"), (1, "QH")])
                .into_playing(1);
            let scores0 = game0.scores().clone();
            let dealer0 = game0.dealer();
            let pone0 = game0.pone();

            let PassResult::Playing(game1) = game0.pass(pone0).expect("valid pass") else {
                panic!("unexpected state")
            };

            let PassResult::Finished(game1) = game1.pass(dealer0).expect("valid pass") else {
                panic!("unexpected state")
            };

            let winner1 = game1.winner();
            let peggings1 = game1.peggings();

            assert_eq!(winner1, dealer0);
            assert_eq!(peggings1[&pone0], scores0.peggings()[&pone0]);
            assert_eq!(
                peggings1[&dealer0],
                scores0.peggings()[&dealer0].add(1.into())
            );
        }

        #[test]
        fn swap_player_after_pone_pass() {
            let game0 = GameBuilder::default()
                .with_peggings(0, 0)
                .with_cut("AS")
                .with_hands("8H8D", "5SJH")
                .with_current_plays(&[(1, "4S"), (0, "9C"), (1, "TH"), (0, "7H")])
                .into_playing(1);
            let dealer0 = game0.dealer();
            let pone0 = game0.pone();

            let PassResult::Playing(game1) = game0.pass(pone0).expect("valid pass") else {
                panic!("unexpected event")
            };

            let dealer1 = game1.dealer();
            let pone1 = game1.pone();
            let play_state1 = game1.play_state();

            assert_eq!(dealer1, dealer0);
            assert_eq!(pone1, pone0);
            assert_eq!(play_state1.next_to_play(), dealer1);
        }

        #[test]
        fn swap_player_after_dealer_pass() {
            let game0 = GameBuilder::default()
                .with_peggings(0, 0)
                .with_cut("AS")
                .with_hands("7H8H8D", "4S5S")
                .with_current_plays(&[(1, "JH"), (0, "9C"), (1, "TH")])
                .into_playing(0);
            let dealer0 = game0.dealer();
            let pone0 = game0.pone();

            let PassResult::Playing(game1) = game0.pass(dealer0).expect("valid pass") else {
                panic!("unexpected state")
            };

            let dealer1 = game1.dealer();
            let pone1 = game1.pone();
            let play_state1 = game1.play_state();

            assert_eq!(dealer1, dealer0);
            assert_eq!(pone1, pone0);
            assert_eq!(play_state1.next_to_play(), pone0);
        }

        #[test]
        fn reset_play_after_pone_then_dealer_pass() {
            let game0 = GameBuilder::default()
                .with_peggings(0, 0)
                .with_cut("AS")
                .with_hands("8H8D", "5SJH")
                .with_current_plays(&[(1, "4S"), (0, "9C"), (1, "TH"), (0, "7H")])
                .into_playing(1);
            let dealer0 = game0.dealer();
            let pone0 = game0.pone();
            let play_state0 = game0.play_state().clone();

            let PassResult::Playing(game1) = game0.pass(pone0).expect("valid pass") else {
                panic!("unexpected state")
            };

            let PassResult::Playing(game1) = game1.pass(dealer0).expect("valid pass") else {
                panic!("unexpected state")
            };

            let dealer1 = game1.dealer();
            let pone1 = game1.pone();
            let play_state1 = game1.play_state();

            assert_eq!(dealer1, dealer0);
            assert_eq!(pone1, pone0);
            assert_eq!(play_state1.next_to_play(), pone1);
            assert_eq!(play_state1.previous_plays(), play_state0.current_plays());
            assert!(play_state1.current_plays().is_empty());
            assert_eq!(play_state1.pass_count(), 0);
        }

        #[test]
        fn reset_play_after_after_dealer_then_pone_pass() {
            let game0 = GameBuilder::default()
                .with_peggings(0, 0)
                .with_cut("AS")
                .with_hands("7H8H8D", "4S5S")
                .with_current_plays(&[(1, "JH"), (0, "9C"), (1, "TH")])
                .into_playing(0);
            let dealer0 = game0.dealer();
            let pone0 = game0.pone();
            let play_state0 = game0.play_state().clone();

            let PassResult::Playing(game1) = game0.pass(dealer0).expect("valid pass") else {
                panic!("unexpected state")
            };

            let PassResult::Playing(game1) = game1.pass(pone0).expect("valid pass") else {
                panic!("unexpected state")
            };

            let dealer1 = game1.dealer();
            let pone1 = game1.pone();
            let play_state1 = game1.play_state();

            assert_eq!(dealer1, dealer0);
            assert_eq!(pone1, pone0);
            assert_eq!(play_state1.next_to_play(), dealer1);
            assert_eq!(play_state1.previous_plays(), play_state0.current_plays());
            assert!(play_state1.current_plays().is_empty());
            assert_eq!(play_state1.pass_count(), 0);
        }
    }

    /// ## Pegging
    ///
    /// The object in play is to score points by pegging. In addition to a Go, a player may score
    /// for the following combinations:
    ///
    ///   - Fifteen: For adding a card that makes the total 15 Peg 2
    ///   - Pair: For adding a card of the same rank as the card just played Peg 2 (Note that face
    ///     cards pair only by actual rank: jack with jack, but not jack with queen.)
    ///   - Triplet: For adding the third card of the same rank. Peg 6
    ///   - Four: (also called "Double Pair" or "Double Pair Royal") For adding the fourth card of
    ///     the same rank Peg 12
    ///   - Run (Sequence): For adding a card that forms, with those just played:
    ///     - For a sequence of three Peg 3
    ///     - For a sequence of four. Peg 4
    ///     - For a sequence of five. Peg 5
    ///     - (Peg one point more for each extra card of a sequence. Note that runs are independent
    ///       of suits, but go strictly by rank; to illustrate: 9, 10, J, or J, 9, 10 is a run but
    ///       9, 10, Q is not)
    ///
    /// It is important to keep track of the order in which cards are played to determine whether
    /// what looks like a sequence or a run has been interrupted by a "foreign card." Example:
    /// Cards are played in this order: 8, 7, 7, 6. The dealer pegs 2 for 15, and the opponent
    /// pegs 2 for pair, but the dealer cannot peg for run because of the extra seven (foreign
    /// card) that has been played. Example: Cards are played in this order: 9, 6, 8, 7. The
    /// dealer pegs 2 for fifteen when he plays the six and pegs 4 for run when he plays the seven
    /// (the 6, 7, 8, 9 sequence). The cards were not played in sequential order, but they form a
    /// true run with no foreign card.
    mod pegging {
        use super::*;

        #[test]
        fn should_score_fifteens() {
            let game = GameBuilder::default()
                .with_peggings(0, 0)
                .with_hands("", "")
                .with_current_plays(&[(0, "JD"), (0, "5H")])
                .with_cut("AH")
                .into_playing(1);
            let play_state = game.play_state();
            assert_eq!(
                CurrentPlayScorer::new(play_state).score().points(),
                2.into()
            )
        }

        #[test]
        fn should_score_pairs() {
            let game = GameBuilder::default()
                .with_peggings(0, 0)
                .with_hands("", "")
                .with_current_plays(&[(0, "JD"), (0, "AH"), (0, "AS")])
                .with_cut("KH")
                .into_playing(1);
            let play_state = game.play_state();
            assert_eq!(
                CurrentPlayScorer::new(play_state).score().points(),
                2.into()
            )
        }

        #[test]
        fn should_score_royal_pairs() {
            let game = GameBuilder::default()
                .with_peggings(0, 0)
                .with_hands("", "")
                .with_current_plays(&[(0, "AD"), (0, "AH"), (0, "AS")])
                .with_cut("KH")
                .into_playing(1);
            let play_state = game.play_state();
            assert_eq!(
                CurrentPlayScorer::new(play_state).score().points(),
                6.into()
            )
        }

        #[test]
        fn should_score_double_royal_pairs() {
            let game = GameBuilder::default()
                .with_peggings(0, 0)
                .with_hands("", "")
                .with_current_plays(&[(0, "AC"), (0, "AD"), (0, "AH"), (0, "AS")])
                .with_cut("KH")
                .into_playing(1);
            let play_state = game.play_state();
            assert_eq!(
                CurrentPlayScorer::new(play_state).score().points(),
                12.into()
            )
        }

        #[test]
        fn should_score_runs() {
            let current_plays = &[
                (0, "2C"),
                (0, "3C"),
                (0, "4C"),
                (0, "5C"),
                (0, "6C"),
                (0, "7C"),
            ];
            for len in 1..=current_plays.len() {
                let current_plays = *current_plays;
                let current_plays = current_plays.into_iter().take(len);
                let current_plays = Vec::from_iter(current_plays);
                let game = GameBuilder::default()
                    .with_peggings(0, 0)
                    .with_hands("KS", "KD")
                    .with_current_plays(&current_plays)
                    .with_cut("KH")
                    .into_playing(1);
                let play_state = game.play_state();
                assert_eq!(
                    CurrentPlayScorer::new(play_state).score().points(),
                    (if len < 3 { 0 } else { len }).into()
                )
            }
        }

        #[test]
        fn should_score_runs_unordered() {
            let game = GameBuilder::default()
                .with_peggings(0, 0)
                .with_hands("KS", "KD")
                .with_current_plays(&[(0, "3S"), (0, "2C"), (0, "AS")])
                .with_cut("KH")
                .into_playing(1);
            let play_state = game.play_state();
            assert_eq!(
                CurrentPlayScorer::new(play_state).score().points(),
                3.into()
            )
        }

        #[test]
        fn should_score_rules_example_flush() {
            let game0 = GameBuilder::default()
                .with_peggings(0, 0)
                .with_hands("AH", "KD")
                .with_cut("2H")
                .with_current_plays(&[(1, "TH"), (0, "9H"), (1, "QH")])
                .into_playing(0);
            let dealer0 = game0.dealer();

            let PlayResult::Playing(game1) =
                game0.play(dealer0, valid_card("AH")).expect("valid play")
            else {
                panic!("unexpected state")
            };

            let play_state1 = game1.play_state();
            assert_eq!(
                CurrentPlayScorer::new(play_state1).score().points(),
                0.into()
            );
        }

        #[test]
        fn should_score_when_target_not_reached() {
            let game = GameBuilder::default()
                .with_peggings(0, 0)
                .with_hands("", "")
                .with_current_plays(&[(0, "AC"), (0, "AD"), (0, "AH"), (0, "AS")])
                .with_cut("KH")
                .into_playing(1);
            let play_state = game.play_state();
            assert_eq!(EndOfPlayScorer::new(play_state).score().points(), 1.into());
        }

        #[test]
        fn should_score_when_target_reached() {
            let game = GameBuilder::default()
                .with_peggings(0, 0)
                .with_hands("", "")
                .with_current_plays(&[(0, "KC"), (0, "KD"), (0, "KH"), (0, "AS")])
                .with_cut("KS")
                .into_playing(1);
            let play_state = game.play_state();
            assert_eq!(EndOfPlayScorer::new(play_state).score().points(), 2.into())
        }
    }

    /// ## Counting the Hands
    ///
    /// When play ends, the three hands are counted in order: non-dealer's hand (first), dealer's
    /// hand (second), and then the crib (third). This order is important because, toward the end of
    /// a game, the non-dealer may "count out" and win before the dealer has a chance to count, even
    /// though the dealer's total would have exceeded that of the opponent. The starter is
    /// considered to be a part of each hand, so that all hands in counting comprise five cards. The
    /// basic scoring formations are as follows:
    ///
    /// Combinations counts
    ///   - Fifteen. Each combination of cards that totals 15 2
    ///   - Pair. Each pair of cards of the same rank 2
    ///   - Run. Each combination of three or more 1 cards in sequence (for each card in the
    ///     sequence)
    ///   - Flush.
    ///     - Four cards of the same suit in hand 4 (excluding the crib, and the starter)
    ///     - Four cards in hand or crib of the same 5 suit as the starter. (There is no count for
    ///       four-flush in the crib that is not of same suit as the starter)
    ///   - His Nobs. Jack of the same suit as starter in hand or crib 1
    mod counting_the_hands {
        use super::*;

        #[test]
        fn score_pone_hand_when_plays_finished() {
            let game0 = GameBuilder::default()
                .with_peggings(0, 0)
                .with_hands("", "TH")
                .with_cut("4H")
                .with_previous_plays(&[
                    (0, "7H"),
                    (0, "8C"),
                    (0, "AC"),
                    (0, "2C"),
                    (1, "JH"),
                    (1, "KS"),
                    (1, "5H"),
                ])
                .into_playing(1);
            let scores0 = game0.scores().clone();
            let dealer0 = game0.dealer();
            let pone0 = game0.pone();

            let PlayResult::Scoring(game1) =
                game0.play(pone0, valid_card("TH")).expect("valid play")
            else {
                panic!("Unexpected state")
            };

            let scores1 = game1.scores();
            let dealer1 = game1.dealer();
            let pone1 = game1.pone();

            assert_eq!(dealer1, dealer0);
            assert_eq!(pone1, pone0);
            assert_eq!(scores1.peggings()[&dealer1], scores0.peggings()[&dealer0]);
            assert_eq!(
                scores1.peggings()[&pone1],
                scores0.peggings()[&pone0].add(1.into())
            );
        }

        #[test]
        fn score_winning_pone_when_plays_finished() {
            let game0 = GameBuilder::default()
                .with_peggings(0, 115)
                .with_hands("", "TH")
                .with_cut("4H")
                .with_previous_plays(&[
                    (0, "7H"),
                    (0, "8C"),
                    (0, "AC"),
                    (0, "2C"),
                    (1, "JH"),
                    (1, "KS"),
                    (1, "5H"),
                ])
                .into_playing(1);

            let dealer0 = game0.dealer();
            let pone0 = game0.pone();

            let PlayResult::Scoring(game1) =
                game0.play(pone0, valid_card("TH")).expect("valid play")
            else {
                panic!("unexpected state")
            };

            let scores1 = game1.scores().clone();

            let ScorePoneResult::Finished(game2) = game1.score_hand().expect("valid score_hand")
            else {
                panic!("unexpected state")
            };

            let winner2 = game2.winner();
            let peggings2 = game2.peggings();

            assert_eq!(winner2, pone0);
            assert_eq!(peggings2[&dealer0], scores1.peggings()[&dealer0]);
            assert_eq!(peggings2[&pone0], scores1.peggings()[&pone0].add(7.into()));
        }

        #[test]
        fn score_dealer_after_pone_scored() {
            let game0 = GameBuilder::default()
                .with_peggings(0, 0)
                .with_cut("4H")
                .with_hands("7H8CAC2C", "JCKS5HTH")
                .into_scoring_pone();
            let dealer0 = game0.dealer();
            let pone0 = game0.pone();

            let ScorePoneResult::Scoring(game1) = game0.score_hand().expect("valid score_hand")
            else {
                panic!("unexpected state")
            };

            let scores1 = game1.scores().clone();

            let ScoreDealerResult::Scoring(game2) = game1.score_hand().expect("valid score_hand")
            else {
                panic!("unexpected state")
            };

            let scores2 = game2.scores();
            let dealer2 = game2.dealer();
            let pone2 = game2.pone();

            assert_eq!(dealer0, dealer2);
            assert_eq!(pone0, pone2);
            assert_eq!(
                scores2.peggings()[&dealer2],
                scores1.peggings()[&dealer0].add(4.into())
            );
            assert_eq!(scores2.peggings()[&pone2], scores1.peggings()[&pone0]);
        }

        #[test]
        fn score_winning_dealer_after_pone_scored() {
            let game0 = GameBuilder::default()
                .with_peggings(117, 0)
                .with_cut("4H")
                .with_hands("7H8CAC2C", "JCKS5HTH")
                .into_scoring_pone();
            let dealer0 = game0.dealer();
            let pone0 = game0.pone();

            let ScorePoneResult::Scoring(game1) = game0.score_hand().expect("valid score_hand")
            else {
                panic!("unexpected state")
            };

            let scores1 = game1.scores().clone();

            let ScoreDealerResult::Finished(game2) = game1.score_hand().expect("valid score_hand")
            else {
                panic!("unexpected state")
            };

            let winner2 = game2.winner();
            let peggings2 = game2.peggings();

            assert_eq!(winner2, dealer0);
            assert_eq!(
                peggings2[&winner2],
                scores1.peggings()[&dealer0].add(4.into())
            );
            assert_eq!(peggings2[&pone0], scores1.peggings()[&pone0]);
        }

        #[test]
        fn score_crib_after_dealer_scored() {
            let game0 = GameBuilder::default()
                .with_peggings(0, 0)
                .with_cut("4H")
                .with_hands("7H8CAC2C", "JCKS5HTH")
                .with_crib("AHADASTD")
                .into_scoring_dealer();
            let dealer0 = game0.dealer();
            let pone0 = game0.pone();

            let ScoreDealerResult::Scoring(game1) = game0.score_hand().expect("valid score_hand")
            else {
                panic!("unexpected state")
            };

            let scores1 = game1.scores().clone();

            let ScoreCribResult::Discarding(game2) = game1.score_crib().expect("valid score_crib")
            else {
                panic!("unexpected state")
            };

            let scores2 = game2.scores();
            let dealer2 = game2.dealer();
            let pone2 = game2.pone();

            assert_eq!(pone2, dealer0);
            assert_eq!(dealer2, pone0);
            assert_eq!(
                scores2.peggings()[&pone2],
                scores1.peggings()[&dealer0].add(12.into())
            );
            assert_eq!(scores2.peggings()[&dealer2], scores1.peggings()[&pone0]);
        }

        #[test]
        fn redeal_after_crib_scored() {
            let game0 = GameBuilder::default()
                .with_peggings(0, 0)
                .with_cut("4H")
                .with_hands("7H8CAC2C", "JCKS5HTH")
                .with_crib("AHADASTD")
                .into_scoring_crib();
            let dealer0 = game0.dealer();
            let pone0 = game0.pone();

            let ScoreCribResult::Discarding(game1) = game0.score_crib().expect("valid score_crib")
            else {
                panic!("unexpected state")
            };

            let dealer1 = game1.dealer();
            let pone1 = game1.pone();
            let hands1 = game1.hands();
            let crib1 = game1.crib();
            let deck1 = game1.deck();

            assert_eq!(dealer1, pone0);
            assert_eq!(pone1, dealer0);
            assert_eq!(hands1[&dealer1].len(), 6);
            assert_eq!(hands1[&pone1].len(), 6);
            assert!(crib1.is_empty());
            assert_eq!(deck1.len(), 40);
        }

        #[test]
        fn score_winning_crib_after_dealer_scored() {
            let game0 = GameBuilder::default()
                .with_peggings(110, 0)
                .with_cut("4H")
                .with_hands("7H8CAC2C", "JCKS5HTH")
                .with_crib("AHADASTD")
                .into_scoring_dealer();
            let dealer0 = game0.dealer();
            let pone0 = game0.pone();

            let ScoreDealerResult::Scoring(game1) = game0.score_hand().expect("valid score_hand")
            else {
                panic!("unexpected state")
            };

            let scores1 = game1.scores().clone();

            let ScoreCribResult::Finished(game2) = game1.score_crib().expect("valid score_crib")
            else {
                panic!("unexpected state")
            };

            let winner2 = game2.winner();
            let peggings2 = game2.peggings();

            assert_eq!(winner2, dealer0);
            assert_eq!(
                peggings2[&winner2],
                scores1.peggings()[&dealer0].add(12.into())
            );
            assert_eq!(peggings2[&pone0], scores1.peggings()[&pone0]);
        }

        #[test]
        fn hand_should_score_fifteens() {
            assert_eq!(
                *HandScorer::new(&valid_hand("7H8CAC2C"), valid_card("4H"))
                    .score()
                    .points(),
                4
            );
            assert_eq!(
                *HandScorer::new(&valid_hand("THJCKS5H"), valid_card("4H"))
                    .score()
                    .points(),
                6
            );
        }

        #[test]
        fn hand_should_score_pairs() {
            assert_eq!(
                *HandScorer::new(&valid_hand("2H4C5C2C"), valid_card("AH"))
                    .score()
                    .points(),
                2
            );
            assert_eq!(
                *HandScorer::new(&valid_hand("TCASADTH"), valid_card("AH"))
                    .score()
                    .points(),
                8
            );
        }

        #[test]
        fn hand_should_score_royal_pairs() {
            assert_eq!(
                *HandScorer::new(&valid_hand("2H2D5C2C"), valid_card("AH"))
                    .score()
                    .points(),
                6
            );
            assert_eq!(
                *HandScorer::new(&valid_hand("TCASADTH"), valid_card("AH"))
                    .score()
                    .points(),
                8
            );
        }

        #[test]
        fn hand_should_score_double_royal_pairs() {
            assert_eq!(
                *HandScorer::new(&valid_hand("2H2C2D2S"), valid_card("AH"))
                    .score()
                    .points(),
                12
            );
            assert_eq!(
                *HandScorer::new(&valid_hand("TCASADTH"), valid_card("AH"))
                    .score()
                    .points(),
                8
            );
        }

        #[test]
        fn hand_should_score_runs() {
            assert_eq!(
                *HandScorer::new(&valid_hand("JDQCKC2C"), valid_card("AH"))
                    .score()
                    .points(),
                3
            );
            assert_eq!(
                *HandScorer::new(&valid_hand("3C3S2D5H"), valid_card("AH"))
                    .score()
                    .points(),
                8
            );
        }

        #[test]
        fn hand_should_score_flushes() {
            assert_eq!(
                *HandScorer::new(&valid_hand("2H4H6H8H"), valid_card("TH"))
                    .score()
                    .points(),
                5
            );
            assert_eq!(
                *HandScorer::new(&valid_hand("2D4D6D8D"), valid_card("TH"))
                    .score()
                    .points(),
                4
            );
        }

        #[test]
        fn hand_should_score_his_heels() {
            assert_eq!(
                *HandScorer::new(&valid_hand("2D4H6HJH"), valid_card("TH"))
                    .score()
                    .points(),
                1
            );
            assert_eq!(
                *HandScorer::new(&valid_hand("2H4D6DJD"), valid_card("TH"))
                    .score()
                    .points(),
                0
            );
        }

        #[test]
        fn crib_should_score_fifteens() {
            assert_eq!(
                *CribScorer::new(&valid_crib("7H8CAC2C"), valid_card("4H"))
                    .score()
                    .points(),
                4
            );
            assert_eq!(
                *CribScorer::new(&valid_crib("THJCKS5H"), valid_card("4H"))
                    .score()
                    .points(),
                6
            );
        }

        #[test]
        fn crib_should_score_pairs() {
            assert_eq!(
                *CribScorer::new(&valid_crib("2H4C5C2C"), valid_card("AH"))
                    .score()
                    .points(),
                2
            );
            assert_eq!(
                *CribScorer::new(&valid_crib("TCASADTH"), valid_card("AH"))
                    .score()
                    .points(),
                8
            );
        }

        #[test]
        fn crib_should_score_royal_pairs() {
            assert_eq!(
                *CribScorer::new(&valid_crib("2H2D5C2C"), valid_card("AH"))
                    .score()
                    .points(),
                6
            );
            assert_eq!(
                *CribScorer::new(&valid_crib("TCASADTH"), valid_card("AH"))
                    .score()
                    .points(),
                8
            );
        }

        #[test]
        fn crib_should_score_double_royal_pairs() {
            assert_eq!(
                *CribScorer::new(&valid_crib("2H2C2D2S"), valid_card("AH"))
                    .score()
                    .points(),
                12
            );
            assert_eq!(
                *CribScorer::new(&valid_crib("TCASADTH"), valid_card("AH"))
                    .score()
                    .points(),
                8
            );
        }

        #[test]
        fn crib_should_score_runs() {
            assert_eq!(
                *CribScorer::new(&valid_crib("JDQCKC2C"), valid_card("AH"))
                    .score()
                    .points(),
                3
            );
            assert_eq!(
                *CribScorer::new(&valid_crib("3C3S2D5H"), valid_card("AH"))
                    .score()
                    .points(),
                8
            );
        }

        #[test]
        fn crib_should_score_flushes() {
            assert_eq!(
                *CribScorer::new(&valid_crib("2H4H6H8H"), valid_card("TH"))
                    .score()
                    .points(),
                5
            );
            assert_eq!(
                *CribScorer::new(&valid_crib("2D4D6D8D"), valid_card("TH"))
                    .score()
                    .points(),
                0
            );
        }

        #[test]
        fn crib_should_score_his_heels() {
            assert_eq!(
                *CribScorer::new(&valid_crib("2D4H6HJH"), valid_card("TH"))
                    .score()
                    .points(),
                1
            );
            assert_eq!(
                *CribScorer::new(&valid_crib("2H4D6DJD"), valid_card("TH"))
                    .score()
                    .points(),
                0
            );
        }
    }

    /// ### Combinations
    ///
    /// In the above table, the word combination is used in the strict technical sense. Each and
    /// every combination of two cards that make a pair, of two or more cards that make 15, or of
    /// three or more cards that make a run, count separately.
    ///
    /// Example: A hand (including the starter) comprised of 8, 7, 7, 6, 2 scores 8 points for four
    /// combinations that total 15: the 8 with one 7, and the 8 with the other 7; the 6, 2 with each
    /// of the two 7s. The same hand also scores 2 for a pair, and 6 for two runs of three (8, 7, 6
    /// using each of the two 7s). The total score is 16. An experienced player computes the hand
    /// thus: "Fifteen 2, fifteen 4, fifteen 6, fifteen 8, and 8 for double run is 16."
    ///
    /// Note that the ace is always low and cannot form a sequence with a king. Further, a flush
    /// cannot happen during the play of the cards; it occurs only when the hands and the crib are
    /// counted.
    ///
    /// Certain basic formulations should be learned to facilitate counting. For pairs and runs
    /// alone:
    ///
    /// A. A triplet counts 6. A. Four of a kind counts 12. A. A run of three, with one card
    /// duplicated (double run) counts 8. A. A run of four, with one card duplicated, counts 10. A.
    /// A run of three, with one card triplicated (triple run), counts 15. A. A run of three, with
    /// two different cards duplicated, counts 16.
    mod combinations {
        use super::*;
        #[test]
        fn should_score_rules_example_eights_sevens_sixes() {
            assert_eq!(
                *HandScorer::new(&valid_hand("8H7C7D6S"), valid_card("2H"))
                    .score()
                    .points(),
                16
            );
        }

        #[test]
        fn should_score_rules_example_runs() {
            assert_eq!(
                *HandScorer::new(&valid_hand("JHQCKDAS"), valid_card("2D"))
                    .score()
                    .points(),
                3
            );
        }

        #[test]
        fn should_score_rules_example_flush() {
            assert_eq!(
                *HandScorer::new(&valid_hand("THQHKHAH"), valid_card("2H"))
                    .score()
                    .points(),
                5
            );
            assert_eq!(
                *HandScorer::new(&valid_hand("THQHKHAH"), valid_card("2S"))
                    .score()
                    .points(),
                4
            );
            assert_eq!(
                *HandScorer::new(&valid_hand("THQHKHAS"), valid_card("2H"))
                    .score()
                    .points(),
                0
            );
        }
    }

    /// ### A PERFECT 29!
    ///
    /// The highest possible score for combinations in a single Cribbage deal is 29, and it may
    /// occur only once in a Cribbage fan's lifetime -in fact, experts say that a 29 is probably as
    /// rare as a hole-in-one in golf. To make this amazing score, a player must have a five as the
    /// starter (upcard) and the other three fives plus the jack of the same suit as the starter -
    /// His Nobs: 1 point - in his hand. The double pair royal (four 5s) peg another 12 points; the
    /// various fives used to hit 15 can be done four ways for 8 points; and the jack plus a 5 to
    /// hit 15 can also be done four ways for 8 points. Total = 29 points.
    mod a_perfect_29 {
        use super::*;

        #[test]
        fn should_score_rules_example_perfect_29() {
            assert_eq!(
                *HandScorer::new(&valid_hand("5H5C5DJS"), valid_card("5S"))
                    .score()
                    .points(),
                29
            );
        }
    }

    /// ## Miscellaneous
    ///
    /// The following list includes many of the hands that may give the beginner some difficulty in
    /// counting. Note that no hand can make a count of 19, 25, 26, or 27. (In the chart below J
    /// stands for His Nobs, the jack of the same suit as the starter.
    ///
    /// ### Muggins (optional) - not implemented.
    ///
    /// Each player must count his hand (and crib) aloud and announce the total. If he overlooks any
    /// score, the opponent may say "Muggins" and then score the overlooked points for himself. For
    /// experienced players, the Muggins rule is always in effect and adds even more suspense to the
    /// game.
    mod miscellaneous {}

    /// ## Game
    ///
    /// Game may be fixed at either 121 points or 61 points. The play ends the moment either player
    /// reaches the agreed total, whether by pegging or counting one's hand. If the non-dealer "goes
    /// out" by the count of his hand, the game immediately ends and the dealer may not score either
    /// his hand or the crib.
    ///
    /// If a player wins the game before the loser has passed the halfway mark (did not reach 31 in
    /// a game of 61, or 61 in a game of 121), the loser is "lurched," and the winner scores two
    /// games instead of one. A popular variation of games played to 121, is a "skunk" (double game)
    /// for the winner if the losing player fails to pass the three-quarter mark - 91 points or more -
    /// and it is a "double skunk" (quadruple game) if the loser fails to pass the halfway mark (61
    /// or more points).
    mod game {}

    /// ## The Cribbage Board
    ///
    /// The Cribbage board (see illustration) has four rows of 30 holes each, divided into two pairs
    /// of rows by a central panel. There are usually four (or two) additional holes near one end,
    /// called "game holes." With the board come four pegs, usually in two contrasting colors. Note:
    /// There are also continuous track Cribbage boards available which, as the name implies, have
    /// one continuous line of 121 holes for each player.
    ///
    /// The board is placed to one side between the two players, and each player takes two pegs of
    /// the same color. (The pegs are placed in the game holes until the game begins.) Each time a
    /// player scores, he advances a peg along a row on his side of the board, counting one hole per
    /// point. Two pegs are used, and the rearmost peg jumps over the first peg to show the first
    /// increment in score. After another increase in score, the peg behind jumps over the peg in
    /// front to the appropriate hole to show the player's new score, and so on (see diagram next
    /// page). The custom is to "go down" (away from the game holes) on the outer rows and "come up"
    /// on the inner rows. A game of 61 is "once around" and a game of 121 is "twice around." As
    /// noted previously, continuous line Cribbage boards are available.
    ///
    /// If a Cribbage board is not available, each player may use a piece of paper or cardboard,
    /// marked thus:
    ///
    ///   - Units 1, 2, 3, 4, 5, 6, 7, 8, 9, 10
    ///   - Tens 10, 20, 30, 40, 50, 60
    ///
    /// Two small markers, such as small coins or buttons, can substitute for pegs for counting in
    /// each row.
    mod the_cribbage_board {}

    /// ## Strategy
    ///
    /// ### The Crib.
    ///
    /// If the dealer is discarding for the crib, he should “salt” it with the best possible cards,
    /// but at the same time retain good cards in his hand that can be used for high scoring.
    /// Conversely, for the non-dealer, it is best to lay out cards that will be the least
    /// advantageous for the dealer. Laying out a five would be the worst choice, for the dealer
    /// could use it to make 15 with any one of the ten-cards (10, J, Q, K). Laying out a pair is
    /// usually a poor choice too, and the same goes for sequential cards, such as putting both a
    /// six and seven in the crib. The ace and king tend to be good cards to put in the crib because
    /// it is harder to use them in a run.
    ///
    /// ### The Play
    ///
    /// As expected, the five makes for the worst lead in that there are so many ten-cards that the
    /// opponent can use to make a 15. Leading from a pair is a good idea, for even if the opponent
    /// makes a pair, the leader can play the other matching card from his hand and collect for a
    /// pair royal. Leading an ace or deuce is not a good idea, for these cards should be saved
    /// until later to help make a 15, a Go, or a 31. The safest lead is a four because this card
    /// cannot be used to make a 15 at the opponent’s very next turn. Finally, when the opponent
    /// leads a card that can either be paired or make 15, the latter choice is preferred.
    ///
    /// During the play, it is advisable not to try to make a count of 21, for the opponent can then
    /// play one of the many 10-cards and make 31 to gain two points.
    mod the_strategy {}

    /// ## Internal
    mod display {
        use super::*;

        fn common_filters() -> insta::Settings {
            let mut settings = insta::Settings::new();
            settings.add_filter(r"[0-9a-f]{8}", "<playerid>");
            settings.add_filter(r"(A|[2-9]|T|J|Q|K)(H|C|D|S)", "<card>");
            settings.add_filter(r"\[<card>(, <card>)*\]", "[<cards>]");
            settings.add_filter(r"\d+->\d+", "<score>");
            settings
        }

        #[test]
        fn should_output_user_readable_starting_game_in_logs() {
            let game = GameBuilder::default().with_cuts("ASAC").into_starting();
            common_filters().bind(|| insta::assert_snapshot!(game.to_string(), @"Starting(cuts: <playerid> -> <card>, <playerid> -> <card>, deck: [<cards>])"));
        }

        #[test]
        fn should_output_user_readable_discarding_game_in_logs() {
            let game = GameBuilder::default()
                .with_peggings(0, 0)
                .with_hands("AH2H3H4H5H6H", "AC2C3C4C5C6C")
                .into_discarding();
            common_filters().bind(|| insta::assert_snapshot!(game.to_string(), @"Discarding(scores: Peggings(<playerid> -> <score>, <playerid> -> <score>) Reasons([]), roles: Roles(dealer: <playerid>, pone: <playerid>), hands: <playerid> -> [<cards>], <playerid> -> [<cards>], crib: [], deck: [<cards>])"));
        }

        #[test]
        fn should_output_user_readable_playing_game_in_logs() {
            let game = GameBuilder::default()
                .with_peggings(0, 0)
                .with_score_reasons(&[ScoreReason::new(
                    ScoreReasonType::Fifteen,
                    valid_hand("KS5S").as_ref(),
                    2.into(),
                )])
                .with_hands("9S", "4S")
                .with_cut("AS")
                .with_current_plays(&[(0, "AH")])
                .into_playing(1);
            common_filters().bind(|| insta::assert_snapshot!(game.to_string(), @"Playing(scores: Peggings(<playerid> -> <score>, <playerid> -> <score>) Reasons([Fifteen: [<cards>] => 2]), roles: Roles(dealer: <playerid>, pone: <playerid>), hands: <playerid> -> [<cards>], <playerid> -> [<cards>], play_state: Next(<playerid>), Legal(<playerid> -> [<cards>], <playerid> -> [<cards>]), Passes(0), Current((<playerid> -> <card>)), Previous(), cut: <card>, crib: [])"));
        }

        #[test]
        fn should_output_user_readable_pone_scoring_game_in_logs() {
            let game = GameBuilder::default()
                .with_peggings(0, 0)
                .with_hands("AS2S3S4S", "AC2C3C4C")
                .with_cut("JH")
                .with_crib("TSJSQSKS")
                .into_scoring_pone();
            common_filters().bind(|| insta::assert_snapshot!(game.to_string(), @"ScoringPone(scores: Peggings(<playerid> -> <score>, <playerid> -> <score>) Reasons([]), roles: Roles(dealer: <playerid>, pone: <playerid>), hands: <playerid> -> [<cards>], <playerid> -> [<cards>], cut: <card>, crib: [<cards>])"));
        }

        #[test]
        fn should_output_user_readable_dealer_scoring_game_in_logs() {
            let game = GameBuilder::default()
                .with_peggings(0, 0)
                .with_hands("AS2S3S4S", "AC2C3C4C")
                .with_cut("JH")
                .with_crib("TSJSQSKS")
                .into_scoring_dealer();
            common_filters().bind(|| insta::assert_snapshot!(game.to_string(), @"ScoringDealer(scores: Peggings(<playerid> -> <score>, <playerid> -> <score>) Reasons([]), roles: Roles(dealer: <playerid>, pone: <playerid>), hands: <playerid> -> [<cards>], <playerid> -> [<cards>], cut: <card>, crib: [<cards>])"));
        }

        #[test]
        fn should_output_user_readable_crib_scoring_game_in_logs() {
            let game = GameBuilder::default()
                .with_peggings(0, 0)
                .with_hands("AS2S3S4S", "AC2C3C4C")
                .with_cut("JH")
                .with_crib("TSJSQSKS")
                .into_scoring_crib();
            common_filters().bind(|| insta::assert_snapshot!(game.to_string(), @"ScoringCrib(scores: Peggings(<playerid> -> <score>, <playerid> -> <score>) Reasons([]), roles: Roles(dealer: <playerid>, pone: <playerid>), hands: <playerid> -> [<cards>], <playerid> -> [<cards>], cut: <card>, crib: [<cards>])"));
        }

        #[test]
        fn should_output_user_readable_finished_game_in_logs() {
            let game = GameBuilder::default()
                .with_peggings(0, 0)
                .with_hands("AS2S3S4S", "AC2C3C4C")
                .with_cut("JH")
                .with_crib("TSJSQSKS")
                .into_finished();
            common_filters().bind(|| insta::assert_snapshot!(game.to_string(), @"Finished(winner: <playerid>, peggings: <playerid> -> <score>, <playerid> -> <score>, cut: <card>)"));
        }
    }
}
