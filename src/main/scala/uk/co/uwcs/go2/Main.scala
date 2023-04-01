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
import uk.co.uwcs.go2.auth._
import org.pac4j.http4s.SecurityFilterMiddleware

object Main extends IOApp:
  // build the configs that we need
  val goConfig   = GoConfig.apply
  val authConfig = AuthConfigFactory().build(goConfig.rootUrl, goConfig.oidcSecret)

  // for comprehension to compose the resources that build our server
  val server =
    for {
      session <- Session
        .pooled[IO](
          goConfig.db.host,
          goConfig.db.port,
          goConfig.db.user,
          goConfig.db.database,
          password = Some(goConfig.db.password),
          max = 8
        )

      // compose our service from our home route and the redirect service routes
      // turn routes into an app using orNotFound, then wrap in logger middleware
      service = Logger.httpApp(true, true)(
        (LoginCallbackService(authConfig).routes
          <+> AdminService(session).routes
          <+> RedirectService(
            session
          ).routes).orNotFound
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
