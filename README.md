# cribbage

[![BSD 3 Clause License](https://img.shields.io/github/license/nigeleke/cribbage?style=plastic)](https://github.com/nigeleke/cribbage/blob/master/LICENSE)
[![Language](https://img.shields.io/badge/language-Rust-blue.svg?style=plastic)](https://www.rust-lang.org/)
[![Build](https://img.shields.io/github/actions/workflow/status/nigeleke/cribbage/ci.yml?style=plastic)](https://github.com/nigeleke/cribbage/)
[![Coverage](https://img.shields.io/codecov/c/github/nigeleke/cribbage?style=plastic)](https://codecov.io/gh/nigeleke/cribbage)
![Version](https://img.shields.io/github/v/tag/nigeleke/cribbage?style=plastic)

  [Site](https://nigeleke.github.io/cribbage) \| [GitHub](https://github.com/nigeleke/cribbage) \| [API](https://nigeleke.github.io/cribbage/api/index.html) \| [Coverage Report](https://nigeleke.github.io/cribbage/coverage/index.html)

[Cribbage](https://en.wikipedia.org/wiki/Cribbage) is a popular card game, predominately played by two players.

## Background

This project has had many flavours over time. It is being used as a learning platform, initially to implement a practical example using [Akka](https://akka.io/), [CQRS](https://martinfowler.com/bliki/CQRS.html), [Domain Driven Design](https://martinfowler.com/tags/domain%20driven%20design.html), [Event Sourcing](https://martinfowler.com/eaaDev/EventSourcing.html), and [Event Storming](https://www.eventstorming.com/). Later I applied pure functional programming using the [Cats Effects](https://typelevel.org/cats-effect/) stack.

The project then moved to [Rust](https://www.rust-lang.org/) / [Leptos](https://www.leptos.dev/), and now [Rust](https://www.rust-lang.org/) / [Dioxus](https://dioxuslabs.com/).

The project is under active development as of Dec 2025.

## Testing

```bash
DATABASE_URL=postgres://postgres:password@localhost:5432/cribbage
```

```bash
docker-compose -f docker/docker_compose.yml up -d
cargo test --all-features
```

## Build

```bash
docker-compose -f docker/docker_compose.yml up -d
dx build --package=entrypoint [platform]
  where [platform] is
    --desktop
    --mobile
    --web
```

## Run

```bash
docker-compose -f docker/docker_compose.yml up -d
dx serve --package=entry [platform]
  where [platform] is
    --desktop
    --web
```

Navigate to:
  - [app](http://localhost:8080/)
  - [pgadmin](http://localhost:8181/)

## Notes

* This is not an example of authorisation and / or security.
  A "user-id" is simply persisted in browser storage and passed around as-is.
