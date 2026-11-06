use super::*;

use std::assert_matches;

use crate::test_support::EntityFixture;
use crate::user::UserId;

#[test]
fn host_enters_lobby() {
    let id = LobbyId::new();
    let lobby = Lobby::new(id);
    let host = UserId::new();

    EntityFixture::new(lobby)
        .when(LobbyCommand::Create { host })
        .then(|actual| {
            assert_matches!(
                actual,
                LobbyState::WaitingForGuest { host: actual_host, .. }
                    if actual_host == &host,
                "expected WaitingForGuest with the specified host, got {actual:?}");
        });
}

#[test]
fn guest_enters_lobby() {
    let id = LobbyId::new();
    let lobby = Lobby::new(id);
    let host = UserId::new();
    let guest = UserId::new();

    EntityFixture::new(lobby)
        .given_event(LobbyEvent::HostEntered { host })
        .when(LobbyCommand::Join { guest })
        .then(|actual| {
            assert_matches!(
                actual,
                LobbyState::Paired {
                    host: actual_host,
                    guest: actual_guest,
                    ..
                } if actual_host == &host && actual_guest == &guest,
                "expected Paired with the specified host and guest, got {actual:?}"
            );
        });
}

#[test]
fn host_cannot_enter_lobby_twice() {
    let id = LobbyId::new();
    let lobby = Lobby::new(id);
    let host = UserId::new();

    EntityFixture::new(lobby)
        .given_event(LobbyEvent::HostEntered { host })
        .when(LobbyCommand::Create { host })
        .then_error(|error| {
            assert_matches!(
        error,
        LobbyError::InvalidHost(actual_host) if actual_host == &host,
        "expected InvalidHost with the specified host, got {error:?}")
        });
}

#[test]
fn host_cannot_join_their_lobby() {
    let id = LobbyId::new();
    let lobby = Lobby::new(id);
    let host = UserId::new();

    EntityFixture::new(lobby)
        .given_event(LobbyEvent::HostEntered { host })
        .when(LobbyCommand::Join { guest: host })
        .then_error(|error| {
            assert_matches!(
            error,
            LobbyError::InvalidGuest(actual_host) if actual_host == &host,
            "expected InvalidGuest with the specified host, got {error:?}")
        });
}

#[test]
fn guest_cannot_join_lobby_twice() {
    let id = LobbyId::new();
    let lobby = Lobby::new(id);
    let host = UserId::new();
    let guest = UserId::new();

    EntityFixture::new(lobby)
        .given([
            LobbyEvent::HostEntered { host },
            LobbyEvent::GuestJoined { guest },
        ])
        .when(LobbyCommand::Join { guest })
        .then_error(|error| {
            assert_matches!(
                error,
                LobbyError::LobbyFull,
                "expected LobbyFull, got {error:?}"
            )
        });
}

#[test]
fn another_guest_cannot_join_lobby() {
    let id = LobbyId::new();
    let lobby = Lobby::new(id);
    let host = UserId::new();
    let guest0 = UserId::new();
    let guest1 = UserId::new();

    EntityFixture::new(lobby)
        .given([
            LobbyEvent::HostEntered { host },
            LobbyEvent::GuestJoined { guest: guest0 },
        ])
        .when(LobbyCommand::Join { guest: guest1 })
        .then_error(|error| {
            assert_matches!(
                error,
                LobbyError::LobbyFull,
                "expected LobbyFull, got {error:?}"
            )
        });
}

#[test]
fn guest_cannot_join_empty_lobby() {
    let id = LobbyId::new();
    let lobby = Lobby::new(id);
    let guest = UserId::new();

    EntityFixture::new(lobby)
        .when(LobbyCommand::Join { guest })
        .then_error(|error| {
            assert_matches!(
                error,
                LobbyError::LobbyEmpty,
                "expected LobbyError, got {error:?}"
            )
        });
}
