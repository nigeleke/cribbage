ThisBuild / organization := "com.nigeleke"
ThisBuild / scalaVersion := "2.13.2"
ThisBuild / version      := "0.1-SNAPSHOT"

// License
ThisBuild / organizationName := "Nigel Eke"
ThisBuild / startYear := Some(2020)
ThisBuild / licenses += ("AGPL-3.0-or-later", new URL("https://www.gnu.org/licenses/agpl-3.0.txt"))

val akkaVersion = "2.6.9"
val akkaHttpVersion = "10.2.0"
val akkaPersistenceJdbcVersion = "4.0.0"
val inMemoryPersistenceVersion = "2.5.15.2"
val logbackClassicVersion = "1.2.3"
val scalaTestVersion = "3.1.2"

lazy val root = (project in file("."))
  .settings(
    name := "cribbage"
  )
  .aggregate(backend, model, api)

lazy val commonSettings = Seq(
  scalacOptions in Compile ++= Seq("-deprecation", "-feature", "-unchecked", "-Xlog-reflective-calls", "-Xlint"),
  scalacOptions in Test ++= Seq("-deprecation", "-feature", "-unchecked", "-Xlog-reflective-calls", "-Xlint"),
  javacOptions in Compile ++= Seq("-Xlint:unchecked", "-Xlint:deprecation")
)

lazy val backend = (project in file("backend"))
  .settings(
    commonSettings,
    libraryDependencies ++= Seq(
      "ch.qos.logback" % "logback-classic" % logbackClassicVersion,
      "com.typesafe.akka" %% "akka-actor-typed" % akkaVersion,
      "com.typesafe.akka" %% "akka-persistence-query" % akkaVersion,
      "com.typesafe.akka" %% "akka-persistence-typed" % akkaVersion,
      "com.typesafe.akka" %% "akka-serialization-jackson" % akkaVersion,
      "com.github.dnvriend" %% "akka-persistence-inmemory" % inMemoryPersistenceVersion % "test",
      "com.typesafe.akka" %% "akka-actor-testkit-typed" % akkaVersion % "test",
      "com.typesafe.akka" %% "akka-persistence-testkit" % akkaVersion % "test",
      "org.scalatest" %% "scalatest" % scalaTestVersion % "test"
    )
  )
  .dependsOn(model)

lazy val model = (project in file("model"))
  .settings(
    commonSettings,
    libraryDependencies ++= Seq(
      "org.scalatest" %% "scalatest" % scalaTestVersion % "test"
    )
  )

lazy val api = (project in file("api"))
  .settings(
    commonSettings,
    libraryDependencies ++= Seq(
      "com.typesafe.akka" %% "akka-actor-typed" % akkaVersion,
      "com.typesafe.akka" %% "akka-cluster-typed" % akkaVersion,
      "com.typesafe.akka" %% "akka-http-spray-json" % akkaHttpVersion,
      "com.typesafe.akka" %% "akka-http" % akkaHttpVersion,
      "com.typesafe.akka" %% "akka-stream" % akkaVersion,
      "com.typesafe.akka" %% "akka-actor-testkit-typed" % akkaVersion % "test",
      "com.typesafe.akka" %% "akka-http-testkit"        % akkaHttpVersion % "test",
      "org.scalatest" %% "scalatest" % scalaTestVersion % "test"
    )
  )
  .dependsOn(backend)
