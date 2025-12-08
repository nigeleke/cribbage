use cqrs_es::{Aggregate, event_sink::EventSink};
use serde::{Deserialize, Serialize};
use tracing::debug;

use crate::{
    display::format_vec,
    domain::{
        Card, Crib, CutsForDeal, Dealer, Deck, Discarding, DomainError, GameCommand, GameEvent,
        GameId, Hand, Hands, HasCrib, HasCutsForDeal, HasDeck, HasHands, HasPending, HasPlayState,
        HasRoles, HasScoreboard, HasStarterCut, PLAYER0, PLAYER1, Pegging, Pending, Play,
        PlayState, Player, Playing, Roles, ScoreSheet, Scoreboard, ScoringCrib, ScoringDealer,
        ScoringPone, StarterCut, Starting, State, UserId,
        constants::{CARDS_DISCARDED_TO_CRIB, CARDS_KEPT_PER_HAND, PLAYER_COUNT},
        state::Wrap,
    },
    name_builder::generate_game_name,
};

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Game {
    id: GameId,
    host: UserId,
    guest: Option<UserId>,
    name: String,
    state: State,
}

impl Game {
    pub fn new(id: GameId, host: UserId, guest: Option<UserId>, name: &str, state: State) -> Self {
        let name = String::from(name);
        Self {
            id,
            host,
            guest,
            name,
            state,
        }
    }

    pub fn id(&self) -> &GameId {
        &self.id
    }

    pub fn host(&self) -> &UserId {
        &self.host
    }

    pub fn guest(&self) -> Option<&UserId> {
        self.guest.as_ref()
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    #[cfg(test)]
    pub fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }

    pub fn state(&self) -> &State {
        &self.state
    }

    #[cfg(test)]
    pub fn state_mut(&mut self) -> &mut State {
        &mut self.state
    }

    pub fn validate_user(&self, user_id: UserId) -> Option<Player> {
        match user_id {
            id if id == self.host => Some(PLAYER0),
            id if Some(id) == self.guest => Some(PLAYER1),
            _ => None,
        }
    }
}

impl Game {
    fn host_game(&self, host: UserId, game_id: GameId) -> Result<Vec<GameEvent>, DomainError> {
        let not_permitted = || Err(DomainError::NotPermitted(String::from("host game")));

        if self.id != GameId::default() {
            not_permitted()
        } else {
            let name = generate_game_name();
            let events = vec![GameEvent::LobbyGameCreated {
                game_id,
                host,
                name,
            }];
            Ok(events)
        }
    }

    fn join_game(&self, guest: UserId) -> Result<Vec<GameEvent>, DomainError> {
        let not_permitted = || Err(DomainError::NotPermitted(String::from("join game")));

        if self.id == GameId::default() || self.guest.is_some() {
            not_permitted()
        } else if self.host == guest {
            Err(DomainError::InvalidOpponent)
        } else {
            let events = vec![GameEvent::LobbyGameJoined { guest }];
            Ok(events)
        }
    }

    fn play_computer(&self, host: UserId, game_id: GameId) -> Result<Vec<GameEvent>, DomainError> {
        let not_permitted = || Err(DomainError::NotPermitted(String::from("play computer")));

        if self.id != GameId::default() {
            not_permitted()
        } else {
            let guest = UserId::new();
            let name = generate_game_name();
            let events = vec![GameEvent::ComputerGameCreated {
                game_id,
                host,
                guest,
                name,
            }];
            Ok(events)
        }
    }

    fn cut_for_deal(&self, player: Player) -> Result<Vec<GameEvent>, DomainError> {
        let not_permitted = || Err(DomainError::NotPermitted(String::from("cut for deal")));

        let cut_for_deal = |starting: &Starting| {
            if !starting.pending().waiting_on(player) {
                not_permitted()
            } else {
                let mut deck = starting.deck().clone();
                let cut = deck.cut();
                let events = vec![GameEvent::CutForDealMade { player, cut }];
                Ok(events)
            }
        };

        if self.id == GameId::default() {
            not_permitted()
        } else {
            match &self.state {
                State::Starting(starting) => {
                    let events = cut_for_deal(starting)?;
                    Ok(events)
                }
                _ => not_permitted(),
            }
        }
    }

    fn start_game(&self, player: Player) -> Result<Vec<GameEvent>, DomainError> {
        let not_permitted = || Err(DomainError::NotPermitted(String::from("start game")));

        let start_game = |starting: &Starting| {
            let mut events = vec![GameEvent::GameStarted { player }];

            let proceed = starting.pending().clone().acknowledge(player);
            if proceed {
                if let Some(roles) = Roles::from_cuts(starting.cuts_for_deal()) {
                    let mut deck = Deck::shuffled_pack();
                    let hands = deck.deal(PLAYER_COUNT);

                    events.append(&mut vec![
                        GameEvent::CutForDealDecided {
                            dealer: *roles.dealer(),
                        },
                        GameEvent::HandDealt {
                            player: PLAYER0,
                            hand: hands[PLAYER0].clone(),
                        },
                        GameEvent::HandDealt {
                            player: PLAYER1,
                            hand: hands[PLAYER1].clone(),
                        },
                    ]);
                } else {
                    events.push(GameEvent::CutForDealTied);
                }
            }

            events
        };

        if self.id == GameId::default() {
            not_permitted()
        } else {
            match &self.state {
                State::Starting(starting) => {
                    let events = start_game(starting);
                    Ok(events)
                }
                _ => not_permitted(),
            }
        }
    }

    fn discard_cards(
        &self,
        player: Player,
        cards: Vec<Card>,
    ) -> Result<Vec<GameEvent>, DomainError> {
        let not_permitted = || Err(DomainError::NotPermitted("discard cards".into()));

        let discard_cards_to_crib = |discarding: &Discarding| {
            if !discarding.pending().waiting_on(player) {
                not_permitted()
            } else if !discarding.hand(player).contains_all(&cards)
                || cards.len() != CARDS_DISCARDED_TO_CRIB
                || discarding.hand(player).len() - cards.len() != CARDS_KEPT_PER_HAND
            {
                Err(DomainError::InvalidDiscards(format_vec(&cards)))
            } else {
                let mut events = vec![GameEvent::CardsDiscarded {
                    player,
                    cards: cards.clone(),
                }];

                let proceed = discarding.pending().clone().acknowledge(player);
                if proceed {
                    let cut = discarding.deck().clone().cut();
                    let dealer = discarding.dealer().player();
                    let pegging = Pegging::new(dealer, ScoreSheet::his_heels(cut));
                    events.push(GameEvent::StarterSelected { cut, pegging });
                }

                Ok(events)
            }
        };

        if self.id == GameId::default() {
            not_permitted()
        } else {
            match &self.state {
                State::Discarding(discarding) => {
                    let events = discard_cards_to_crib(discarding)?;
                    Ok(events)
                }
                _ => not_permitted(),
            }
        }
    }

    fn play_card(&self, player: Player, card: Card) -> Result<Vec<GameEvent>, DomainError> {
        let not_permitted = || Err(DomainError::NotPermitted("play card".into()));

        let play_card = |playing: &Playing| {
            let play_state = playing.play_state();
            if play_state.next_to_play() != player {
                Err(DomainError::NotPlayersTurn(player))
            } else if !play_state.legal_plays(player).contains(&card) {
                Err(DomainError::InvalidPlay(card))
            } else {
                let mut play_state = play_state.clone();
                let score_sheet = play_state.play(card);
                let pegging = Pegging::new(player, score_sheet);
                Ok(vec![GameEvent::CardPlayed {
                    player,
                    card,
                    pegging,
                }])
            }
        };

        if self.id == GameId::default() {
            not_permitted()
        } else {
            match &self.state {
                State::Playing(playing) => {
                    let events = play_card(playing)?;
                    Ok(events)
                }
                _ => not_permitted(),
            }
        }
    }

    fn go(&self, player: Player) -> Result<Vec<GameEvent>, DomainError> {
        let not_permitted = || Err(DomainError::NotPermitted("go".into()));

        let go = |playing: &Playing| {
            let play_state = playing.play_state();
            if play_state.next_to_play() != player {
                Err(DomainError::NotPlayersTurn(player))
            } else if !play_state.legal_plays(player).is_empty() {
                Err(DomainError::InvalidGo)
            } else {
                // There will always be a valid play before a go can occur. The `or` condition
                // in `map_or` will never occur.
                let recipient = play_state
                    .current_plays()
                    .last()
                    .map_or(player, Play::player);
                let pegging = Pegging::new(recipient, ScoreSheet::go(&play_state));
                Ok(vec![GameEvent::GoCalled { player, pegging }])
            }
        };

        if self.id == GameId::default() {
            not_permitted()
        } else {
            match &self.state {
                State::Playing(playing) => {
                    let events = go(playing)?;
                    Ok(events)
                }
                _ => not_permitted(),
            }
        }
    }

    fn score_pone(&self, player: Player) -> Result<Vec<GameEvent>, DomainError> {
        let not_permitted = || Err(DomainError::NotPermitted(String::from("score pone")));

        let score_pone = |playing: &Playing| {
            let pone = playing.pone().player();
            let hands = playing.play_state().clone().finish_plays();
            let hand = &hands[pone];
            let cut = playing.starter_cut();

            let pegging = Pegging::new(pone, ScoreSheet::hand(hand, *cut));
            vec![GameEvent::PoneScored { player, pegging }]
        };

        if self.id == GameId::default() {
            not_permitted()
        } else {
            match &self.state {
                State::Playing(playing) => {
                    let events = score_pone(playing);
                    Ok(events)
                }
                _ => not_permitted(),
            }
        }
    }

    fn score_dealer(&self, player: Player) -> Result<Vec<GameEvent>, DomainError> {
        let not_permitted = || Err(DomainError::NotPermitted(String::from("score dealer")));

        let score_dealer = |scoring: &ScoringPone| {
            let dealer = scoring.dealer().player();
            let hand = scoring.hand(dealer);
            let cut = scoring.starter_cut();

            let pegging = Pegging::new(dealer, ScoreSheet::hand(hand, *cut));
            vec![GameEvent::DealerScored { player, pegging }]
        };

        if self.id == GameId::default() {
            not_permitted()
        } else {
            match &self.state {
                State::ScoringPone(scoring) => {
                    let events = score_dealer(scoring);
                    Ok(events)
                }
                _ => not_permitted(),
            }
        }
    }

    fn score_crib(&self, player: Player) -> Result<Vec<GameEvent>, DomainError> {
        let not_permitted = || Err(DomainError::NotPermitted(String::from("score crib")));

        let score_crib = |scoring: &ScoringDealer| {
            let dealer = scoring.dealer().player();
            let crib = scoring.crib();
            let cut = scoring.starter_cut();

            let pegging = Pegging::new(dealer, ScoreSheet::crib(crib, *cut));
            vec![GameEvent::CribScored { player, pegging }]
        };

        if self.id == GameId::default() {
            not_permitted()
        } else {
            match &self.state {
                State::ScoringDealer(scoring) => {
                    let events = score_crib(scoring);
                    Ok(events)
                }
                _ => not_permitted(),
            }
        }
    }

    fn start_next_round(&self, player: Player) -> Result<Vec<GameEvent>, DomainError> {
        let not_permitted = || Err(DomainError::NotPermitted(String::from("start next round")));

        let start_next_round = |scoring: &ScoringCrib| {
            let mut events = vec![GameEvent::NextRoundStarted { player }];

            let proceed = scoring.pending().clone().acknowledge(player);
            if proceed {
                let mut deck = Deck::shuffled_pack();
                let hands = deck.deal(PLAYER_COUNT);
                events.append(&mut vec![
                    GameEvent::HandDealt {
                        player: PLAYER0,
                        hand: hands[PLAYER0].clone(),
                    },
                    GameEvent::HandDealt {
                        player: PLAYER1,
                        hand: hands[PLAYER1].clone(),
                    },
                ]);
            };

            events
        };

        if self.id == GameId::default() {
            not_permitted()
        } else {
            match &self.state {
                State::ScoringCrib(scoring) => {
                    let events = start_next_round(scoring);
                    Ok(events)
                }
                _ => not_permitted(),
            }
        }
    }

    pub fn handle_command(&self, command: GameCommand) -> Result<Vec<GameEvent>, DomainError> {
        debug!("COMMAND --- Game:handle_command: {:?}", command);
        match command {
            GameCommand::HostGame { user_id, game_id } => self.host_game(user_id, game_id),
            GameCommand::JoinGame { user_id } => self.join_game(user_id),
            GameCommand::PlayComputer { user_id, game_id } => self.play_computer(user_id, game_id),
            GameCommand::CutForDeal { player } => self.cut_for_deal(player),
            GameCommand::StartGame { player } => self.start_game(player),
            GameCommand::DiscardCards { player, cards } => self.discard_cards(player, cards),
            GameCommand::PlayCard { player, card } => self.play_card(player, card),
            GameCommand::Go { player } => self.go(player),
            GameCommand::ScorePone { player } => self.score_pone(player),
            GameCommand::ScoreDealer { player } => self.score_dealer(player),
            GameCommand::ScoreCrib { player } => self.score_crib(player),
            GameCommand::StartNextRound { player } => self.start_next_round(player),
        }
    }
}

impl Game {
    fn lobby_game_created(&mut self, game_id: GameId, host: UserId, name: String) {
        self.id = game_id;
        self.host = host;
        self.name = name;
        self.state = Starting::default().wrap();
    }

    fn lobby_game_joined(&mut self, guest: UserId) {
        self.guest = Some(guest);
    }

    fn computer_game_created(
        &mut self,
        game_id: GameId,
        host: UserId,
        guest: UserId,
        name: String,
    ) {
        self.id = game_id;
        self.host = host;
        self.guest = Some(guest);
        self.name = name;
        self.state = Starting::default().wrap();
    }

    fn cut_for_deal_made(&mut self, player: Player, cut: Card) {
        if let State::Starting(starting) = &mut self.state {
            *starting.cut_for_deal_mut(player) = Some(cut);
            starting.deck_mut().remove(cut);
        }
    }

    fn game_started(&mut self, player: Player) {
        if let State::Starting(starting) = &mut self.state {
            starting.pending_mut().acknowledge(player);
        }
    }

    fn cut_for_deal_decided(&mut self, dealer: Dealer) {
        if let State::Starting(_) = &self.state {
            let scoreboard = Scoreboard::default();
            let roles = Roles::new(dealer);
            let deck = Deck::shuffled_pack();
            let hands = Hands::default();
            let crib = Crib::default();
            let pending = Pending::default();
            let discarding = Discarding::new(scoreboard, roles, hands, crib, deck, pending);
            self.state = discarding.wrap();
        }
    }

    fn hand_dealt(&mut self, player: Player, hand: Hand) {
        if let State::Discarding(discarding) = &mut self.state {
            let hands = discarding.hands_mut();
            hands[player] = hand.clone();

            let deck = discarding.deck_mut();
            deck.remove_all(hand.as_ref());
        }
    }

    fn cut_for_deal_tied(&mut self) {
        if let State::Starting(_) = &self.state {
            let cuts = CutsForDeal::default();
            let deck = Deck::shuffled_pack();
            let pending = Pending::default();
            let starting = Starting::new(cuts, deck, pending);
            self.state = starting.wrap();
        }
    }

    fn cards_discarded(&mut self, player: Player, cards: &[Card]) {
        if let State::Discarding(discarding) = &mut self.state {
            discarding.hand_mut(player).remove_all(cards);
            discarding.crib_mut().add_all(cards);
            discarding.pending_mut().acknowledge(player);
        }
    }

    fn starter_selected(&mut self, starter_cut: StarterCut, pegging: Pegging) {
        if let State::Discarding(discarding) = &mut self.state {
            discarding.scoreboard_mut().peg(&pegging);

            let play_state = PlayState::new(discarding.pone().player())
                .with_pending_plays(PLAYER0, discarding.hand(PLAYER0).as_ref())
                .with_pending_plays(PLAYER1, discarding.hand(PLAYER1).as_ref());

            let pending = Pending::default();

            let playing = Playing::new(
                discarding.scoreboard().clone(),
                *discarding.roles(),
                discarding.hands().clone(),
                play_state,
                discarding.crib().clone(),
                starter_cut,
                pending,
            );

            self.state = playing.wrap().or_finished();
        }
    }

    fn card_played(&mut self, _player: Player, card: Card, pegging: Pegging) {
        if let State::Playing(playing) = &mut self.state {
            playing.play_card(card);
            playing.scoreboard_mut().peg(&pegging);
            self.state = playing.clone().wrap().or_finished();
        }
    }

    fn go_called(&mut self, _player: Player, pegging: Pegging) {
        if let State::Playing(playing) = &mut self.state {
            playing.go();
            playing.scoreboard_mut().peg(&pegging);
            self.state = playing.clone().wrap().or_finished();
        }
    }

    fn pone_scored(&mut self, player: Player, pegging: Pegging) {
        if let State::Playing(playing) = &mut self.state {
            let proceed = playing.pending_mut().acknowledge(player);
            if proceed {
                playing.scoreboard_mut().peg(&pegging);

                let hands = playing.play_state_mut().finish_plays();
                let pending = Pending::default();

                let scoring = ScoringPone::new(
                    playing.scoreboard().clone(),
                    *playing.roles(),
                    hands,
                    playing.crib().clone(),
                    *playing.starter_cut(),
                    pegging,
                    pending,
                );

                self.state = scoring.wrap().or_finished();
            }
        }
    }

    fn dealer_scored(&mut self, player: Player, pegging: Pegging) {
        if let State::ScoringPone(scoring) = &mut self.state {
            let proceed = scoring.pending_mut().acknowledge(player);
            if proceed {
                scoring.scoreboard_mut().peg(&pegging);

                let pending = Pending::default();

                let scoring = ScoringDealer::new(
                    scoring.scoreboard().clone(),
                    *scoring.roles(),
                    scoring.hands().clone(),
                    scoring.crib().clone(),
                    *scoring.starter_cut(),
                    pegging,
                    pending,
                );

                self.state = scoring.wrap().or_finished();
            }
        }
    }

    fn crib_scored(&mut self, player: Player, pegging: Pegging) {
        if let State::ScoringDealer(scoring) = &mut self.state {
            let proceed = scoring.pending_mut().acknowledge(player);
            if proceed {
                scoring.scoreboard_mut().peg(&pegging);

                let pending = Pending::default();

                let scoring = ScoringCrib::new(
                    scoring.scoreboard().clone(),
                    *scoring.roles(),
                    scoring.hands().clone(),
                    scoring.crib().clone(),
                    *scoring.starter_cut(),
                    pegging,
                    pending,
                );

                self.state = scoring.wrap().or_finished();
            }
        }
    }

    fn next_round_started(&mut self, player: Player) {
        if let State::ScoringCrib(scoring) = &mut self.state {
            let proceed = scoring.pending_mut().acknowledge(player);
            if proceed {
                let scoreboard = scoring.scoreboard().clone();
                let mut roles = *scoring.roles();
                roles.swap();
                let hands = Hands::default();
                let crib = Crib::default();
                let deck = Deck::default();
                let pending = Pending::default();

                self.state = Discarding::new(scoreboard, roles, hands, crib, deck, pending).wrap();
            }
        }
    }

    pub fn apply_event(&mut self, event: GameEvent) {
        debug!("EVENT ----- Game:apply_event: {:?}", event);
        match event {
            GameEvent::LobbyGameCreated {
                game_id,
                host,
                name,
            } => self.lobby_game_created(game_id, host, name),
            GameEvent::LobbyGameJoined { guest } => self.lobby_game_joined(guest),
            GameEvent::ComputerGameCreated {
                game_id,
                host,
                guest,
                name,
            } => self.computer_game_created(game_id, host, guest, name),
            GameEvent::CutForDealMade { player, cut } => self.cut_for_deal_made(player, cut),
            GameEvent::GameStarted { player } => self.game_started(player),
            GameEvent::CutForDealDecided { dealer } => self.cut_for_deal_decided(dealer),
            GameEvent::HandDealt { player, hand } => self.hand_dealt(player, hand),
            GameEvent::CutForDealTied => self.cut_for_deal_tied(),
            GameEvent::CardsDiscarded { player, cards } => self.cards_discarded(player, &cards),
            GameEvent::StarterSelected { cut, pegging } => self.starter_selected(cut, pegging),
            GameEvent::CardPlayed {
                player,
                card,
                pegging,
            } => self.card_played(player, card, pegging),
            GameEvent::GoCalled { player, pegging } => self.go_called(player, pegging),
            GameEvent::PoneScored { player, pegging } => self.pone_scored(player, pegging),
            GameEvent::DealerScored { player, pegging } => self.dealer_scored(player, pegging),
            GameEvent::CribScored { player, pegging } => self.crib_scored(player, pegging),
            GameEvent::NextRoundStarted { player } => self.next_round_started(player),

            #[cfg(test)]
            GameEvent::GamePreloaded { game, .. } => *self = game,
        }
    }

    pub fn apply_events(&mut self, events: &[GameEvent]) {
        let events = events.to_owned();
        for event in events {
            self.apply_event(event);
        }
    }
}

#[derive(Clone, Default)]
pub struct GameServices;

impl Aggregate for Game {
    const TYPE: &'static str = stringify!(Game);

    type Command = GameCommand;
    type Event = GameEvent;
    type Error = DomainError;
    type Services = GameServices;

    async fn handle(
        &mut self,
        command: Self::Command,
        _services: &Self::Services,
        sink: &EventSink<Self>,
    ) -> Result<(), Self::Error> {
        let events = self.handle_command(command)?;

        for event in events {
            sink.write(event, self).await;
        }

        Ok(())
    }

    fn apply(&mut self, event: Self::Event) {
        self.apply_event(event);
    }
}

impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}\n{} {}\n{}",
            self.name,
            self.host,
            self.guest
                .map_or("-".into(), |g| std::convert::identity(g).to_string()),
            self.state
        )
    }
}

#[cfg(test)]
impl From<&[GameEvent]> for Game {
    fn from(value: &[GameEvent]) -> Self {
        let mut game = Self::default();
        game.apply_events(value);
        game
    }
}

#[cfg(test)]
impl From<State> for Game {
    fn from(state: State) -> Self {
        let id = GameId::new();
        let host = UserId::new();
        let guest = Some(UserId::new());
        let name = format!("test_game_{}_{}", state.as_ref(), chrono::Utc::now());
        Self {
            id,
            host,
            guest,
            name,
            state,
        }
    }
}

#[cfg(test)]
#[coverage(off)]
mod test {
    use super::*;

    /// # [Cribbage Rules](https://www.officialgamerules.org/cribbage)
    #[allow(clippy::expect_used)]

    /// ## Number of Players
    ///
    /// Two or three people can play. Or four people can play two against two as partners. But
    /// Cribbage is basically best played by two people, and the rules that follow are for that
    /// number.
    mod players {
        use super::*;
        use crate::{
            domain::{DomainError, GameCommand, GameEvent, GameId, UserId},
            find_then, function_name, game_test,
        };

        #[test]
        fn a_user_can_host_game() {
            let user_id = UserId::new();
            let game_id = GameId::new();

            game_test! {
                when: GameCommand::HostGame { user_id, game_id },
                then_events: |events: &[GameEvent]| {
                    find_then!(events, GameEvent::LobbyGameCreated {
                        game_id: event_game_id,
                        host: event_host,
                        name: event_name,
                    } => {
                        assert_eq!(event_game_id, &game_id);
                        assert_eq!(event_host, &user_id);
                        assert_ne!(event_name.trim(), &String::default());
                    });
                }
            }
        }

        #[test]
        fn a_user_can_play_the_computer() {
            let user_id = UserId::new();
            let game_id = GameId::new();

            game_test! {
                when: GameCommand::PlayComputer { user_id, game_id },
                then_events: |events: &[GameEvent]| {
                    find_then!(events, GameEvent::ComputerGameCreated {
                        game_id: event_game_id,
                        host: event_host,
                        guest: event_guest,
                        name: event_name,
                    } => {
                        assert_eq!(event_game_id, &game_id);
                        assert_eq!(event_host, &user_id);
                        assert_ne!(event_guest, &UserId::default());
                        assert_ne!(event_guest, &user_id);
                        assert_ne!(event_name.trim(), String::default());
                    });
                }
            }
        }

        #[test]
        fn a_user_can_join_lobby_game() {
            let game_id = GameId::new();
            let host = UserId::new();
            let guest = UserId::new();
            let name = function_name!();

            game_test! {
                given: &[GameEvent::LobbyGameCreated {
                    game_id,
                    host,
                    name: name.clone(),
                }],
                when: GameCommand::JoinGame { user_id: guest },
                then_events: |events: &[GameEvent]| {
                    assert_eq!(events, vec![GameEvent::LobbyGameJoined { guest }]);
                }
            }
        }

        #[test]
        fn a_user_cannot_join_active_game() {
            let game_id = GameId::new();
            let host = UserId::new();
            let guest = UserId::new();
            let name = function_name!();

            game_test! {
                given: &[GameEvent::ComputerGameCreated {
                    game_id,
                    host,
                    guest,
                    name,
                }],
                when: GameCommand::JoinGame { user_id: UserId::new() },
                then_error: DomainError::NotPermitted(String::from("join game"))
            }
        }

        #[test]
        fn a_different_user_must_join_lobby_game() {
            let game_id = GameId::new();
            let host = UserId::new();
            let guest = host;
            let name = function_name!();

            game_test! {
                given: &[GameEvent::LobbyGameCreated {
                    game_id,
                    host,
                    name,
                }],
                when: GameCommand::JoinGame { user_id: guest },
                then_error: DomainError::InvalidOpponent

            }
        }
    }

    /// ## The Pack
    ///
    /// The standard 52-card pack is used.
    ///
    /// Rank of Cards: K (high), Q, J, 10, 9, 8, 7, 6, 5, 4, 3, 2, A.
    mod deck {
        use super::*;
        use crate::{
            assert_state_then,
            domain::{DomainError, GameEvent, HasDeck, STANDARD_DECK_SIZE},
            function_name, game_test,
        };

        #[test]
        fn use_a_standard_pack_of_cards() {
            let user_id = UserId::new();
            let game_id = GameId::new();
            game_test! {
                when: GameCommand::PlayComputer { user_id, game_id },
                then_state: |state: &State| {
                    assert_state_then!(state, State::Starting(starting) => {
                        let deck = starting.deck();
                        assert_eq!(deck.len(), STANDARD_DECK_SIZE);
                    });
                }
            }
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
        use std::str::FromStr;

        use super::*;
        use crate::{
            assert_state_then, card,
            domain::{
                Dealer, DomainError, GameCommand, GameEvent, GameId, PLAYER0, PLAYER1,
                STANDARD_DECK_SIZE, UserId, constants::CARDS_DEALT_PER_HAND,
            },
            find_then, function_name, game_test,
        };

        #[test]
        fn user_must_cut_for_dealer_1() {
            let game_id = GameId::new();
            let host = UserId::new();
            let guest = UserId::new();
            let name = function_name!();

            game_test! {
                given: &[GameEvent::ComputerGameCreated {
                    game_id,
                    host,
                    guest,
                    name,
                }],
                when: GameCommand::CutForDeal { player: PLAYER0 },
                then_events: |events: &[GameEvent]| {
                    find_then!(events, GameEvent::CutForDealMade { player, .. } => {
                        assert_eq!(player, &PLAYER0);
                    });
                }
            }
        }

        #[test]
        fn user_must_cut_for_dealer_2() {
            let game_id = GameId::new();
            let host = UserId::new();
            let guest = UserId::new();
            let name = function_name!();

            let cut0 = card!("AS");

            game_test! {
                given: &[
                    GameEvent::ComputerGameCreated {
                        game_id,
                        host,
                        guest,
                        name,
                    },
                    GameEvent::CutForDealMade {
                        player: PLAYER0,
                        cut: cut0,
                    },
                ],
                when: GameCommand::CutForDeal { player: PLAYER1 },
                then_events: |events: &[GameEvent]| {
                    find_then!(events, GameEvent::CutForDealMade { player, cut } => {
                        assert_eq!(player, &PLAYER1);
                        assert_ne!(cut, &cut0);
                    });
                }
            }
        }

        #[test]
        fn dealer_decided_with_lowest_cut() {
            let game_id = GameId::new();
            let host = UserId::new();
            let guest = UserId::new();
            let name = function_name!();

            let cut0 = card!("AS");
            let cut1 = card!("QH");

            game_test! {
                given: &[
                    GameEvent::ComputerGameCreated {
                        game_id,
                        host,
                        guest,
                        name,
                    },
                    GameEvent::CutForDealMade {
                        player: PLAYER0,
                        cut: cut0,
                    },
                    GameEvent::CutForDealMade {
                        player: PLAYER1,
                        cut: cut1,
                    },
                    GameEvent::GameStarted { player: PLAYER0 },
                ],
                when: GameCommand::StartGame { player: PLAYER1 },
                then_events: |events: &[GameEvent]| {
                    assert_eq!(
                        events[..2],
                        vec![
                            GameEvent::GameStarted { player: PLAYER1 },
                            GameEvent::CutForDealDecided {
                                dealer: Dealer::from(PLAYER0)
                            }
                        ]
                    );
                    let deals = events
                        .iter()
                        .filter_map(|e| matches!(e, GameEvent::HandDealt { .. }).then_some(e))
                        .collect::<Vec<_>>();
                    assert_eq!(deals.len(), PLAYER_COUNT);
                },
                then_state: |state: &State| {
                    assert_state_then!(state, State::Discarding(discarding) => {
                        assert_eq!(discarding.deck().len(), STANDARD_DECK_SIZE - (CARDS_DEALT_PER_HAND * PLAYER_COUNT));
                    });
                }
            }
        }

        #[test]
        fn dealer_undecided_with_tied_cut() {
            let game_id = GameId::new();
            let host = UserId::new();
            let guest = UserId::new();
            let name = function_name!();

            let cut0 = card!("AS");
            let cut1 = card!("AH");

            game_test! {
                given: &[
                    GameEvent::ComputerGameCreated {
                        game_id,
                        host,
                        guest,
                        name,
                    },
                    GameEvent::CutForDealMade {
                        player: PLAYER0,
                        cut: cut0,
                    },
                    GameEvent::CutForDealMade {
                        player: PLAYER1,
                        cut: cut1,
                    },
                    GameEvent::GameStarted { player: PLAYER0 },
                ],
                when: GameCommand::StartGame { player: PLAYER1 },
                then_events: |events: &[GameEvent]| {
                    assert_eq!(
                        events,
                        vec![
                            GameEvent::GameStarted { player: PLAYER1 },
                            GameEvent::CutForDealTied
                        ]
                    )
                }
            }
        }
    }

    /// ## The Deal
    ///
    /// The dealer distributes six cards face down to his opponent and himself, beginning with the
    /// opponent.
    mod deal {
        use std::str::FromStr;

        use super::*;
        use crate::{
            card,
            domain::constants::{CARDS_DEALT_PER_HAND, PLAYER_COUNT},
            function_name, game_test,
        };

        #[test]
        fn dealer_deals_six_cards_each() {
            let game_id = GameId::new();
            let host = UserId::new();
            let guest = UserId::new();
            let name = function_name!();

            let cut0 = card!("AS");
            let cut1 = card!("QH");

            game_test! {
                given: &[
                    GameEvent::ComputerGameCreated {
                        game_id,
                        host,
                        guest,
                        name,
                    },
                    GameEvent::CutForDealMade {
                        player: PLAYER0,
                        cut: cut0,
                    },
                    GameEvent::CutForDealMade {
                        player: PLAYER1,
                        cut: cut1,
                    },
                    GameEvent::GameStarted { player: PLAYER0 },
                ],
                when: GameCommand::StartGame { player: PLAYER1 },
                then_events: |events: &[GameEvent]| {
                    assert_eq!(
                        events[..2],
                        vec![
                            GameEvent::GameStarted { player: PLAYER1 },
                            GameEvent::CutForDealDecided {
                                dealer: Dealer::from(PLAYER0)
                            }
                        ]
                    );
                    let hands = events
                        .into_iter()
                        .filter_map(|event| match event {
                            GameEvent::HandDealt { hand, player: _ } => Some(hand),
                            _ => None,
                        })
                        .collect::<Vec<_>>();
                    assert_eq!(hands.len(), PLAYER_COUNT);
                    assert_eq!(hands[PLAYER0].len(), CARDS_DEALT_PER_HAND);
                    assert_eq!(hands[PLAYER1].len(), CARDS_DEALT_PER_HAND);
                }
            }
        }
    }

    /// ## Object of the Game

    /// The goal is to be the first player to score 121 points. (Some games are to 61 points.)
    /// Players earn points during play and for making various card combinations.
    mod object_of_the_game {}

    /// ## The Crib
    ///
    /// Each player looks at his six cards and "lays away" (discards) two of them face down to
    /// reduce the hand to four. The four cards laid away together constitute "the crib". The crib
    /// belongs to the dealer, but these cards are not exposed or used until after the hands have
    /// been played.
    mod the_crib {
        use std::str::FromStr;

        use super::*;
        use crate::{
            assert_state_then, cards,
            domain::{
                DomainError, GameEvent, PLAYER0, STANDARD_DECK_SIZE,
                constants::CARDS_DEALT_PER_HAND, test::GameBuilder,
            },
            function_name, game_test, scenario,
        };

        #[test]
        fn player_can_discard_own_cards_to_the_crib() {
            game_test! {
                given: &scenario!(as_discarding;
                    with_hands("AH2H3H4H5H6H", "AC2C3C4C5C6C"),
                    with_cut("QD"),
                    with_crib("")
                ),
                when: GameCommand::DiscardCards {
                    player: PLAYER0,
                    cards: cards!("AH2H"),
                },
                then_events: |events: &[GameEvent]| {
                    assert_eq!(events, &[GameEvent::CardsDiscarded {
                        player: PLAYER0,
                        cards: cards!("AH2H"),
                    }])
                },
                then_state: |state: &State| {
                    assert_state_then!(state, State::Discarding(discarding) => {
                        assert!(discarding.deck().contains_none(&cards!("AH2H")));
                        assert_eq!(discarding.deck().len(), STANDARD_DECK_SIZE - (CARDS_DEALT_PER_HAND * PLAYER_COUNT));
                    });
                }
            }
        }

        #[test]
        fn player_cannot_discard_other_than_two_held_cards_to_the_crib() {
            for cards in vec!["AH2H3H", "AH"] {
                let cards = cards!(cards);
                let expected_error_text = format_vec(&cards);

                game_test! {
                    given: &scenario!(
                        as_discarding;
                        with_hands("AH2H3H4H5H6H", "AC2C3C4C5C6C"),
                        with_cut("QD"),
                        with_crib("")
                    ),
                    when: GameCommand::DiscardCards {
                        player: PLAYER0,
                        cards,
                    },
                    then_error: DomainError::InvalidDiscards(expected_error_text)
                }
            }
        }

        #[test]
        fn player_cannot_discard_unowned_cards_to_the_crib() {
            let cards = cards!("AC2C");
            let expected_error_text = format_vec(&cards);

            game_test! {
                given: &scenario!(
                    as_discarding;
                    with_hands("AH2H3H4H5H6H", "AC2C3C4C5C6C"),
                    with_cut("QD"),
                    with_crib("")
                ),
                when: GameCommand::DiscardCards {
                    player: PLAYER0,
                    cards,
                },
                then_error: DomainError::InvalidDiscards(expected_error_text)
            }
        }

        #[test]
        fn player_cannot_discard_if_already_discarded() {
            game_test! {
                given: &scenario!(
                    as_discarding;
                    with_hands("3H4H5H6H", "AC2C3C4C5C6C"),
                    with_ack(0),
                    with_cut("QD"),
                    with_crib("AH2H")
                ),
                when: GameCommand::DiscardCards {
                    player: PLAYER0,
                    cards: cards!("5H6H"),
                },
                then_error: DomainError::NotPermitted("discard cards".into())
            }
        }
    }

    /// ## Before the Play
    ///
    /// After the crib is laid away, the non-dealer cuts the pack. The dealer turns up the top card
    /// of the lower packet and places it face up on top of the pack. This card is the "starter." If
    /// the starter is a jack, it is called "His Heels," and the dealer pegs (scores) 2 points at
    /// once. The starter is not used in the play phase of Cribbage , but is used later for making
    /// various card combinations that score points.
    mod before_the_play {
        use std::str::FromStr;

        use super::*;
        use crate::{
            assert_state_then, card, cards,
            domain::{GameEvent, PLAYER0, PLAYER1, Points, test::GameBuilder},
            find_then, function_name, game_test, scenario,
        };

        #[test]
        fn start_the_play_after_discards() {
            game_test! {
                given: &scenario!(
                    as_discarding;
                    with_hands("3H4H5H6H", "AC2C3C4C5C6C"),
                    with_ack(0),
                    with_cut("QD"),
                    with_crib("AH2H")
                ),
                when: GameCommand::DiscardCards {
                    player: PLAYER1,
                    cards: cards!("AC2C"),
                },
                then_events: |events: &[GameEvent]| {
                    find_then!(events, GameEvent::CardsDiscarded { player, cards } => {
                        assert_eq!(player, &PLAYER1);
                        assert_eq!(cards, &cards!("AC2C"));
                    });

                    find_then!(events, GameEvent::StarterSelected { .. } => {});
                },
                then_state: |state: &State| {
                    assert_state_then!(state, State::Playing(playing) => {
                        let starter_cut = *playing.starter_cut();
                        assert!(!playing.hand(PLAYER0).contains(starter_cut));
                        assert!(!playing.hand(PLAYER1).contains(starter_cut));
                        assert!(!playing.crib().contains(starter_cut));
                    });
                }
            }
        }

        #[test]
        fn score_his_heels_when_jack_cut_after_discards() {
            game_test! {
                given: &scenario!(
                    as_discarding;
                    with_hands("3H4H5H6H", "AC2C3C4C5C6C"),
                    with_ack(0),
                    with_cut("JC"),
                    with_crib("AH2H")
                ),
                when: GameCommand::DiscardCards {
                    player: PLAYER1,
                    cards: cards!("AC2C"),
                },
                then_events: |events: &[GameEvent]| {
                    find_then!(events, GameEvent::StarterSelected { cut, pegging } => {
                        assert_eq!(pegging.player(), &PLAYER0);
                        assert_eq!(pegging.score_sheet().points(), Points::from(2));
                        assert_eq!(pegging.score_sheet(), &ScoreSheet::his_heels(card!("JC")));
                    });
                }
            }
        }

        #[test]
        fn finish_game_when_jack_cut_after_discards() {
            game_test! {
                given: &scenario!(
                    as_discarding;
                    with_hands("3H4H5H6H", "AC2C3C4C5C6C"),
                    with_ack(0),
                    with_cut("JC"),
                    with_crib("AH2H"),
                    with_points(119, 0)
                ),
                when: GameCommand::DiscardCards {
                    player: PLAYER1,
                    cards: cards!("AC2C"),
                },
                then_state: |state: &State| {
                    assert_state_then!(state, State::Finished(finished) => {
                        assert_eq!(finished.winner(), PLAYER0)
                    });
                }
            }
        }
    }

    /// ## The Play
    ///
    /// After the starter is turned, the non-dealer lays one of his cards face up on the table. The
    /// dealer similarly exposes a card, then non-dealer again, and so on - the hands are exposed
    /// card by card, alternately except for a "Go", as noted below. Each player keeps his
    /// cards separate from those of his opponent.
    ///
    /// As each person plays, he announces a running total of pips reached by the addition of the
    /// last card to all those previously2 played. (Example: The non-dealer begins with a four,
    /// saying "Four." The dealer plays a nine, saying "Thirteen".) The kings, queens and jacks
    /// count 10 each; every other card counts its pip value (the ace counts one).
    mod the_play {
        use std::str::FromStr;

        use super::*;
        use crate::{
            assert_state_then, card, cards,
            domain::{
                Card, DomainError, GameCommand, GameEvent, PLAYER1, Pegging, Player, Points, Pone,
                ScoreKind, test::GameBuilder,
            },
            function_name, game_test, plays, scenario,
        };

        #[test]
        fn accept_valid_play() {
            game_test! {
                given: &scenario!(
                    as_playing(1);
                    with_cut("AC"),
                    with_hands("QH", "4C")
                ),
                when: GameCommand::PlayCard {
                    player: PLAYER1,
                    card: card!("4C"),
                },
                then_events: |events: &[GameEvent]| {
                    assert_eq!(events, &[GameEvent::CardPlayed {
                        player: PLAYER1,
                        card: card!("4C"),
                        pegging: Pegging::new(PLAYER1, ScoreSheet::default()),
                    }]);
                }
            }
        }

        #[test]
        fn accept_valid_play_after_opponent_go_called() {
            game_test! {
                given: &scenario!(
                    as_playing(1);
                    with_go(),
                    with_cut("AC"),
                    with_hands("9S", "4SAS"),
                    with_current_plays(&[(1, "TC"), (0, "TD"), (0, "5C")])
                ),
                when:GameCommand::PlayCard {
                    player: PLAYER1,
                    card: card!("4S"),
                },
                then_events: |events: &[GameEvent]| {
                    assert_eq!(events, &[GameEvent::CardPlayed {
                        player: PLAYER1,
                        card: card!("4S"),
                        pegging: Pegging::new(PLAYER1, ScoreSheet::default()),
                    }])
                }
            }
        }

        #[test]
        fn cannot_play_when_unheld_card() {
            game_test! {
                given: &scenario!(
                    as_playing(1);
                    with_points(0,0),
                    with_hands("9S", "4S"),
                    with_cut("AS")
                ),
                when: GameCommand::PlayCard {
                    player: PLAYER1,
                    card: card!("9S"),
                },
                then_error: DomainError::InvalidPlay(card!("9S"))
            }
        }

        #[test]
        fn cannot_play_when_not_their_turn() {
            game_test! {
                given: &scenario!(
                    as_playing(1);
                    with_points(0, 0),
                    with_hands("9S", "4S"),
                    with_cut("AS")
                ),
                when: GameCommand::PlayCard {
                    player: PLAYER0,
                    card: card!("9S"),
                },
                then_error: DomainError::NotPlayersTurn(PLAYER0)
            }
        }

        #[test]
        fn cannot_play_when_play_exceeds_target() {
            game_test! {
                given: &scenario!(
                    as_playing(1);
                    with_points(0, 0),
                    with_hands("9S", "4S"),
                    with_cut("AS"),
                    with_current_plays(&[(0, "KH"), (0, "KC"), (0, "KD")])
                ),
                when: GameCommand::PlayCard {
                    player: PLAYER1,
                    card: card!("4S"),
                },
                then_error: DomainError::InvalidPlay(card!("4S"))
            }
        }

        #[test]
        fn score_play_when_target_not_reached_mid_play() {
            game_test! {
                given: &scenario!(
                    as_playing(1);
                    with_points(0, 0),
                    with_hands("5S", "5H"),
                    with_cut("AS"),
                    with_current_plays(&[(0, "TH")])
                ),
                when: GameCommand::PlayCard {
                    player: PLAYER1,
                    card: card!("5H"),
                },
                then_events: |events: &[GameEvent]| {
                    assert_eq!(events, &[GameEvent::CardPlayed {
                        player: PLAYER1,
                        card: card!("5H"),
                        pegging: Pegging::new(
                            PLAYER1,
                            ScoreSheet::default().add_event(
                                ScoreKind::Fifteen,
                                &cards!("TH5H"),
                                Points::from(2),
                            ),
                        ),
                    }])
                }
            }
        }

        #[test]
        fn score_play_when_target_not_reached_end_play() {
            game_test! {
                given: &scenario!(
                    as_playing(1);
                    with_points(0, 0),
                    with_hands("QS", "2H"),
                    with_cut("QC"),
                    with_current_plays(&[(0, "JH"), (0, "2C")]),
                    with_previous_plays(&[(0, "7C"), (1, "6S"), (1, "2S"), (1, "KS")])
                ),
                when: GameCommand::PlayCard {
                    player: PLAYER1,
                    card: card!("2H"),
                },
                then_events: |events: &[GameEvent]| {
                    assert_eq!(events, &[GameEvent::CardPlayed {
                        player: PLAYER1,
                        card: card!("2H"),
                        pegging: Pegging::new(
                            PLAYER1,
                            ScoreSheet::default().add_event(
                                ScoreKind::Pair,
                                &cards!("2H2C"),
                                Points::from(2),
                            ),
                        ),
                    }])
                }
            }
        }

        #[test]
        fn score_play_when_target_not_reached_finished() {
            game_test! {
                given: &scenario!(
                    as_playing(1);
                    with_points(0, 120),
                    with_hands("AH", "5H"),
                    with_cut("QC"),
                    with_current_plays(&[(0, "JH")]),
                    with_previous_plays(&[(0, "9H"), (0, "7C"), (1, "6S"), (1, "2S"), (1, "KS")])
                ),
                when: GameCommand::PlayCard { player: PLAYER1, card: card!("5H") },
                then_events: |events: &[GameEvent]| {
                    assert_eq!(events, &[GameEvent::CardPlayed {
                        player: PLAYER1,
                        card: card!("5H"),
                        pegging: Pegging::new(
                            PLAYER1,
                            ScoreSheet::default().add_event(
                                ScoreKind::Fifteen,
                                &cards!("JH5H"),
                                Points::from(2),
                            ),
                        ),
                    }])
                }
            }
        }

        #[test]
        fn score_play_when_target_reached_mid_play() {
            game_test! {
                given: &scenario!(as_playing(1);
                    with_points(0, 0),
                    with_hands("9H", "AH"),
                    with_cut("KC"),
                    with_current_plays(&[(0, "TH"), (0, "JH"), (0, "QH")]),
                    with_previous_plays(&[(1, "2S"), (1, "QS"), (1, "6S")])
                ),
                when: GameCommand::PlayCard {
                    player: PLAYER1,
                    card: card!("AH"),
                },
                then_events: |events: &[GameEvent]| {
                    assert_eq!(events, &[GameEvent::CardPlayed {
                        player: PLAYER1,
                        card: card!("AH"),
                        pegging: Pegging::new(
                            PLAYER1,
                            ScoreSheet::default().add_event(
                                ScoreKind::ThirtyOne,
                                &cards!("THJHQHAH"),
                                Points::from(2),
                            ),
                        ),
                    }])
                }
            }
        }

        #[test]
        fn score_play_when_target_reached_end_play() {
            game_test! {
                given: &scenario!(as_playing(1);
                    with_points(0, 0),
                    with_hands("QC", "AH"),
                    with_cut("KC"),
                    with_current_plays(&[(0, "TH"), (0, "JH"), (0, "QH")]),
                    with_previous_plays(&[(1, "2S"), (1, "QS"), (1, "6S")])
                ),
                when: GameCommand::PlayCard {
                    player: PLAYER1,
                    card: card!("AH"),
                },
                then_events: |events: &[GameEvent]| {
                    assert_eq!(events, &[GameEvent::CardPlayed {
                        player: PLAYER1,
                        card: card!("AH"),
                        pegging: Pegging::new(
                            PLAYER1,
                            ScoreSheet::default().add_event(
                                ScoreKind::ThirtyOne,
                                &cards!("THJHQHAH"),
                                Points::from(2),
                            ),
                        ),
                    }])
                }
            }
        }

        #[test]
        fn score_play_when_target_reached_finished() {
            game_test! {
                given: &scenario!(
                    as_playing(1);
                    with_points(0, 120),
                    with_hands("QC", "AH"),
                    with_cut("KC"),
                    with_current_plays(&[(0, "TH"), (1, "JH"), (0, "QH")]),
                    with_previous_plays(&[(1, "9H"), (1, "5S"), (0, "6S")])
                ),
                when: GameCommand::PlayCard {
                    player: PLAYER1,
                    card: card!("AH"),
                },
                then_events: |events: &[GameEvent]| {
                    assert_eq!(events, &[GameEvent::CardPlayed {
                        player: PLAYER1,
                        card: card!("AH"),
                        pegging: Pegging::new(
                            PLAYER1,
                            ScoreSheet::default().add_event(
                                ScoreKind::ThirtyOne,
                                &cards!("THJHQHAH"),
                                Points::from(2),
                            ),
                        ),
                    }]);
                }
            }
        }

        #[test]
        fn score_play_when_plays_finished_and_game_not_finished() {
            game_test! {
                given: &scenario!(
                    as_playing(1);
                    with_points(0, 60),
                    with_hands("", "AH"),
                    with_cut("KC"),
                    with_current_plays(&[(0, "8H"), (1, "JH"), (0, "QH")]),
                    with_previous_plays(&[(1, "9H"), (0, "4S"), (1, "5S"), (0, "6S")])
                ),
                when: GameCommand::PlayCard {
                    player: PLAYER1,
                    card: card!("AH"),
                },
                then_events: |events: &[GameEvent]| {
                    assert_eq!(events, &[GameEvent::CardPlayed {
                        player: PLAYER1,
                        card: card!("AH"),
                        pegging: Pegging::new(
                            PLAYER1,
                            ScoreSheet::default().add_event(ScoreKind::LastCard, &[], Points::from(1)),
                        ),
                    }]);
                }
            }
        }

        #[test]
        fn score_play_when_plays_finished_and_game_finished() {
            game_test! {
                given: &scenario!(
                    as_playing(1);
                    with_points(0, 120),
                    with_hands("", "AH"),
                    with_cut("KC"),
                    with_current_plays(&[(0, "8H"), (1, "JH"), (0, "QH")]),
                    with_previous_plays(&[(1, "9H"), (0, "4S"), (1, "5S"), (0, "6S")])
                ),
                when: GameCommand::PlayCard {
                    player: PLAYER1,
                    card: card!("AH"),
                },
                then_events: |events: &[GameEvent]| {
                    assert_eq!(events, &[GameEvent::CardPlayed {
                        player: PLAYER1,
                        card: card!("AH"),
                        pegging: Pegging::new(
                            PLAYER1,
                            ScoreSheet::default().add_event(ScoreKind::LastCard, &[], Points::from(1)),
                        ),
                    }]);
                }
            }
        }

        #[test]
        fn swap_player_after_pone_play() {
            game_test! {
                given: &scenario!(
                    as_playing(1);
                    with_points(0, 0),
                    with_cut("AS"),
                    with_hands("7H8H8D9C", "4S5STHJH")
                ),
                when: GameCommand::PlayCard {
                    player: PLAYER1,
                    card: card!("4S"),
                },
                then_state: |state: &State| {
                    assert_state_then!(state, State::Playing(playing) => {
                        assert_eq!(playing.play_state().next_to_play(), PLAYER0);
                    });
                }
            }
        }

        #[test]
        fn swap_player_after_dealer_play() {
            game_test! {
                given: &scenario!(
                    as_playing(0);
                    with_points(0, 0),
                    with_cut("AS"),
                    with_hands("7H8H8D9C", "5STHJH"),
                    with_current_plays(&[(1, "4S")])
                ),
                when: GameCommand::PlayCard {
                    player: PLAYER0,
                    card: card!("9C"),
                },
                then_state: |state: &State| {
                    assert_state_then!(state, State::Playing(playing) => {
                        assert_eq!(playing.play_state().next_to_play(), PLAYER1);
                    });
                }
            }
        }

        #[test]
        fn reset_play_after_exact_target_reached() {
            game_test! {
                given: &scenario!(
                    as_playing(0);
                    with_points(0, 0),
                    with_cut("AS"),
                    with_hands("7H8H8D", "5STH"),
                    with_current_plays(&[(1, "JH"), (0, "9C"), (1, "4S")])
                ),
                when: GameCommand::PlayCard {
                    player: PLAYER0,
                    card: card!("8H"),
                },
                then_state: |state: &State| {
                    assert_state_then!(state, State::Playing(playing) => {
                        assert_eq!(playing.dealer(), &Dealer::from(PLAYER0));
                        assert_eq!(playing.pone(), &Pone::from(PLAYER1));
                        assert_eq!(playing.play_state().next_to_play(), PLAYER1);
                        assert_eq!(
                            playing.play_state().previous_plays(),
                            plays!(&[(1, "JH"), (0, "9C"), (1, "4S"), (0, "8H")])
                        );
                        assert!(playing.play_state().current_plays().is_empty());
                    });
                }
            }
        }

        #[test]
        fn score_play_points_for_fifteens() {
            game_test! {
                given: &scenario!(
                    as_playing(1);
                    with_points(0, 0),
                    with_cut("AS"),
                    with_hands("KH", "8D"),
                    with_current_plays(&[(0, "7D")])
                ),
                when: GameCommand::PlayCard {
                    player: PLAYER1,
                    card: card!("8D"),
                },
                then_events: |events: &[GameEvent]| {
                    assert_eq!(events, &[GameEvent::CardPlayed {
                        player: PLAYER1,
                        card: card!("8D"),
                        pegging: Pegging::new(
                            PLAYER1,
                            ScoreSheet::default().add_event(
                                ScoreKind::Fifteen,
                                &cards!("7D8D"),
                                Points::from(2),
                            ),
                        ),
                    }]);
                }
            }
        }

        #[test]
        fn score_play_points_for_pair() {
            game_test! {
                given: &scenario!(
                    as_playing(1);
                    with_points(0, 0),
                    with_cut("AS"),
                    with_hands("KH", "8D"),
                    with_current_plays(&[(0, "8S")])
                ),
                when: GameCommand::PlayCard {
                    player: PLAYER1,
                    card: card!("8D"),
                },
                then_events: |events: &[GameEvent]| {
                    assert_eq!(events, &[GameEvent::CardPlayed {
                        player: PLAYER1,
                        card: card!("8D"),
                        pegging: Pegging::new(
                            PLAYER1,
                            ScoreSheet::default().add_event(
                                ScoreKind::Pair,
                                &cards!("8D8S"),
                                Points::from(2),
                            ),
                        ),
                    }])
                }
            }
        }

        #[test]
        fn score_play_points_for_triplet() {
            game_test! {
                given: &scenario!(
                    as_playing(1);
                    with_points(0, 0),
                    with_cut("AS"),
                    with_hands("KH", "8DAH"),
                    with_current_plays(&[(1, "8C"), (0, "8S")])
                ),
                when: GameCommand::PlayCard {
                    player: PLAYER1,
                    card: card!("8D"),
                },
                then_events: |events: &[GameEvent]| {
                    assert_eq!(events, &[GameEvent::CardPlayed {
                        player: PLAYER1,
                        card: card!("8D"),
                        pegging: Pegging::new(
                            PLAYER1,
                            ScoreSheet::default().add_event(
                                ScoreKind::Triplet,
                                &cards!("8D8S8C"),
                                Points::from(6),
                            ),
                        ),
                    }])
                }
            }
        }

        #[test]
        fn score_play_points_for_quartet() {
            game_test! {
                given: &scenario!(
                    as_playing(1);
                    with_points(0, 0),
                    with_cut("AS"),
                    with_hands("KH", "7DAH"),
                    with_current_plays(&[(1, "7C"), (0, "7S"), (0, "7H")])
                ),
                when: GameCommand::PlayCard {
                    player: PLAYER1,
                    card: card!("7D"),
                },
                then_events: |events: &[GameEvent]| {
                    assert_eq!(events, &[GameEvent::CardPlayed {
                        player: PLAYER1,
                        card: card!("7D"),
                        pegging: Pegging::new(
                            PLAYER1,
                            ScoreSheet::default().add_event(
                                ScoreKind::Quadruplet,
                                &cards!("7D7H7S7C"),
                                Points::from(12),
                            ),
                        ),
                    }])
                }
            }
        }

        #[test]
        fn score_play_points_for_run() {
            game_test! {
                given: &scenario!(
                    as_playing(1);
                    with_points(0, 0),
                    with_cut("AC"),
                    with_hands("KH", "AS"),
                    with_current_plays(&[(1, "2D"), (0, "3H")])
                ),
                when: GameCommand::PlayCard {
                    player: PLAYER1,
                    card: card!("AS"),
                },
                then_events: |events: &[GameEvent]| {
                    assert_eq!(events, &[GameEvent::CardPlayed {
                        player: PLAYER1,
                        card: card!("AS"),
                        pegging: Pegging::new(
                            PLAYER1,
                            ScoreSheet::default().add_event(
                                ScoreKind::Run,
                                &cards!("AS2D3H"),
                                Points::from(3),
                            ),
                        ),
                    }])
                }
            }
        }

        #[test]
        fn score_play_points_for_run_edge_case_1() {
            game_test! {
                given: &scenario!(
                    as_playing(0);
                    with_points(0, 0),
                    with_cut("AS"),
                    with_hands("7H6H", "AH"),
                    with_current_plays(&[(1, "8S"), (0, "7H"), (1, "7S")])
                ),
                when: GameCommand::PlayCard {
                    player: PLAYER0,
                    card: card!("6H"),
                },
                then_events: |events: &[GameEvent]| {
                    assert_eq!(events, &[GameEvent::CardPlayed {
                        player: PLAYER0,
                        card: card!("6H"),
                        pegging: Pegging::new(PLAYER0, ScoreSheet::default()),
                    }])
                }
            }
        }

        #[test]
        fn score_play_points_for_run_edge_case_2() {
            game_test! {
                given: &scenario!(
                    as_playing(0);
                    with_points(0, 0),
                    with_cut("AS"),
                    with_hands("5H7H", "AH"),
                    with_current_plays(&[(1, "9S"), (0, "6H"), (1, "8S")])
                ),
                when: GameCommand::PlayCard {
                    player: PLAYER0,
                    card: card!("7H"),
                },
                then_events: |events: &[GameEvent]| {
                    assert_eq!(events, &[GameEvent::CardPlayed {
                        player: PLAYER0,
                        card: card!("7H"),
                        pegging: Pegging::new(
                            PLAYER0,
                            ScoreSheet::default().add_event(
                                ScoreKind::Run,
                                &cards!("6H7H8S9S"),
                                Points::from(4),
                            ),
                        ),
                    }])
                }
            }
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
        use std::str::FromStr;

        use crate::{
            assert_state_then, card,
            domain::{
                Card, Dealer, DomainError, GameCommand, GameEvent, HasPlayState, HasRoles, PLAYER0,
                PLAYER1, Pegging, Play, Player, Points, Pone, ScoreKind, ScoreSheet, State,
                test::GameBuilder,
            },
            function_name, game_test, plays, scenario,
        };

        #[test]
        fn accept_go_when_pone_has_no_valid_card() {
            game_test! {
                given: &scenario!(
                    as_playing(1);
                    with_points(0, 0),
                    with_cut("AS"),
                    with_hands("AH", "KH"),
                    with_current_plays(&[(0, "TH"), (0, "JH"), (0, "QH")])
                ),
                when: GameCommand::Go { player: PLAYER1 },
                then_events: |events: &[GameEvent]| {
                    assert_eq!(events, &[GameEvent::GoCalled {
                        player: PLAYER1,
                        pegging: Pegging::new(PLAYER0, ScoreSheet::default()),
                    }])
                }
            }
        }

        #[test]
        fn accept_go_when_dealer_has_no_valid_card() {
            game_test! {
                given: &scenario!(
                    as_playing(0);
                    with_go(),
                    with_points(0, 0),
                    with_cut("AS"),
                    with_hands("KH", "KS"),
                    with_current_plays(&[(0, "TH"), (1, "QH"), (0, "JH")])
                ),
                when: GameCommand::Go { player: PLAYER0 },
                then_events: |events: &[GameEvent]| {
                    assert_eq!(events, &[GameEvent::GoCalled {
                        player: PLAYER0,
                        pegging: Pegging::new(
                            PLAYER0,
                            ScoreSheet::default().add_event(ScoreKind::LastCard, &[], Points::from(1)),
                        ),
                    }])
                }
            }
        }

        #[test]
        fn cannot_call_go_when_valid_card_held() {
            game_test! {
                given: &scenario!(
                    as_playing(1);
                    with_points(0, 0),
                    with_cut("AC"),
                    with_hands("AH", "AS"),
                    with_current_plays(&[(0, "TH"), (0, "JH"), (0, "8H")])
                ),
                when: GameCommand::Go { player: PLAYER1 },
                then_error: DomainError::InvalidGo
            }
        }

        #[test]
        fn cannot_call_go_when_not_turn() {
            game_test! {
                given: &scenario!(
                    as_playing(1);
                    with_points(0, 0),
                    with_cut("AC"),
                    with_hands("AH", "AS"),
                    with_current_plays(&[(0, "TH"), (0, "JH"), (0, "8H")])
                ),
                when: GameCommand::Go { player: PLAYER0 },
                then_error: DomainError::NotPlayersTurn(PLAYER0)
            }
        }

        #[test]
        fn score_go_when_both_players_called_go_playing() {
            game_test! {
                given: &scenario!(
                    as_playing(0);
                    with_go(),
                    with_points(0, 0),
                    with_cut("AS"),
                    with_hands("KH", "KS"),
                    with_current_plays(&[(0, "TH"), (1, "QH"), (0, "JH")])
                ),
                when: GameCommand::Go { player: PLAYER0 },
                then_events: |events: &[GameEvent]| {
                    assert_eq!(events, &[GameEvent::GoCalled {
                        player: PLAYER0,
                        pegging: Pegging::new(
                            PLAYER0,
                            ScoreSheet::default().add_event(ScoreKind::LastCard, &[], Points::from(1)),
                        ),
                    }])
                }
            }
        }

        #[test]
        fn score_go_when_both_players_called_go_finished() {
            game_test! {
                given: &scenario!(
                    as_playing(0);
                    with_go(),
                    with_points(120, 0),
                    with_cut("AS"),
                    with_hands("KH", "KS"),
                    with_current_plays(&[(0, "TH"), (1, "QH"), (0, "JH")])
                ),
                when: GameCommand::Go { player: PLAYER0 },
                then_events: |events: &[GameEvent]| {
                    assert_eq!(events, &[GameEvent::GoCalled {
                        player: PLAYER0,
                        pegging: Pegging::new(
                            PLAYER0,
                            ScoreSheet::default().add_event(ScoreKind::LastCard, &[], Points::from(1)),
                        ),
                    }])
                },
                then_state: |state: &State| {
                    assert_state_then!(state, State::Finished(finished) => {
                        assert_eq!(finished.winner(), PLAYER0);
                    })
                }
            }
        }

        #[test]
        fn swap_player_after_pone_called_go() {
            game_test! {
                given: &scenario!(
                    as_playing(1);
                    with_points(0, 0),
                    with_cut("AS"),
                    with_hands("8H8D", "5SJH"),
                    with_current_plays(&[(1, "4S"), (0, "9C"), (1, "TH"), (0, "7H")])
                ),
                when: GameCommand::Go { player: PLAYER1 },
                then_state: |state: &State| {
                    assert_state_then!(state, State::Playing(playing) => {
                        assert_eq!(playing.play_state().next_to_play(), PLAYER0)
                    })
                }
            }
        }

        #[test]
        fn swap_player_after_dealer_called_go() {
            game_test! {
                given: &scenario!(
                    as_playing(0);
                    with_points(0, 0),
                    with_cut("AS"),
                    with_hands("7H8H8D", "4S5S"),
                    with_current_plays(&[(1, "JH"), (0, "9C"), (1, "TH")])
                ),
                when: GameCommand::Go { player: PLAYER0 },
                then_state: |state: &State| {
                    assert_state_then!(state, State::Playing(playing) => {
                        assert_eq!(playing.play_state().next_to_play(), PLAYER1)
                    })
                }
            }
        }

        #[test]
        fn reset_play_after_pone_then_dealer_called_go() {
            game_test! {
                given: &scenario!(
                    as_playing(0);
                    with_go(),
                    with_points(0, 0),
                    with_cut("AS"),
                    with_hands("8H8D", "5SJH"),
                    with_current_plays(&[(1, "4S"), (0, "9C"), (1, "TH"), (0, "7H")])
                ),
                when: GameCommand::Go { player: PLAYER0 },
                then_state: |state: &State| {
                    assert_state_then!(state, State::Playing(playing) => {
                        assert_eq!(playing.dealer(), &Dealer::from(PLAYER0));
                        assert_eq!(playing.pone(), &Pone::from(PLAYER1));
                        assert_eq!(playing.play_state().next_to_play(), PLAYER1);
                        assert_eq!(
                            playing.play_state().previous_plays(),
                            plays!(&[(1, "4S"), (0, "9C"), (1, "TH"), (0, "7H")])
                        );
                        assert!(playing.play_state().current_plays().is_empty());
                    })
                }
            }
        }

        #[test]
        fn reset_play_after_after_dealer_then_pone_called_go() {
            game_test! {
                given: &scenario!(
                    as_playing(1);
                    with_go(),
                    with_points(0, 0),
                    with_cut("AS"),
                    with_hands("7H8H8D", "4S5S"),
                    with_current_plays(&[(1, "JH"), (0, "9C"), (1, "TH")])
                ),
                when: GameCommand::Go { player: PLAYER1 },
                then_state: |state: &State| {
                    assert_state_then!(state, State::Playing(playing) => {
                        assert_eq!(playing.dealer(), &Dealer::from(PLAYER0));
                        assert_eq!(playing.pone(), &Pone::from(PLAYER1));
                        assert_eq!(playing.play_state().next_to_play(), PLAYER0);
                        assert_eq!(
                            playing.play_state().previous_plays(),
                            plays!(&[(1, "JH"), (0, "9C"), (1, "TH")])
                        );
                        assert!(playing.play_state().current_plays().is_empty());
                    })
                }
            }
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
        use crate::{
            domain::{Game, GameEvent, HasPlayState, Points, ScoreSheet, State, test::GameBuilder},
            function_name, scenario,
        };

        #[test]
        fn should_score_fifteens() {
            let State::Playing(playing) = Game::from(
                scenario!(
                    as_playing(1);
                    with_points(0, 0),
                    with_hands("AC", ""),
                    with_current_plays(&[(0, "JD"), (0, "5H")]),
                    with_cut("AH")
                )
                .as_slice(),
            )
            .state
            else {
                panic!("unexpected state");
            };

            assert_eq!(
                ScoreSheet::play_card(playing.play_state()).points(),
                Points::from(2)
            )
        }

        #[test]
        fn should_score_pairs() {
            let State::Playing(playing) = Game::from(
                scenario!(
                    as_playing(1);
                    with_points(0, 0),
                    with_hands("AC", ""),
                    with_current_plays(&[(0, "JD"), (0, "AH"), (0, "AS")]),
                    with_cut("KH")
                )
                .as_slice(),
            )
            .state
            else {
                panic!("unexpected state");
            };

            assert_eq!(
                ScoreSheet::play_card(playing.play_state()).points(),
                Points::from(2)
            )
        }

        #[test]
        fn should_score_royal_pairs() {
            let State::Playing(playing) = Game::from(
                scenario!(
                    as_playing(1);
                    with_points(0, 0),
                    with_hands("AC", ""),
                    with_current_plays(&[(0, "AD"), (0, "AH"), (0, "AS")]),
                    with_cut("KH")
                )
                .as_slice(),
            )
            .state
            else {
                panic!("unexpected state");
            };

            assert_eq!(
                ScoreSheet::play_card(playing.play_state()).points(),
                Points::from(6)
            )
        }

        #[test]
        fn should_score_double_royal_pairs() {
            let State::Playing(playing) = Game::from(
                scenario!(
                    as_playing(1);
                    with_points(0, 0),
                    with_hands("2H", ""),
                    with_current_plays(&[(0, "AC"), (0, "AD"), (0, "AH"), (0, "AS")]),
                    with_cut("KH")
                )
                .as_slice(),
            )
            .state
            else {
                panic!("unexpected state")
            };

            assert_eq!(
                ScoreSheet::play_card(playing.play_state()).points(),
                Points::from(12)
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
                let State::Playing(playing) = Game::from(
                    scenario!(
                        as_playing(1);
                        with_points(0, 0),
                        with_hands("AS", "AD"),
                        with_current_plays(&current_plays),
                        with_cut("KH")
                    )
                    .as_slice(),
                )
                .state
                else {
                    panic!("unexpected state")
                };

                assert_eq!(
                    ScoreSheet::play_card(playing.play_state()).points(),
                    Points::from(if len < 3 { 0 } else { len })
                )
            }
        }

        #[test]
        fn should_score_runs_unordered() {
            let State::Playing(playing) = Game::from(
                scenario!(
                    as_playing(1);
                    with_points(0, 0),
                    with_hands("KS", "KD"),
                    with_current_plays(&[(0, "3S"), (0, "2C"), (0, "AS")]),
                    with_cut("KH")
                )
                .as_slice(),
            )
            .state
            else {
                panic!("unexpected state");
            };

            assert_eq!(
                ScoreSheet::play_card(playing.play_state()).points(),
                Points::from(3)
            )
        }

        #[test]
        fn should_score_rules_example_flush() {
            let State::Playing(playing) = Game::from(
                scenario!(
                    as_playing(0);
                    with_points(0, 0),
                    with_hands("", "2H"),
                    with_cut("3H"),
                    with_current_plays(&[(1, "TH"), (0, "8H"), (1, "QH"), (0, "AH")])
                )
                .as_slice(),
            )
            .state
            else {
                panic!("unexpected state");
            };

            assert_eq!(
                ScoreSheet::play_card(playing.play_state()).points(),
                Points::from(0)
            );
        }

        #[test]
        fn should_score_when_target_not_reached() {
            let State::Playing(playing) = Game::from(
                scenario!(
                    as_playing(1);
                    with_go(),
                    with_points(0, 0),
                    with_hands("", ""),
                    with_current_plays(&[(0, "AC"), (0, "2D"), (0, "5H"), (0, "4S")]),
                    with_cut("KH")
                )
                .as_slice(),
            )
            .state
            else {
                panic!("unexpected state");
            };

            assert_eq!(
                ScoreSheet::go(playing.play_state()).points(),
                Points::from(1)
            );
        }

        #[test]
        fn should_score_when_target_reached() {
            let State::Playing(playing) = Game::from(
                scenario!(
                    as_playing(1);
                    with_points(0, 0),
                    with_hands("", ""),
                    with_current_plays(&[(0, "KC"), (0, "KD"), (0, "KH"), (0, "AS")]),
                    with_cut("KS")
                )
                .as_slice(),
            )
            .state
            else {
                panic!("unexpected state");
            };

            assert_eq!(
                ScoreSheet::play_card(playing.play_state()).points(),
                Points::from(2)
            )
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
        use std::str::FromStr;

        use crate::{
            assert_state_then, card, crib,
            domain::{
                Card, Crib, Dealer, DomainError, GameCommand, GameEvent, Hand, HasHands, HasRoles,
                PLAYER0, PLAYER1, Points, Pone, ScoreSheet, State, constants::CARDS_DEALT_PER_HAND,
                test::GameBuilder,
            },
            find_then, function_name, game_test, hand, scenario,
        };

        #[test]
        fn score_pone_hand_when_plays_finished() {
            game_test! {
                given: &scenario!(
                    as_playing(1);
                    with_points(0, 0),
                    with_hands("", "TH"),
                    with_cut("4H"),
                    with_previous_plays(&[
                        (0, "7H"), (0, "8C"), (0, "AC"), (0, "2C"),
                        (1, "QH"), (1, "KS"), (1, "5H"), (1, "TH"),
                    ]),
                    with_ack(0)
                ),
                when: GameCommand::ScorePone { player: PLAYER1 },
                then_events: |events: &[GameEvent]| {
                    assert_eq!(events.len(), 1);

                    find_then!(events, GameEvent::PoneScored { player, pegging } => {
                        assert_eq!(player, &PLAYER1);
                        assert_eq!(pegging.player(), &PLAYER1);
                        assert_eq!(pegging.score_sheet().points(), Points::from(6));
                    });
                }
            }
        }

        #[test]
        fn score_winning_pone_hand_when_plays_finished() {
            game_test! {
                given: &scenario!(
                    as_playing(1);
                    with_points(0, 115),
                    with_hands("", "TH"),
                    with_cut("4H"),
                    with_previous_plays(&[
                        (0, "7H"), (0, "8C"), (0, "AC"), (0, "2C"),
                        (1, "QH"), (1, "KS"), (1, "5H"), (1, "TH"),
                    ]),
                    with_ack(0)
                ),
                when: GameCommand::ScorePone { player: PLAYER1 },
                then_events: |events: &[GameEvent]| {
                    find_then!(events, GameEvent::PoneScored { player, pegging } => {
                        assert_eq!(player, &PLAYER1);
                        assert_eq!(pegging.player(), &PLAYER1);
                        assert_eq!(pegging.score_sheet().points(), Points::from(6));
                    });
                },
                then_state: |state: &State| {
                    assert_state_then!(state, State::Finished(finished) => {
                        assert_eq!(finished.winner(), PLAYER1);
                    });
                }
            }
        }

        #[test]
        fn score_dealer_hand_when_pone_score_acknowledged() {
            game_test! {
                given: &scenario!(
                    as_scoring_pone;
                    with_points(0, 0),
                    with_cut("4H"),
                    with_hands("7H8CAC2C", "JCKS5HTH"),
                    with_crib("AHADASTD"),
                    with_ack(0),
                ),
                when: GameCommand::ScoreDealer { player: PLAYER1 },
                then_events: |events: &[GameEvent]| {
                    assert_eq!(events.len(), 1);

                    find_then!(events, GameEvent::DealerScored { player, pegging } => {
                        assert_eq!(player, &PLAYER1);
                        assert_eq!(pegging.player(), &PLAYER0);
                        assert_eq!(pegging.score_sheet().points(), Points::from(4));
                    });
                }
            }
        }

        #[test]
        fn score_winning_dealer_hand_when_pone_score_acknowledged() {
            game_test! {
                given: &scenario!(
                    as_scoring_pone;
                    with_points(117, 0),
                    with_cut("4H"),
                    with_hands("7H8CAC2C", "JCKS5HTH"),
                    with_crib("AHADASTD"),
                    with_ack(0),
                ),
                when: GameCommand::ScoreDealer { player: PLAYER1 },
                then_events: |events: &[GameEvent]| {
                    assert_eq!(events.len(), 1);

                    find_then!(events, GameEvent::DealerScored { player, pegging } => {
                        assert_eq!(player, &PLAYER1);
                        assert_eq!(pegging.player(), &PLAYER0);
                        assert_eq!(pegging.score_sheet().points(), Points::from(4));
                    });
                },
                then_state: |state: &State| {
                    assert_state_then!(state, State::Finished(finished) => {
                        assert_eq!(finished.winner(), PLAYER0);
                    });
                }
            }
        }

        #[test]
        fn score_crib_when_dealer_score_acknowledged() {
            game_test! {
                given: &scenario!(
                    as_scoring_dealer;
                    with_points(0, 0),
                    with_cut("4H"),
                    with_hands("7H8CAC2C", "JCKS5HTH"),
                    with_crib("AHADASTD"),
                    with_ack(0),
                ),
                when: GameCommand::ScoreCrib { player: PLAYER1 },
                then_events: |events: &[GameEvent]| {
                    assert_eq!(events.len(), 1);

                    find_then!(events, GameEvent::CribScored { player, pegging } => {
                        assert_eq!(player, &PLAYER1);
                        assert_eq!(pegging.player(), &PLAYER0);
                        assert_eq!(pegging.score_sheet().points(), Points::from(12));
                    });
                }
            }
        }

        #[test]
        fn score_winning_crib_when_dealer_score_acknowledged() {
            game_test! {
                given: &scenario!(
                    as_scoring_dealer;
                    with_points(109, 0),
                    with_cut("4H"),
                    with_hands("7H8CAC2C", "JCKS5HTH"),
                    with_crib("AHADASTD"),
                    with_ack(0),
                ),
                when: GameCommand::ScoreCrib { player: PLAYER1 },
                then_events: |events: &[GameEvent]| {
                    assert_eq!(events.len(), 1);

                    find_then!(events, GameEvent::CribScored { player, pegging } => {
                        assert_eq!(player, &PLAYER1);
                        assert_eq!(pegging.player(), &PLAYER0);
                        assert_eq!(pegging.score_sheet().points(), Points::from(12));
                    });
                },
                then_state: |state: &State| {
                    assert_state_then!(state, State::Finished(finished) => {
                        assert_eq!(finished.winner(), PLAYER0);
                    });
                }
            }
        }

        #[test]
        fn redeal_when_crib_score_acknowledged() {
            game_test! {
                given: &scenario!(
                    as_scoring_crib;
                    with_points(0, 0),
                    with_cut("4H"),
                    with_hands("7H8CAC2C", "JCKS5HTH"),
                    with_crib("AHADASTD"),
                    with_ack(0)
                ),
                when: GameCommand::StartNextRound { player: PLAYER1 },
                then_state: |state: &State| {
                    assert_state_then!(state, State::Discarding(discarding) => {
                        assert_eq!(discarding.dealer(), &Dealer::from(PLAYER1));
                        assert_eq!(discarding.pone(), &Pone::from(PLAYER0));
                        assert_eq!(discarding.hand(PLAYER0).len(), CARDS_DEALT_PER_HAND);
                        assert_eq!(discarding.hand(PLAYER1).len(), CARDS_DEALT_PER_HAND);
                    })
                }
            }
        }

        #[test]
        fn hand_should_score_fifteens() {
            assert_eq!(
                ScoreSheet::hand(&hand!("7H8CAC2C"), card!("4H")).points(),
                Points::from(4)
            );
            assert_eq!(
                ScoreSheet::hand(&hand!("THJCKS5H"), card!("4H")).points(),
                Points::from(6)
            );
        }

        #[test]
        fn hand_should_score_pairs() {
            assert_eq!(
                ScoreSheet::hand(&hand!("2H4C5C2C"), card!("AH")).points(),
                Points::from(2)
            );
            assert_eq!(
                ScoreSheet::hand(&hand!("TCASADTH"), card!("AH")).points(),
                Points::from(8)
            );
        }

        #[test]
        fn hand_should_score_royal_pairs() {
            assert_eq!(
                ScoreSheet::hand(&hand!("2H2D5C2C"), card!("AH")).points(),
                Points::from(6)
            );
            assert_eq!(
                ScoreSheet::hand(&hand!("TCASADTH"), card!("AH")).points(),
                Points::from(8)
            );
        }

        #[test]
        fn hand_should_score_double_royal_pairs() {
            assert_eq!(
                ScoreSheet::hand(&hand!("2H2C2D2S"), card!("AH")).points(),
                Points::from(12)
            );
            assert_eq!(
                ScoreSheet::hand(&hand!("TCASADTH"), card!("AH")).points(),
                Points::from(8)
            );
        }

        #[test]
        fn hand_should_score_runs() {
            assert_eq!(
                ScoreSheet::hand(&hand!("JDQCKC2C"), card!("AH")).points(),
                Points::from(3)
            );
            assert_eq!(
                ScoreSheet::hand(&hand!("3C3S2D5H"), card!("AH")).points(),
                Points::from(8)
            );
        }

        #[test]
        fn hand_should_score_flushes() {
            assert_eq!(
                ScoreSheet::hand(&hand!("2H4H6H8H"), card!("TH")).points(),
                Points::from(5)
            );
            assert_eq!(
                ScoreSheet::hand(&hand!("2D4D6D8D"), card!("TH")).points(),
                Points::from(4)
            );
        }

        #[test]
        fn hand_should_score_nobs() {
            assert_eq!(
                ScoreSheet::hand(&hand!("2D4H6HJH"), card!("TH")).points(),
                Points::from(1)
            );
            assert_eq!(
                ScoreSheet::hand(&hand!("2H4D6DJD"), card!("TH")).points(),
                Points::from(0)
            );
        }

        #[test]
        fn crib_should_score_fifteens() {
            assert_eq!(
                ScoreSheet::crib(&crib!("7H8CAC2C"), card!("4H")).points(),
                Points::from(4)
            );
            assert_eq!(
                ScoreSheet::crib(&crib!("THJCKS5H"), card!("4H")).points(),
                Points::from(6)
            );
        }

        #[test]
        fn crib_should_score_pairs() {
            assert_eq!(
                ScoreSheet::crib(&crib!("2H4C5C2C"), card!("AH")).points(),
                Points::from(2)
            );
            assert_eq!(
                ScoreSheet::crib(&crib!("TCASADTH"), card!("AH")).points(),
                Points::from(8)
            );
        }

        #[test]
        fn crib_should_score_royal_pairs() {
            assert_eq!(
                ScoreSheet::crib(&crib!("2H2D5C2C"), card!("AH")).points(),
                Points::from(6)
            );
            assert_eq!(
                ScoreSheet::crib(&crib!("TCASADTH"), card!("AH")).points(),
                Points::from(8)
            );
        }

        #[test]
        fn crib_should_score_double_royal_pairs() {
            assert_eq!(
                ScoreSheet::crib(&crib!("2H2C2D2S"), card!("AH")).points(),
                Points::from(12)
            );
            assert_eq!(
                ScoreSheet::crib(&crib!("TCASADTH"), card!("AH")).points(),
                Points::from(8)
            );
        }

        #[test]
        fn crib_should_score_runs() {
            assert_eq!(
                ScoreSheet::crib(&crib!("JDQCKC2C"), card!("AH")).points(),
                Points::from(3)
            );
            assert_eq!(
                ScoreSheet::crib(&crib!("3C3S2D5H"), card!("AH")).points(),
                Points::from(8)
            );
        }

        #[test]
        fn crib_should_score_flushes() {
            assert_eq!(
                ScoreSheet::crib(&crib!("2H4H6H8H"), card!("TH")).points(),
                Points::from(5)
            );
            assert_eq!(
                ScoreSheet::crib(&crib!("2D4D6D8D"), card!("TH")).points(),
                Points::from(0)
            );
        }

        #[test]
        fn crib_should_score_nobs() {
            assert_eq!(
                ScoreSheet::crib(&crib!("2D4H6HJH"), card!("TH")).points(),
                Points::from(1)
            );
            assert_eq!(
                ScoreSheet::crib(&crib!("2H4D6DJD"), card!("TH")).points(),
                Points::from(0)
            );
        }
    }

    /// ### Combinations

    /// In the above table, the word combination is used in the strict technical sense. Each and
    /// every combination of two cards that make a pair, of two or more cards that make 15, or of
    /// three or more cards that make a run, count separately.

    /// Example: A hand (including the starter) comprised of 8, 7, 7, 6, 2 scores 8 points for four
    /// combinations that total 15: the 8 with one 7, and the 8 with the other 7; the 6, 2 with each
    /// of the two 7s. The same hand also scores 2 for a pair, and 6 for two runs of three (8, 7, 6
    /// using each of the two 7s). The total score is 16. An experienced player computes the hand
    /// thus: "Fifteen 2, fifteen 4, fifteen 6, fifteen 8, and 8 for double run is 16."

    /// Note that the ace is always low and cannot form a sequence with a king. Further, a flush
    /// cannot happen during the play of the cards; it occurs only when the hands and the crib are
    /// counted.

    /// Certain basic formulations should be learned to facilitate counting. For pairs and runs
    /// alone:

    /// A. A triplet counts 6. A. Four of a kind counts 12. A. A run of three, with one card
    /// duplicated (double run) counts 8. A. A run of four, with one card duplicated, counts 10. A.
    /// A run of three, with one card triplicated (triple run), counts 15. A. A run of three, with
    /// two different cards duplicated, counts 16.
    mod combinations {
        use std::str::FromStr;

        use crate::{
            card,
            domain::{Card, Hand, Points, ScoreSheet},
            hand,
        };

        #[test]
        fn should_score_rules_example_eights_sevens_sixes() {
            assert_eq!(
                ScoreSheet::hand(&hand!("8H7C7D6S"), card!("2H")).points(),
                Points::from(16)
            );
        }

        #[test]
        fn should_score_rules_example_runs() {
            assert_eq!(
                ScoreSheet::hand(&hand!("JHQCKDAS"), card!("2D")).points(),
                Points::from(3)
            );
        }

        #[test]
        fn should_score_rules_example_flush() {
            assert_eq!(
                ScoreSheet::hand(&hand!("THQHKHAH"), card!("2H")).points(),
                Points::from(5)
            );
            assert_eq!(
                ScoreSheet::hand(&hand!("THQHKHAH"), card!("2S")).points(),
                Points::from(4)
            );
            assert_eq!(
                ScoreSheet::hand(&hand!("THQHKHAS"), card!("2H")).points(),
                Points::from(0)
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
        use std::str::FromStr;

        use crate::{
            card,
            domain::{Card, Hand, Points, ScoreSheet},
            hand,
        };

        #[test]
        fn should_score_rules_example_perfect_29() {
            assert_eq!(
                ScoreSheet::hand(&hand!("5H5C5DJS"), card!("5S")).points(),
                Points::from(29)
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
    mod internal {

        use crate::domain::test::GameBuilder;

        fn common_filters() -> insta::Settings {
            let mut settings = insta::Settings::new();
            settings.add_filter(
                r"\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}\.\d{1,9}",
                "<timestamp>",
            );
            settings.add_filter(
                r"UserId\([0-9a-f]{8}-([0-9a-f]{4}-){3}[0-9a-f]{12}\)",
                "<userid>",
            );
            settings.add_filter(r"Player\([0-1]\)", "<player>");
            settings.add_filter(r"(A|[2-9]|T|J|Q|K)(H|C|D|S)", "<card>");
            settings.add_filter(r"<card>(, <card>)*", "[<cards>]");
            settings.add_filter(r"\s*\d+ ->\s*\d+", "<score>");
            settings
        }

        #[test]
        fn should_output_user_readable_starting_game_in_logs() {
            let game = GameBuilder::default().with_cuts("ASAC").as_starting();
            common_filters().bind(|| {
                insta::assert_snapshot!(game.to_string(), @r"
                test-game__<timestamp> U[<cards>]
                <userid> <userid>
                Starting(
                    cuts: [<cards>]
                    deck: Deck([<cards>])
                    pending: Pending(<player>, <player>)
                )
                ")
            });
        }

        #[test]
        fn should_output_user_readable_discarding_game_in_logs() {
            let game = GameBuilder::default()
                .with_points(0, 0)
                .with_hands("AH2H3H4H5H6H", "AC2C3C4C5C6C")
                .as_discarding();
            common_filters().bind(|| {
                insta::assert_snapshot!(game.to_string(), @r"
                test-game__<timestamp> U[<cards>]
                <userid> <userid>
                Discarding(
                    scoreboard: Scoreboard(<score>,<score>)
                    roles: Dealer(<player>), Pone(<player>)
                    hands: Hand([<cards>]), Hand([<cards>])
                    crib: Crib()
                    deck: Deck([<cards>])
                    pending: Pending(<player>, <player>)
                )
                ")
            });
        }

        #[test]
        fn should_output_user_readable_playing_game_in_logs() {
            let game = GameBuilder::default()
                .with_points(0, 0)
                .with_hands("9S", "4S")
                .with_cut("AS")
                .with_current_plays(&[(0, "AH")])
                .as_playing(1);
            common_filters().bind(|| insta::assert_snapshot!(game.to_string(), @r"
                                     test-game__<timestamp> U[<cards>]
                                     <userid> <userid>
                                     Playing(
                                         scoreboard: Scoreboard(<score>,<score>),
                                         roles: Dealer(<player>), Pone(<player>),
                                         hands: Hand([<cards>]), Hand([<cards>]),
                                         play_state: Next(<player>), GoStatus(NotCalled), Pending(<player> -> [<cards>], <player> -> [<cards>]), Current((<player> -> [<cards>])), Previous(),
                                         cut: [<cards>],
                                         crib: Crib(),
                                         pending: Pending(<player>, <player>)
                                     )
                                     "));
        }

        #[test]
        fn should_output_user_readable_pone_scoring_game_in_logs() {
            let game = GameBuilder::default()
                .with_points(0, 0)
                .with_hands("AS2S3S4S", "AC2C3C4C")
                .with_cut("JH")
                .with_crib("TSJSQSKS")
                .as_scoring_pone();
            common_filters().bind(|| {
                insta::assert_snapshot!(game.to_string(), @r"
                test-game__<timestamp> U[<cards>]
                <userid> <userid>
                ScoringPone(
                    scoreboard: Scoreboard(<score>,<score>),
                    roles: Dealer(<player>), Pone(<player>),
                    hands: Hand([<cards>]), Hand([<cards>]),
                    cut: [<cards>],
                    crib: Crib([<cards>]),
                    pegging: <player> -> Fifteen: ([<cards>]) -> 2, Fifteen: ([<cards>]) -> 2, Run: ([<cards>]) -> 4, Flush: ([<cards>]) -> 4,
                    pending: Pending(<player>, <player>)
                )
                ")
            });
        }

        #[test]
        fn should_output_user_readable_dealer_scoring_game_in_logs() {
            let game = GameBuilder::default()
                .with_points(0, 0)
                .with_hands("AS2S3S4S", "AC2C3C4C")
                .with_cut("JH")
                .with_crib("TSJSQSKS")
                .as_scoring_dealer();
            common_filters().bind(|| {
                insta::assert_snapshot!(game.to_string(), @r"
                test-game__<timestamp> U[<cards>]
                <userid> <userid>
                ScoringDealer(
                    scoreboard: Scoreboard(<score>,<score>),
                    roles: Dealer(<player>), Pone(<player>),
                    hands: Hand([<cards>]), Hand([<cards>]),
                    cut: [<cards>],
                    crib: Crib([<cards>]),
                    pegging: <player> -> Fifteen: ([<cards>]) -> 2, Fifteen: ([<cards>]) -> 2, Run: ([<cards>]) -> 4, Flush: ([<cards>]) -> 4,
                    pending: Pending(<player>, <player>)
                )
                ")
            });
        }

        #[test]
        fn should_output_user_readable_crib_scoring_game_in_logs() {
            let game = GameBuilder::default()
                .with_points(0, 0)
                .with_hands("AS2S3S4S", "AC2C3C4C")
                .with_cut("JH")
                .with_crib("TSJSQSKS")
                .as_scoring_crib();
            common_filters().bind(|| {
                insta::assert_snapshot!(game.to_string(), @r"
                test-game__<timestamp> U[<cards>]
                <userid> <userid>
                ScoringCrib(
                    scoreboard: Scoreboard(<score>,<score>),
                    roles: Dealer(<player>), Pone(<player>),
                    hands: Hand([<cards>]), Hand([<cards>]),
                    cut: [<cards>],
                    crib: Crib([<cards>]),
                    pegging: <player> -> Pair: ([<cards>]) -> 2, Run: ([<cards>]) -> 4, Run: ([<cards>]) -> 4,
                    pending: Pending(<player>, <player>)
                )
                ")
            });
        }

        #[test]
        fn should_output_user_readable_finished_game_in_logs() {
            let game = GameBuilder::default()
                .with_points(0, 121)
                .with_winner(1)
                .with_hands("AS2S3S4S", "AC2C3C4C")
                .with_cut("JH")
                .with_crib("TSJSQSKS")
                .as_finished();
            common_filters().bind(|| {
                insta::assert_snapshot!(game.to_string(), @r"
                test-game__<timestamp> U[<cards>]
                <userid> <userid>
                Finished(
                    winner: <player>,
                    scoreboard: Scoreboard(<score>,<score>),
                    roles: Dealer(<player>), Pone(<player>),
                    hands: Hand([<cards>]), Hand([<cards>]),
                    crib: Crib([<cards>]),
                    cut: [<cards>]
                )
                ")
            });
        }
    }
}
