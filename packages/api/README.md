# api

This crate exposes the HTTP API for the game server. Its main responsibilities are:

  1. Exposing HTTP endpoints for clients (web or other consumers).
  2. Providing DTOs (Data Transfer Objects) for communication.
  3. Translating between DTOs and server domain objects.
  4. Calling the appropriate server services to perform game logic.

## Modules
  - `dto`: Defines Data Transfer Objects for requests and responses.
  - `error`: Defines `ApiError` and other API-level errors.
  - `services`: Implements the core business logic for API endpoints.

## Server Integration

When compiled with the `server` feature, the crate exposes:

  - `ServerStateExtractor`: a wrapper around `server::ServerState` for use by the API.
  - `initialize_server_state`: function to initialize the shared server state.
