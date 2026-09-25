use crate::domain::{DomainError, Game, GameCommand, GameEvent, GameServices, Phase};

pub(crate) fn __private_game_test_impl(
    function_name: String,
    given: &[GameEvent],
    when: GameCommand,
    then_events: Option<impl Fn(&[GameEvent])>,
    then_state: Option<impl Fn(&Phase)>,
    then_error: Option<DomainError>,
) {
    let is_happy_path_tests = then_events.is_some() || then_state.is_some();
    let is_error_path_tests = then_error.is_some();

    assert!(
        (is_happy_path_tests && !is_error_path_tests)
            || (is_error_path_tests && !is_happy_path_tests)
    );

    let mut game = Game::from(given);

    let result = cqrs_es::test::TestFramework::<Game>::with(GameServices)
        .given(Vec::from(given))
        .when(when)
        .inspect_result();

    match result {
        Ok(events) if is_happy_path_tests => {
            then_events.iter().for_each(|f| f(events.as_slice()));
            game.apply_events(&events);
            let state = game.phase();
            then_state.iter().for_each(|f| f(state));
        }
        Ok(events) => panic!("unexpected result in {function_name}: {events:?}"),
        Err(error) if is_error_path_tests => {
            then_error
                .into_iter()
                .for_each(|e| assert_eq!(error, e, "in function {function_name}"));
        }
        Err(error) => panic!("unexpected result in {function_name}: {error:?}"),
    };
}
