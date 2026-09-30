# domain

The `domain` package contains the core cribbage domain model and rules for the game of cribbage.

## Summary

The domain defines:

* cards, faces, suits, ranks, and card values
* hands, cribs, decks, and other card collections
* players
* game phases and state
* plays and scoring
* validation and domain errors
* the rules governing the game of cribbage

## Determinism and randomness

The crate is intentionally free of randomness. All card orderings and
cuts are supplied by the caller. Given the same sequence of commands and
the same initial deck, every state transition is reproducible.

Callers are responsible for shuffling (e.g. `Deck::new().shuffled(rng)`)
before passing a deck into [`Game::deal`] or choosing cards for
[`Game::cut_for_deal`] / starter cuts. The domain only enforces rules
and validates that supplied cards are legal in the current state.
