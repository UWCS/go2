val Http4sVersion  = "0.23.16"
val LogbackVersion = "1.2.10"
val SkunkVersion   = "0.3.2"

lazy val root = (project in file("."))
  .settings(
    organization := "uk.co.uwcs",
    name         := "go2",
    version      := "0.0.1-SNAPSHOT",
    scalaVersion := "3.2.1",
    libraryDependencies ++= Seq(
      "org.http4s"    %% "http4s-ember-server" % Http4sVersion,
      "org.http4s"    %% "http4s-dsl"          % Http4sVersion,
      "ch.qos.logback" % "logback-classic"     % LogbackVersion,
      "org.tpolecat"  %% "skunk-core"          % SkunkVersion
    ),
    testFrameworks += new TestFramework("munit.Framework")
  )
