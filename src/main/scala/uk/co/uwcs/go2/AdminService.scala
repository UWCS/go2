package uk.co.uwcs.go2

import cats.effect._
import cats.syntax.all._
import org.http4s._
import org.http4s.dsl.io._
import org.http4s.implicits._
import org.http4s.twirl._
import skunk._
import skunk.implicits._
import skunk.codec.all._
import org.http4s.headers.Location

case class Redirect(source: String, sink: String, usages: Int)

class AdminService(val session: Resource[IO, Session[IO]]):

  val routes = HttpRoutes
    .of[IO] {
      case GET -> Root => getAllRedirects().map(html.panel.apply).flatMap(Ok(_))
      // case req @ POST -> Root => req.as[Redirect].flatMap(addNewRedirect) >> Found(Location(uri"/"))
      case req @ POST -> Root =>
        req.decode { (f: UrlForm) =>
          val src = f.values("source").headOption
          val snk = f.values("sink").headOption
          (src, snk) match
            case (Some(src), Some(snk)) => addNewRedirect(src, snk) >> SeeOther(Location(uri"/"))
            case _                      => BadRequest(s"Invalid data: " + f.values.mkString("\n"))
        }
    }

  private def getAllRedirects() =
    val getAllQ: skunk.Query[Void, Redirect] =
      sql"SELECT source, sink, usages FROM redirect_redirect"
        .query(varchar(50) ~ varchar(1024) ~ int4)
        .gmap[Redirect]

    session.use(_.execute(getAllQ))

  private def addNewRedirect(source: String, sink: String): IO[Unit] =
    val addNewC: Command[String ~ String] =
      sql"INSERT INTO redirect_redirect (source, sink, usages, permanent) VALUES ($varchar, $varchar, 0, 'false')".command

    session.use(_.prepare(addNewC).use(_.execute(source, sink))).void
