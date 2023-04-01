package uk.co.uwcs.go2.auth

import cats.effect._
import cats.syntax.all._
import org.http4s._
import org.http4s.dsl.io._
import org.http4s.implicits._
import org.pac4j.http4s.{ CallbackService, Http4sWebContext }
import org.pac4j.core.config.Config

class LoginCallbackService(authConfig: Config):
  val callbackService = new CallbackService(authConfig, Http4sWebContext.ioInstance)

  val routes = HttpRoutes.of[IO] {
    case req @ GET -> Root / "callback" =>
      callbackService.callback(req)
    case req @ POST -> Root / "callback" =>
      callbackService.callback(req)
  }
