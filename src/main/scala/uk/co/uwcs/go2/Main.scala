package uk.co.uwcs.go2

import cats.effect._
import com.comcast.ip4s._
import org.http4s.implicits._
import org.http4s.ember.server._
import cats.implicits._

object Main extends IOApp:
  val services = (RootService.routes <+> RedirectService.routes).orNotFound
  def run(args: List[String]): IO[ExitCode] =
    EmberServerBuilder
      .default[IO]
      .withHost(ipv4"0.0.0.0")
      .withPort(port"8080")
      .withHttpApp(services)
      .build
      .use(_ => IO.never)
      .as(ExitCode.Success)
