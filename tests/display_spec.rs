use cribbage::prelude::*;

mod builder;
use builder::GameBuilder;

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
            Hand::from("KS5S").as_ref(),
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
    common_filters().bind(|| insta::assert_snapshot!(game.to_string(), @"Finished(winner: <playerid>, peggings: <playerid> -> <score>, <playerid> -> <score>)"));
}
