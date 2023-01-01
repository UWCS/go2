val Http4sVersion     = "0.23.16"
val LogbackVersion    = "1.2.10"
val SkunkVersion      = "0.3.2"
val CatsEffectVersion = "3.4.3"

lazy val root = (project in file("."))
  .settings(
    organization               := "uk.co.uwcs",
    name                       := "go2",
    version                    := "0.1.0",
    scalaVersion               := "3.2.1",
    assembly / assemblyJarName := "go2.jar",
    libraryDependencies ++= Seq(
      "org.http4s"    %% "http4s-ember-server" % Http4sVersion,
      "org.http4s"    %% "http4s-dsl"          % Http4sVersion,
      "ch.qos.logback" % "logback-classic"     % LogbackVersion,
      "org.tpolecat"  %% "skunk-core"          % SkunkVersion,
      "org.typelevel" %% "cats-effect"         % CatsEffectVersion
    )
  )
