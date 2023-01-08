package uk.co.uwcs.go2

import cats.effect._
import cats.effect.std.Env
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
import natchez.Trace.Implicits.noop

object Main extends IOApp:
  // for comprehension to compose the resources that build our server
  val server =
    for {
      env <- Resource.eval(Env[IO].entries.map(_.toMap))

      // will throw exceptions on startup if keys missing
      // slightly messy error handling in this functional context but ehhhhh
      session <-
        try
          Session.pooled[IO](
            host = env.get("POSTGRES_HOST").get,
            port = env.get("POSTGRES_PORT").get.toInt,
            user = env.get("POSTGRES_USER").get,
            database = env.get("POSTGRES_DB").get,
            password = Some(env.get("POSTGRES_PASSWORD").get),
            max = 8
          )
        catch
          case e: java.util.NoSuchElementException =>
            throw java.util.NoSuchElementException("Environment variables for Postgres connection not defined")

      // compose our service from our home route and the redirect service routes
      // turn routes into an app using orNotFound, then wrap in logger middleware
      service = Logger.httpApp(true, true)(
        (AdminService(session).routes <+> RedirectService(session).routes).orNotFound
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
