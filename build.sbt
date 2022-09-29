val scala3Version = "3.2.0"

//val catsVersion = "2.7.0"
//val http4sVersion = "1.0.0-M33"
val scalatestVersion = "3.2.11"
//val userVersion = "0.1.1"

ThisBuild / organization := "com.nigeleke"
ThisBuild / scalaVersion := scala3Version
ThisBuild / version := "0.1.0-SNAPSHOT"

Compile / compile / wartremoverErrors ++= Warts.allBut(Wart.Equals, Wart.SizeIs)
Compile / doc / scalacOptions ++= Seq("-groups")

lazy val root = project
  .in(file("."))
  .settings(
    name := "cribbage"
  )
  .aggregate(core)

lazy val core = project
  .in(file("core"))
  .settings(
    name := "cribbage-core",
    libraryDependencies ++= Seq(
      "org.scalactic" %% "scalactic" % scalatestVersion,
      "org.scalatest" %% "scalatest" % scalatestVersion % "test"
    )
  )

//lazy val api = project
//  .in(file("api"))
//  .settings(
//    name := "cribbage-api",
//    libraryDependencies ++= Seq(
//      "com.nigeleke" %% "user" % userVersion,
//      "org.http4s" %% "http4s-dsl" % http4sVersion,
//      "org.scalactic" %% "scalactic" % scalatestVersion,
//      "org.scalatest" %% "scalatest" % scalatestVersion % "test"
//    )
//  )
//  .dependsOn(core, shared)
//
//lazy val shared = project
//  .in(file("shared"))
//  .settings(
//    name := "cribbage-shared",
//    libraryDependencies ++= Seq(
//      "org.scalactic" %% "scalactic" % scalatestVersion,
//      "org.scalatest" %% "scalatest" % scalatestVersion % "test"
//    )
//  )
