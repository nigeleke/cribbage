# cribbage

[![BSD 3 Clause License](https://img.shields.io/github/license/nigeleke/cribbage?style=plastic)](https://github.com/nigeleke/cribbage/blob/main/LICENSE)
[![Language](https://img.shields.io/badge/language-Rust-blue.svg?style=plastic)](https://www.rust-lang.org/)
[![Build](https://img.shields.io/github/actions/workflow/status/nigeleke/cribbage/ci.yml?style=plastic)](https://github.com/nigeleke/cribbage/)
[![Coverage](https://img.shields.io/codecov/c/github/nigeleke/cribbage?style=plastic)](https://codecov.io/gh/nigeleke/cribbage)
![Version](https://img.shields.io/github/v/tag/nigeleke/cribbage?style=plastic)

  [Site](https://nigeleke.github.io/cribbage) \| [GitHub](https://github.com/nigeleke/cribbage) \| [Coverage Report](https://nigeleke.github.io/cribbage/llvm-cov/index.html)

[Cribbage](https://en.wikipedia.org/wiki/Cribbage) is a popular card game, predominately played by two players.

## Background

An example of a fullstack [cribbage](https://en.wikipedia.org/wiki/Cribbage) playing program, allowing games to be played with remote users.

It has been created as "a bit of fun" and, as of Dec 2025, the project is essentially complete, and now presents as an example project for general perusal. I'm always learning and welcome feedback any aspects of the project, especially if there are ways to make things simpler and / or clearer. If you're interested in helping me along that learning path please make any comments through the github issues page.

  * Rust: core programming language.
  * Dioxus 0.7: cross-platform UI framework; including some Dioxus Components.
  * Multiple workspaces: web, ui, api, server.
  * Event sourcing: CQRS-ES
  * PostgreSQL: persisted events and aggregates.
  * Docker Compose: containerization for local development.

## Packages

```mermaid
graph TD
    dioxus["dioxus 0.7"] 

    web["web"]
    ui["ui"] 
    api["api"]
    server["server"]

    web --> dioxus
    web --> ui
    web --> api

    ui --> dioxus
    ui --> api
    
    api --> dioxus
    api --> server
```

## Project Documentation
  * [constants](doc/constants/index.html)
  * [server](doc/server/index.html)
  * [aip](doc/api/index.html)
  * [ui](doc/ui/index.html)
  * [web](doc/web/index.html)

## Setup

Note: git pre-commit checks require active database because the following commands are executed:

  * `cargo sqlx migrate run`
  * `cargo sqlx prepare`

```bash
DATABASE_URL=postgres://postgres:password@localhost:5432/cribbage
docker-compose -f docker/compose.yml up -d
cd packages/server
cargo sqlx migrate run
```

## Testing

```bash
cargo test --all-features
```

## Build

```bash
dx build --package=web
```

## Run

```bash
dx serve --package=web
```

Navigate to:
  - [app](http://localhost:8080/)
  - [pgadmin](http://localhost:8181/)

## Notes

* This is not an example of authorisation and / or security.
  A "user-id" is simply persisted in browser storage and passed around as-is.

## History

This project has been used as a learning platform and had many flavours over time. The [Rust](https://www.rust-lang.org/) / [Dioxus](https://dioxuslabs.com/) combination proved the most productive and sucessful.

The key interest revolved around:

  * [CQRS](https://martinfowler.com/bliki/CQRS.html)
  * [Domain Driven Design](https://martinfowler.com/tags/domain%20driven%20design.html)
  * [Event Sourcing](https://martinfowler.com/eaaDev/EventSourcing.html), and,
  * [Event Storming](https://www.eventstorming.com/).

Previous technologies included [Scala](https://www.scala-lang.org/), [Akka](https://akka.io/) (now [Pekko](https://pekko.apache.org/)), and later trying out the [Cats Effects](https://typelevel.org/cats-effect/) stack.

The project then moved to [Rust](https://www.rust-lang.org/) initially using [Leptos](https://www.leptos.dev/) before finally committing to [Dioxus](https://dioxuslabs.com/).

__I have to raise my hat to the [Dioxus team](https://github.com/DioxusLabs) and [community](https://discord.gg/XgGxMSkvUM) as a whole for being incredibly supportive, proactive, reactive and helpful with any questions / queries I had during the development.__
