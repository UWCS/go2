package uk.co.uwcs.go2

import cats.effect._
import cats.syntax.all._
import org.http4s._
import org.http4s.dsl.io._
import org.http4s.implicits._
import org.http4s.headers.Location
import skunk._
import skunk.implicits._
import skunk.codec.all._

// the redirect service takes a session resource
// cats-effect Resource monad - https://typelevel.org/cats-effect/docs/std/resource
class RedirectService(val session: Resource[IO, Session[IO]]):
  val routes = HttpRoutes
    .of[IO] { case GET -> Root / source => TemporaryRedirect(Location(getSink(source))) }

  def getSink(source: String): Uri = ???

  val q: skunk.Query[String, String] =
    sql"""SELECT sink FROM redirect_redirect WHERE source= $varchar""".query(varchar)
