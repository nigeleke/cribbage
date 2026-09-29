# tellus

The tellus application layer for Cribbage.

`cribbage-tellus` sits between the pure Cribbage domain and the application/API layers.
It uses [tellus](https://github.com/hseeberger/tellus.git) to manage game actors, events,
persistence, and projections while keeping the rules and state transitions in `cribbage-domain`.

## Summary

`cribbage-tellus` is responsible for:

* Actors representing Cribbage games
* Mapping application commands to domain operations
* Raising events from domain outcomes
* Persisting game state and events
* Maintaining projections/read models
* Associating application `UserId`s with domain `Player`s
* Enforcing application-level game lifecycle rules

It is **not** responsible for implementing the rules of Cribbage. Those belong in `cribbage-domain`.
