val scala3Version = "3.3.0-RC5"

organizationName := "Nigel Eke"
organization     := "nigeleke"

val bsd3License = Some(HeaderLicense.BSD3Clause("2022", "Nigel Eke"))

val catsEffectTestingVersion = "1.5.0"
val http4sVersion            = "0.23.19" // Must match tapir dependency...
val scalatestVersion         = "3.2.15"
val tapirVersion             = "1.2.13"

lazy val root = project
  .in(file("."))
  .disablePlugins(HeaderPlugin)
  .settings(
    name           := "cribbage",
    publish / skip := true
  )
  .aggregate(core, api)

lazy val core = project
  .in(file("core"))
  .settings(
    name           := "cribbage-core",
    scalaVersion   := scala3Version,
    headerLicense  := bsd3License,
    publish / skip := true,
    libraryDependencies ++= Seq(
      "org.scalactic" %% "scalactic" % scalatestVersion,
      "org.scalatest" %% "scalatest" % scalatestVersion % "test"
    )
  )

lazy val api = project
  .in(file("api"))
  .settings(
    name           := "cribbage-api",
    scalaVersion   := scala3Version,
    headerLicense  := bsd3License,
    publish / skip := true,
    libraryDependencies ++= Seq(
      "com.softwaremill.sttp.tapir" %% "tapir-core"                    % tapirVersion,
      "com.softwaremill.sttp.tapir" %% "tapir-http4s-server"           % tapirVersion,
      "com.softwaremill.sttp.tapir" %% "tapir-json-circe"              % tapirVersion,
      "org.http4s"                  %% "http4s-circe"                  % http4sVersion,
      "org.scalactic"               %% "scalactic"                     % scalatestVersion,
      "com.softwaremill.sttp.tapir" %% "tapir-http4s-client"           % tapirVersion             % "test",
      "com.softwaremill.sttp.tapir" %% "tapir-testing"                 % tapirVersion             % "test",
      "org.scalatest"               %% "scalatest"                     % scalatestVersion         % "test",
      "org.typelevel"               %% "cats-effect-testing-scalatest" % catsEffectTestingVersion % "test"
    )
  )
  .dependsOn(core)
