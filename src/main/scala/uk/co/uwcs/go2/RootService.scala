package uk.co.uwcs.go2

import cats.effect._
import cats.syntax.all._
import org.http4s._
import org.http4s.dsl.io._
import org.http4s.implicits._

object RootService:
  val routes = HttpRoutes
    .of[IO] { case GET -> Root => Ok("Welcome to go2!") }
