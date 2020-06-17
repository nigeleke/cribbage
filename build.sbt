import Versions._

ThisBuild / organization := "com.nigeleke"
ThisBuild / scalaVersion := "2.13.2"
ThisBuild / version      := "0.1-SNAPSHOT"

lazy val root = (project in file("."))
  .settings(
    name := "cribbage"
  )
  .aggregate(backend, model)

lazy val backend = project
  .settings(
    libraryDependencies ++= Seq(
      "com.typesafe.akka" %% "akka-actor-typed" % akkaVersion,
      "ch.qos.logback" % "logback-classic" % logbackVersion % "runtime",
      "com.typesafe.akka" %% "akka-actor-testkit-typed" % akkaVersion % "test",
      "org.scalatest" %% "scalatest" % scalaTestVersion % "test"
    )
  )
  .dependsOn(model)

lazy val model = project
  .settings(
    libraryDependencies ++= Seq(
      "org.scalatest" %% "scalatest" % scalaTestVersion % "test"
    )
  )
