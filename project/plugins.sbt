addSbtPlugin("io.github.davidgregory084" % "sbt-tpolecat" % "0.1.14")
addSbtPlugin("io.spray"                  % "sbt-revolver" % "0.9.1")
addSbtPlugin("com.eed3si9n"              % "sbt-assembly" % "0.14.10")
addSbtPlugin("org.scalameta"             % "sbt-scalafmt" % "2.4.6")
addSbtPlugin("com.typesafe.sbt"          % "sbt-twirl"    % "1.5.1")

ThisBuild / libraryDependencySchemes ++= Seq(
  "org.scala-lang.modules" %% "scala-xml" % VersionScheme.Always
)
