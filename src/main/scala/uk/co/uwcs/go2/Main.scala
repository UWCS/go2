package uk.co.uwcs.go2

import cats.effect._
import com.comcast.ip4s._
import org.http4s.implicits._
import org.http4s.ember.server._
import org.http4s.server.Router
import org.http4s.dsl.io._
import cats.implicits._
import org.http4s.server.middleware._
import cats.syntax.all._
import org.http4s._
import skunk._
import skunk.implicits._
import skunk.codec.all._
import natchez.Trace.Implicits.noop

object Main extends IOApp:
  val home = HttpRoutes.of[IO] { case GET -> Root => Ok("Welcome to go2!") }

  // for comprehension to compose the effects that build our server
  val server = for {
    session <- Session.pooled[IO](
      host = "localhost",
      port = 5432,
      user = "uwcs_go",
      database = "uwcs_go",
      password = Some("banana"),
      max = 6
    )

    // compose our service from our home route and the redirect service routes
    // turn routes into an app using orNotFound, then wrap in logger middleware
    service = Logger.httpApp(true, true)(
      (home <+> RedirectService(session).routes).orNotFound
    )

    // build the server
    server <- EmberServerBuilder
      .default[IO]
      .withHost(ipv4"0.0.0.0")
      .withPort(port"8080")
      .withHttpApp(service)
      .build

  } yield server

  override def run(args: List[String]): IO[ExitCode] =
    server
      .use(_ => IO.never)
      .as(ExitCode.Success)
