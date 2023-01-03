# Cribbage

[![BSD 3 Clause License](https://img.shields.io/github/license/nigeleke/cribbage?style=plastic)](https://github.com/nigeleke/cribbage/blob/master/LICENSE)
[![Language](https://img.shields.io/badge/language-Scala-blue.svg?style=plastic)](https://www.scala-lang.org)
[![Build](https://img.shields.io/github/actions/workflow/status/nigeleke/cribbage/acceptance.yml?style=plastic)](https://github.com/nigeleke/cribbage/actions/workflows/acceptance.yml)
[![Coverage](https://img.shields.io/badge/dynamic/xml?style=plastic&color=success&label=coverage&query=%28%2Fscoverage%2F%40statement-rate%20%2B%20%2Fscoverage%2F%40branch-rate%29%20div%202&suffix=%20%25&url=https%3A%2F%2Fnigeleke.github.io%2Fcribbage%2Fcoverage%2Fscoverage.xml)](https://nigeleke.github.io/cribbage/coverage)
![Version](https://img.shields.io/github/v/tag/nigeleke/cribbage?style=plastic)

[Cribbage](https://en.wikipedia.org/wiki/Cribbage) is a popular card state, predominately played by two players.

This project has been developed as a learning exercise, using Cribbage as the example domain. It started with the following aims:

  - [Akka](https://akka.io/) and its related libraries
  - [CQRS](https://martinfowler.com/bliki/CQRS.html) - Command Query Responsibility Segregation
  - [Domain Driven Design](https://martinfowler.com/tags/domain%20driven%20design.html) 
  - [Event Sourcing](https://martinfowler.com/eaaDev/EventSourcing.html), and
  - [Event Storming](https://www.eventstorming.com/)

The [Akka](https://akka.io/) implementation has been abandoned in favour of the [Cats Effects](https://typelevel.org/cats-effect/) stack. As of Jan 2023, this is still under development.

## Documentation

* [Site](https://nigeleke.github.io/cribbage)
* [GitHub](https://github.com/nigeleke/cribbage)
* [API](https://nigeleke.github.io/cribbage/api/index.html)
* [Coverage Report](https://nigeleke.github.io/cribbage/coverage/index.html)
