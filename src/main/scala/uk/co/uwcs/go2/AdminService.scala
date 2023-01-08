package uk.co.uwcs.go2

import cats.effect._
import cats.syntax.all._
import org.http4s._
import org.http4s.dsl.io._
import org.http4s.implicits._
import org.http4s.twirl._
import play.twirl.api.Html
import skunk._
import skunk.implicits._
import skunk.codec.all._

case class Redirect(source: String, sink: String, usages: Int)

class AdminService(val session: Resource[IO, Session[IO]]):

  val routes = HttpRoutes
    .of[IO] {
      case GET -> Root  => getAllRedirects().map(html.panel.apply).flatMap(Ok(_))
      case POST -> Root => Ok()
    }

  private def getAllRedirects() =
    val getAllQ: skunk.Query[Void, Redirect] =
      sql"SELECT source, sink, usages FROM redirect_redirect"
        .query(varchar(50) ~ varchar(1024) ~ int4)
        .gmap[Redirect]

    session.use(_.execute(getAllQ))

  private def addNewRedirect(r: Redirect): IO[Unit] =
    val addNewC: Command[Redirect] =
      sql"INSERT INTO redirect_redirect (source, sink, usages) VALUES ($varchar, $varchar, $int4)".command
        .gcontramap[Redirect]

    session.use(_.prepare(addNewC).use(_.execute(r))).void
