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
        for {     // for comprehension to compose the effects in the IO monad
          lookup <- s
            .prepare(sinkQ)        // prepare the statement
            .use(_.option(source)) // use statement to look up in db, ps.option returns either Some(row) or None

          response <- lookup.map(Uri.fromString) match
            case None          => NotFound()            // 404 if not found
            case Some(Left(_)) => InternalServerError() // 500 if found but couldn't parse uri (unlikely)
            case Some(Right(sink)) => TemporaryRedirect(Location(sink)) // 307 to redirect if found

            // only increase count if we actually do the redirect
            // could probably be nicer composed through some monad than using a condition
          _ <- if response.status == TemporaryRedirect then s.prepare(countC).use(_.execute(source)) else IO.pure(())
        } yield response
      }

  // sink is varchar(1024)
  // source is varchar(50)
  private val sinkQ: skunk.Query[String, String] =
    sql"""SELECT sink FROM redirect_redirect WHERE source=$varchar""".query(varchar(1024))

  private val countC: Command[String] =
    sql"""UPDATE redirect_redirect SET usages = usages + 1 WHERE source=$varchar""".command
