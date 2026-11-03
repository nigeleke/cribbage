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
            let LobbyState::WaitingForGuest {
                host: actual_host, ..
            } = actual
            else {
                panic!("expected WaitingForGuest");
            };
            assert_eq!(actual_host, &host);
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
            let LobbyState::Paired {
                host: actual_host,
                guest: actual_guest,
                ..
            } = actual
            else {
                panic!("expected WaitingForGuest");
            };
            assert_eq!(actual_host, &host);
            assert_eq!(actual_guest, &guest);
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
        .then(|actual| assert_matches!(actual, LobbyState::WaitingForGuest { .. }));
}

#[test]
fn host_cannot_join_their_lobby() {
    let id = LobbyId::new();
    let lobby = Lobby::new(id);
    let host = UserId::new();

    EntityFixture::new(lobby)
        .given_event(LobbyEvent::HostEntered { host })
        .when(LobbyCommand::Join { guest: host })
        .then(|actual| assert_matches!(actual, LobbyState::WaitingForGuest { .. }));
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
        .then(|actual| assert_matches!(actual, LobbyState::Paired { .. }));
}

#[test]
fn guest_cannot_join_empty_lobby() {
    let id = LobbyId::new();
    let lobby = Lobby::new(id);
    let guest = UserId::new();

    EntityFixture::new(lobby)
        .when(LobbyCommand::Join { guest })
        .then(|actual| assert_matches!(actual, LobbyState::Unpaired));
}
