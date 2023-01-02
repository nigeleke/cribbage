val scala3Version = "3.2.2-RC2"

val catsVersion          = "2.9.0"
val catsScalatestVersion = "2.1.5"
val scalatestVersion     = "3.2.14"

lazy val root = project
  .in(file("."))
  .settings(
    name    := "cribbage",
    version := "0.1.0-SNAPSHOT"
  )
  .aggregate(core)

lazy val core = project
  .in(file("core"))
  .settings(
    name         := "cribbage-core",
    scalaVersion := scala3Version,
    libraryDependencies ++= Seq(
      "org.scalactic" %% "scalactic" % scalatestVersion,
      "org.scalatest" %% "scalatest" % scalatestVersion % "test"
    )
  )
