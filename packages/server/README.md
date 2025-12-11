# server

Core server crate for managing server side logic and services for the Cribbage games.

This crate provides the server-side logic for creating, hosting, and managing Cribbage games.
It includes modules for domain logic, database access, projections, services, and server
state management.

---

## Features

  - **Game Management**: Create, join, and start Cribbage games.
  - **Player Actions**: Play cards, go, discard to crib, and score hands.
  - **Server State**: Centralized `ServerState` managing active games and player sessions.
  - **Event Streams**: Real-time updates for available games and individual game states.
  - **Domain Logic**: Types and traits representing players, hands, scores, and game phases.
  - **Error Handling**: Error types via `ServerError`.
  - **Persistence**: Database integration (Postgres).

---

## Modules

  - `domain` - Types and traits for Cribbage domain logic, such as `Player`, `Hand`, `ScoreSheet`, and `Phase`.
  - `error` - Error types returned by server operations.
  - `server_state` - Server state management, including `ServerState` and initialization helpers.
  - `services` - Async functions for interacting with games.

Internal modules (not part of the public API):

  - `convertors` - Helper conversions between database types and domain types.
  - `database` - Database interaction and change notifications.
  - `display` - Formatting and view helpers for output or logs.
  - `name_builder` - Generates user-facing game or player names.
  - `projections` - Constructs derived views of the game state.
