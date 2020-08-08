ThisBuild / organization := "com.nigeleke"
ThisBuild / scalaVersion := "2.13.2"
ThisBuild / version      := "0.1-SNAPSHOT"

// License
ThisBuild / organizationName := "Nigel Eke"
ThisBuild / startYear := Some(2020)
ThisBuild / licenses += ("AGPL-3.0-or-later", new URL("https://www.gnu.org/licenses/agpl-3.0.txt"))

val akkaVersion = "2.6.8"
val guiceVersion = "4.2.4-20200419-NEWAOP-BETA"
val logbackVersion = "1.2.3"
val scalaTestVersion = "3.1.2"
val scalaTestPlusPlayVersion = "5.1.0"

lazy val root = (project in file("."))
  .settings(
    name := "cribbage"
  )
  .aggregate(backend.jvm, model.js, model.jvm, ui.js)

lazy val backend = crossProject(JSPlatform, JVMPlatform)
  .withoutSuffixFor(JSPlatform)
  .crossType((CrossType.Pure))
  .settings(
    libraryDependencies ++= Seq(
      "com.typesafe.akka" %% "akka-actor-typed" % akkaVersion,
      "com.typesafe.akka" %% "akka-persistence-typed" % akkaVersion,
      "ch.qos.logback" % "logback-classic" % logbackVersion,
      "com.typesafe.akka" %% "akka-actor-testkit-typed" % akkaVersion % "test",
      "com.typesafe.akka" %% "akka-persistence-testkit" % akkaVersion % "test",
      "org.scalatest" %% "scalatest" % scalaTestVersion % "test"
    )
  )
  .jvmSettings(
    scalacOptions in Compile ++= Seq("-deprecation", "-feature", "-unchecked", "-Xlog-reflective-calls", "-Xlint"),
    scalacOptions in Test ++= Seq("-deprecation", "-feature", "-unchecked", "-Xlog-reflective-calls", "-Xlint"),
    javacOptions in Compile ++= Seq("-Xlint:unchecked", "-Xlint:deprecation"),
  )
  .dependsOn(model)

lazy val model = crossProject(JSPlatform, JVMPlatform)
  .crossType(CrossType.Pure)
  .settings(
    libraryDependencies ++= Seq(
      "org.scalatest" %% "scalatest" % scalaTestVersion % "test"
    )
  )

lazy val ui = crossProject(JSPlatform, JVMPlatform)
  .withoutSuffixFor(JVMPlatform)
  .crossType(CrossType.Pure)
  .settings(
    libraryDependencies ++= Seq(
      "org.scala-js" %%% "scalajs-dom" % "1.0.0",
      "com.lihaoyi" %%% "scalatags" % "0.9.1",
      "org.scalatest" %% "scalatest" % scalaTestVersion % "test"
    )
  )
  .jsSettings(
    scalaJSUseMainModuleInitializer := true
  )
  .dependsOn(model)
  .aggregate(model)

