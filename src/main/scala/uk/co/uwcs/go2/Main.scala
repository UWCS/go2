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
import natchez.Trace.Implicits.noop

object Main extends IOApp:
  val home = HttpRoutes.of[IO] { case GET -> Root => Ok("Welcome to go2!") }

  // for comprehension to compose the effects that build our server
  def createServer(env: Map[String, String]) = for {

    session <- Session.pooled[IO](
      host = env.get("POSTGRES_HOST").get,
      port = env.get("POSTGRES_PORT").get.toInt,
      user = env.get("POSTGRES_USER").get,
      database = env.get("POSTGRES_DB").get,
      password = Some(env.get("POSTGRES_PASSWORD").get),
      max = 8
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
    createServer(sys.env)
      .use(_ => IO.never)
      .as(ExitCode.Success)
