#[macro_export]
macro_rules! game {
    ($state:expr, scoreboard: $scoreboard:expr $(,)?) => {
        Game {
            scoreboard: $scoreboard,
            state: $state,
        }
    };

    ($state:expr $(,)?) => {
        Game {
            scoreboard: Scoreboard::default(),
            state: $state,
        }
    };
}

#[macro_export]
macro_rules! starting {
    ($deck:expr, $cuts:expr) => {
        crate::game::Starting {
            deck: $deck,
            cuts: $cuts,
        }
    };
}

#[macro_export]
macro_rules! dealing {
    ($roles:expr) => {
        crate::game::Dealing { roles: $roles }
    };
}

#[macro_export]
macro_rules! discarding {
    (
        $roles:expr,
        $hands:expr,
        $deck:expr,
        $discards:expr
    ) => {
        crate::game::Discarding {
            roles: $roles,
            hands: $hands,
            deck: $deck,
            discards: $discards,
        }
    };
}

#[macro_export]
macro_rules! cutting {
    (
        $roles:expr,
        $hands:expr,
        $crib:expr,
        $deck:expr
    ) => {
        crate::game::Cutting {
            roles: $roles,
            hands: $hands,
            crib: $crib,
            deck: $deck,
        }
    };
}

#[macro_export]
macro_rules! playing {
    (
        $roles:expr,
        $hands:expr,
        $crib:expr,
        $starter:expr,
        $next_to_play:ident
        ) => {{
        let play_state = crate::PlayState::new(crate::Player::$next_to_play, &$hands);
        crate::game::Playing {
            roles: $roles,
            hands: $hands,
            crib: $crib,
            starter: $starter,
            play_state,
        }
    }};

    (
        $roles:expr,
        $hands:expr,
        $crib:expr,
        $starter:expr,
        $next_to_play:ident,
        go_status: $go_status:ident
        ) => {{
        let play_state = crate::PlayState::new(crate::Player::$next_to_play, &$hands)
            .with_go_status(crate::GoStatus::$go_status);
        crate::game::Playing {
            roles: $roles,
            hands: $hands,
            crib: $crib,
            starter: $starter,
            play_state,
        }
    }};

    (
        $roles:expr,
        $hands:expr,
        $crib:expr,
        $starter:expr,
        $next_to_play:ident,
        go_status: $go_status:ident,
        current: $current_plays:expr,
        previous: $previous_plays:expr
        ) => {{
        let play_state = crate::PlayState::new(crate::Player::$next_to_play, &$hands)
            .with_go_status(crate::GoStatus::$go_status)
            .with_current_plays($current_plays)
            .with_previous_plays($previous_plays);
        crate::game::Playing {
            roles: $roles,
            hands: $hands,
            crib: $crib,
            starter: $starter,
            play_state,
        }
    }};
}

#[macro_export]
macro_rules! scoring_pone {
    (
        $roles:expr,
        $hands:expr,
        $crib:expr,
        $starter:expr
    ) => {
        ScoringPone {
            roles: $roles,
            hands: $hands,
            crib: $crib,
            starter: $starter,
            _marker: std::marker::PhantomData,
        }
    };
}

#[macro_export]
macro_rules! scoring_dealer {
    (
        $roles:expr,
        $hands:expr,
        $crib:expr,
        $starter:expr
    ) => {
        ScoringDealer {
            roles: $roles,
            hands: $hands,
            crib: $crib,
            starter: $starter,
            _marker: std::marker::PhantomData,
        }
    };
}

#[macro_export]
macro_rules! scoring_crib {
    (
        $roles:expr,
        $hands:expr,
        $crib:expr,
        $starter:expr
    ) => {
        ScoringCrib {
            roles: $roles,
            hands: $hands,
            crib: $crib,
            starter: $starter,
            _marker: std::marker::PhantomData,
        }
    };
}

#[macro_export]
macro_rules! finished {
    (
        $roles:expr,
        $hands:expr,
        $crib:expr,
        $starter:expr,
    ) => {
        crate::game::Finished {
            roles: $roles,
            hands: $hands,
            crib: $crib,
            starter: $starter,
            play_state: None,
        }
    };

    (
        $roles:expr,
        $hands:expr,
        $crib:expr,
        $starter:expr,
        $next_to_play:ident,
        go_status: $go_status:ident,
        current: $current_plays:expr,
        previous: $previous_plays:expr
    ) => {
        crate::game::Finished {
            roles: $roles,
            hands: $hands,
            crib: $crib,
            starter: $starter,
            play_state: Some(
                crate::PlayState::new(crate::Player::$next_to_play, &$hands)
                    .with_go_status(crate::GoStatus::$go_status)
                    .with_current_plays($current_plays)
                    .with_previous_plays($previous_plays),
            ),
        }
    };
}
