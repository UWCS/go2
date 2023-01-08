addSbtPlugin("io.github.davidgregory084" % "sbt-tpolecat" % "0.1.14")
addSbtPlugin("io.spray"                  % "sbt-revolver" % "0.9.1")
addSbtPlugin("com.eed3si9n"              % "sbt-assembly" % "0.14.10")
addSbtPlugin("org.scalameta"             % "sbt-scalafmt" % "2.4.6")
addSbtPlugin("com.typesafe.play"         % "sbt-twirl"    % "1.6.0-RC1")
addSbtPlugin("nl.gn0s1s"                 % "sbt-dotenv"   % "3.0.0")

ThisBuild / libraryDependencySchemes ++= Seq(
  "org.scala-lang.modules" %% "scala-xml" % VersionScheme.Always
)
