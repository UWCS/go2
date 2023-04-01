package uk.co.uwcs.go2

import cats.effect._
import cats.effect.std.Env
import cats.implicits._
import sys.env

case class PostgresConfig(
    host: String,
    port: Int,
    user: String,
    database: String,
    password: String
)

case class GoConfig(
    db: PostgresConfig,
    rootUrl: String,
    oidcSecret: String
)

object GoConfig:
  // builds the config object
  // throws (bad) but need to stop if cant get config
  // so EitherT or whatever would just be theatre
  def apply: GoConfig =
    val host     = env("POSTGRES_HOST")
    val port     = env("POSTGRES_PORT").toInt
    val user     = env("POSTGRES_USER")
    val database = env("POSTGRES_DB")
    val password = env("POSTGRES_PASSWORD")
    val pgconf   = PostgresConfig(host, port, user, database, password)

    val rootUrl    = env("ROOT_URL")
    val oidcSecret = env("OIDC_SECRET")
    GoConfig(pgconf, rootUrl, oidcSecret)
