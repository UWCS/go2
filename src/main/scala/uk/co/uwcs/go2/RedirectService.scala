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
    println(source)
    session
      .use { s =>                       // use the database session
        s.prepare(q)                    // prepare the statement
          .use(ps => ps.option(source)) // use statement to look up in db, ps.option returns either Some(row) or None
      }
      .flatMap(_.map(Uri.fromString) match
        case None              => NotFound()                        // 404 if not found
        case Some(Left(_))     => InternalServerError()             // 500 if found but couldn't parse uri (unlikely)
        case Some(Right(sink)) => TemporaryRedirect(Location(sink)) // 307 to redirect if found
      )

  // sink is varchar(1024)
  // source is varchar(50)
  private val q: skunk.Query[String, String] =
    sql"""SELECT sink FROM redirect_redirect WHERE source=$varchar""".query(varchar(1024))
