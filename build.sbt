val Http4sVersion     = "0.23.18"
val LogbackVersion    = "1.2.10"
val SkunkVersion      = "0.3.2"
val CatsEffectVersion = "3.4.3"

lazy val root = (project in file("."))
  .enablePlugins(SbtTwirl)
  .settings(
    organization               := "uk.co.uwcs",
    name                       := "go2",
    version                    := "0.1.0",
    scalaVersion               := "3.2.2",
    assembly / assemblyJarName := "go2.jar",
    libraryDependencies ++= Seq(
      "org.http4s"    %% "http4s-ember-server" % Http4sVersion,
      "org.http4s"    %% "http4s-dsl"          % Http4sVersion,
      "org.http4s"    %% "http4s-twirl"        % "0.24.0-M1",
      "ch.qos.logback" % "logback-classic"     % LogbackVersion,
      "org.tpolecat"  %% "skunk-core"          % "0.5.1",
      "org.typelevel" %% "cats-effect"         % "3.4.3",
      "org.pac4j"     %% "http4s-pac4j"        % "4.1.0",
      "org.pac4j"      % "pac4j-oidc"          % "5.7.0"
    )
  )
