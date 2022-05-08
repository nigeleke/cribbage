val scala3Version = "3.1.2"

val catsVersion = "2.7.0"
val scalatestVersion = "3.2.11"

lazy val root = project
  .in(file("."))
  .settings(name := "cribbage", version := "0.1.0-SNAPSHOT")
  .aggregate(core)

lazy val core = project
  .in(file("core"))
  .settings(
    name := "cribbage-core",
    scalaVersion := scala3Version,
    scalafixDependencies in ThisBuild += "org.scalalint" %% "rules" % "0.1.4",
    libraryDependencies ++= Seq(
      "org.typelevel" %% "cats-core" % catsVersion,
      "org.scalactic" %% "scalactic" % scalatestVersion,
      "org.scalatest" %% "scalatest" % scalatestVersion % "test"
    )
  )
