macro_rules! scenario {
    ( $final_method:ident ( $($final_arg:expr),* $(,)? ) ; $($method:ident( $($arg:expr),* $(,)? )),* $(,)? ) => {{
        let builder = GameBuilder::default()
            $( . $method ( $($arg),* ) )* ;

        let mut game = builder. $final_method ( $($final_arg),* );
        *game.name_mut() = $crate::macros::function_name!();
        vec![GameEvent::GamePreloaded { game }]
    }};

    ( $final_method:ident ; $($method:ident( $($arg:expr),* $(,)? )),* $(,)? ) => {{
        let builder = GameBuilder::default()
            $( . $method ( $($arg),* ) )* ;

        let mut game = builder. $final_method ();
        *game.name_mut() = $crate::macros::function_name!();
        vec![GameEvent::GamePreloaded { game }]
    }};
}

macro_rules! find_then {
    ($slice:expr, $pat:pat => $assert:block) => {
        #[allow(unused)]
        if let Some(event) = $slice.iter().find(|e| matches!(e, $pat)) {
            if let $pat = event {
                $assert
            } else {
                unreachable!()
            }
        } else {
            panic!("expected {} not found", stringify!($pat));
        }
    };
}

macro_rules! assert_state_then {
    ($state:expr, $pat:pat $(if $guard:expr)? => $assert:block) => {{
        match $state {
            $pat $(if $guard)? => $assert,
            other => panic!("unexpected state: {:?}", other),
        }
    }};
}

macro_rules! game_test {
    {
        given: $given:expr,
        when: $when:expr,
        then_events: $events_fn:expr,
        then_state: $state_fn:expr
    } => {{
        $crate::domain::test::__private_game_test_impl(
            $crate::macros::function_name!(),
            $given,
            $when,
            Some($events_fn),
            Some($state_fn),
            None::<DomainError>
        );
    }};

    {
        given: $given:expr,
        when: $when:expr,
        then_events: $events_fn:expr
    } => {{
        $crate::domain::test::__private_game_test_impl(
            $crate::macros::function_name!(),
            $given,
            $when,
            Some($events_fn),
            None::<fn(&Phase)>,
            None::<DomainError>
        );
    }};

    {
        given: $given:expr,
        when: $when:expr,
        then_state: $state_fn:expr
    } => {{
        $crate::domain::test::__private_game_test_impl(
            $crate::macros::function_name!(),
            $given,
            $when,
            None::<fn(&[GameEvent])>,
            Some($state_fn),
            None::<DomainError>
        );
    }};

    {
        given: $given:expr,
        when: $when:expr,
        then_error: $error:expr
    } => {{
        $crate::domain::test::__private_game_test_impl(
            $crate::macros::function_name!(),
            $given,
            $when,
            None::<fn(&[GameEvent])>,
            None::<fn(&Phase)>,
            Some($error)
        );
    }};

    {
        when: $when:expr,
        then_events: $events_fn:expr,
        then_state: $state_fn:expr
    } => {{
        $crate::domain::test::__private_game_test_impl(
            $crate::macros::function_name!(),
            &[],
            $when,
            Some($events_fn),
            Some($state_fn),
            None::<DomainError>
        );
    }};

    {
        when: $when:expr,
        then_events: $events_fn:expr
    } => {{
        $crate::domain::test::__private_game_test_impl(
            $crate::macros::function_name!(),
            &[],
            $when,
            Some($events_fn),
            None::<fn(&Phase)>,
            None::<DomainError>
        );
    }};

    {
        when: $when:expr,
        then_state: $state_fn:expr
    } => {{
        $crate::domain::test::__private_game_test_impl(
            $crate::macros::function_name!(),
            &[],
            $when,
            None::<fn(&[GameEvent])>,
            Some($state_fn),
            None::<DomainError>
        );
    }};

    {
        when: $when:expr,
        then_error: $error:expr
    } => {{
        $crate::domain::test::__private_game_test_impl(
            $crate::macros::function_name!(),
            &[],
            $when,
            None::<fn(&[GameEvent])>,
            None::<fn(&Phase)>,
            Some($error)
        );
    }};
}

pub(crate) use { scenario, find_then, assert_state_then, game_test };
