# cribbage

[![BSD 3 Clause License](https://img.shields.io/github/license/nigeleke/cribbage?style=plastic)](https://github.com/nigeleke/cribbage/blob/master/LICENSE)
[![Language](https://img.shields.io/badge/language-Rust-blue.svg?style=plastic)](https://www.rust-lang.org/)
[![Build](https://img.shields.io/github/actions/workflow/status/nigeleke/cribbage/acceptance.yml?style=plastic)](https://github.com/nigeleke/cribbage/actions/workflows/acceptance.yml)
[![Coverage](https://img.shields.io/codecov/c/github/nigeleke/cribbage?style=plastic)](https://codecov.io/gh/nigeleke/cribbage)
![Version](https://img.shields.io/github/v/tag/nigeleke/cribbage?style=plastic)

  [Site](https://nigeleke.github.io/cribbage) \| [GitHub](https://github.com/nigeleke/cribbage) \| [API](https://nigeleke.github.io/cribbage/api/index.html) \| [Coverage Report](https://nigeleke.github.io/cribbage/coverage/index.html)

[Cribbage](https://en.wikipedia.org/wiki/Cribbage) is a popular card game, predominately played by two players.

## Background

This project has had many flavours over time. It is being used as a learning platform, initially to implement a practical example using [Akka](https://akka.io/), [CQRS](https://martinfowler.com/bliki/CQRS.html), [Domain Driven Design](https://martinfowler.com/tags/domain%20driven%20design.html), [Event Sourcing](https://martinfowler.com/eaaDev/EventSourcing.html), and [Event Storming](https://www.eventstorming.com/). Later I applied pure functional programming using the [Cats Effects](https://typelevel.org/cats-effect/) stack.

The project has now completely moved to [Rust](https://www.rust-lang.org/) using [Leptos](https://www.leptos.dev/) (server-side rendering).

The project is under active development as of May 2024.

## Development

```
cargo test
```

## Run

```
cargo leptos watch
```

