ThisBuild / organization := "com.nigeleke"
ThisBuild / scalaVersion := "2.13.2"
ThisBuild / version      := "0.1-SNAPSHOT"

// License
ThisBuild / organizationName := "Nigel Eke"
ThisBuild / startYear := Some(2020)
ThisBuild / licenses += ("AGPL-3.0-or-later", new URL("https://www.gnu.org/licenses/agpl-3.0.txt"))

val akkaVersion = "2.6.8"
val logbackVersion = "1.2.3"
val scalaTestVersion = "3.1.2"

lazy val root = (project in file("."))
  .settings(
    name := "cribbage"
  )
  .aggregate(backend, model)

lazy val backend = project
  .settings(
    scalacOptions in Compile ++= Seq("-deprecation", "-feature", "-unchecked", "-Xlog-reflective-calls", "-Xlint"),
    scalacOptions in Test ++= Seq("-deprecation", "-feature", "-unchecked", "-Xlog-reflective-calls", "-Xlint"),
    javacOptions in Compile ++= Seq("-Xlint:unchecked", "-Xlint:deprecation"),
    libraryDependencies ++= Seq(
      "com.typesafe.akka" %% "akka-actor-typed" % akkaVersion,
      "com.typesafe.akka" %% "akka-persistence-typed" % akkaVersion,
      "ch.qos.logback" % "logback-classic" % logbackVersion,
      "com.typesafe.akka" %% "akka-actor-testkit-typed" % akkaVersion % "test",
      "com.typesafe.akka" %% "akka-persistence-testkit" % akkaVersion % "test",
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
