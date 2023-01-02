val scala3Version = "3.2.2-RC2"

organizationName := "Nigel Eke"
organization     := "nigeleke"

val bsd3License = Some(HeaderLicense.BSD3Clause("2022", "Nigel Eke"))

val scalatestVersion = "3.2.14"

lazy val root = project
  .in(file("."))
  .disablePlugins(HeaderPlugin)
  .settings(
    name    := "cribbage",
    version := "0.1.0-SNAPSHOT"
  )
  .aggregate(core)

lazy val core = project
  .in(file("core"))
  .settings(
    name           := "cribbage-core",
    scalaVersion   := scala3Version,
    headerLicense  := bsd3License,
    libraryDependencies ++= Seq(
      "org.scalactic" %% "scalactic" % scalatestVersion,
      "org.scalatest" %% "scalatest" % scalatestVersion % "test"
    ),
    publish / skip := true
  )
