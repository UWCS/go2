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
// this is all done in the IO monad
class RedirectService(val session: Resource[IO, Session[IO]]):
  val routes = HttpRoutes
    .of[IO] { case GET -> Root / source => redirect(source) }

  // implement our redirect
  // use the session, prepare the statement, and use it to look up the source
  private def redirect(source: String): IO[Response[IO]] =
    session
      .use { s => // use the database session
        s.prepare(sinkQ).use(_.option(source)).map(_.map(Uri.fromString)).flatMap {
          case None          => NotFound()            // 404 if not found
          case Some(Left(_)) => InternalServerError() // 500 if found but couldn't parse uri (unlikely)
          case Some(Right(sink)) =>
            s.prepare(countC).use(_.execute(source))     // bump count
              >> s.prepare(dateC).use(_.execute(source)) // bump last used date
              >> MovedPermanently(Location(sink))        // 301 to redirect if found
        }
      }

  // sink is varchar(1024)
  // source is varchar(50)
  private val sinkQ: skunk.Query[String, String] =
    sql"SELECT sink FROM redirects WHERE source=$varchar".query(varchar(1024))

  private val countC: Command[String] =
    sql"UPDATE redirects SET usages = usages + 1 WHERE source=$varchar".command

  private val dateC: Command[String] =
    sql"UPDATE redirects SET last_used=now()::timestamptz WHERE source=$varchar".command
