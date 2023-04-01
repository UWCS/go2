package uk.co.uwcs.go2.auth

import cats.effect.{ IO, Sync }
import org.pac4j.core.client.Clients
import org.pac4j.core.config.{ Config, ConfigFactory }
import org.pac4j.oidc.client.KeycloakOidcClient
import org.pac4j.oidc.config.KeycloakOidcConfiguration
import org.pac4j.http4s.Http4sCacheSessionStore
import org.pac4j.http4s.DefaultHttpActionAdapter

class AuthConfigFactory extends ConfigFactory {
  override def build(parameters: AnyRef*): Config = {
    // cursed but java interop moment
    val rootUrl    = parameters(0).asInstanceOf[String]
    val oidcSecret = parameters(1).asInstanceOf[String]
    val clients =
      new Clients(rootUrl, keycloakClient(oidcSecret)) // this won't work on a dev machine

    val config = new Config(clients)
    config.setHttpActionAdapter(new DefaultHttpActionAdapter[IO]) // <-- Render a nicer page
    config.setSessionStore(new Http4sCacheSessionStore[IO])
    config
  }

  def keycloakClient(oidcSecret: String) = {
    val config = new KeycloakOidcConfiguration
    config.setBaseUri("https://auth.uwcs.co.uk")
    config.setRealm("uwcs")
    val client = new KeycloakOidcClient(config)
    client
  }
}
