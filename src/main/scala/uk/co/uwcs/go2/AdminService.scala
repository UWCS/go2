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
import java.time.OffsetDateTime

case class Redirect(
    source: String,
    sink: String,
    usages: Int,
    used: Option[OffsetDateTime],
    created: Option[OffsetDateTime]
)

class AdminService(val session: Resource[IO, Session[IO]]):

  val routes = HttpRoutes
    .of[IO] {
      case GET -> Root => getAllRedirects().map(html.panel.apply).flatMap(Ok(_)) // the homepage

      case request @ GET -> Root / "static" / file =>
        StaticFile.fromResource(request.pathInfo.toString, Some(request)).getOrElseF(NotFound()) // static files from classpath

      case req @ POST -> Root =>
        req.decode { (f: UrlForm) =>
          val src = f.values("source").headOption
          val snk = f.values("sink").headOption
          (src, snk) match
            case (Some(src), Some(snk)) => addNewRedirect(src, snk) >> SeeOther(Location(uri"/"))
            case _                      => BadRequest(s"Invalid data: " + f.values.mkString("\n"))
        } // POST to root url adds a new db row
    }

  private def getAllRedirects() =
    val getAllQ: skunk.Query[Void, Redirect] =
      sql"SELECT source, sink, usages, last_used, created FROM redirects ORDER BY last_used desc NULLS LAST"
        .query(varchar(50) ~ varchar(1024) ~ int4 ~ timestamptz.opt ~ timestamptz.opt)
        .gmap[Redirect]

    session.use(_.execute(getAllQ))

  private def addNewRedirect(source: String, sink: String): IO[Unit] =
    val addNewC: Command[String ~ String] =
      sql"INSERT INTO redirects (source, sink) VALUES ($varchar, $varchar)".command

    session.use(_.prepare(addNewC).flatMap(_.execute(source, sink))).void
