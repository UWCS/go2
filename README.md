# go2

`go2` is a simpler URL shortener and redirection service for creating short links. It's a successor to [UWCS-go](https://github.com/UWCS/uwcs-go), designed to be more streamlined.

## Overview

`go2` is written in Scala 3.

- [`http4s`](https://http4s.org/) provides a http server and tools for building a web API
- [`cats-effect`](https://typelevel.org/cats-effect/) provides an purely functional, asynchronous runtime
- [`skunk`](https://tpolecat.github.io/skunk/) provides a purely functional, asynchronous interface with PostgreSQL

## Development

sbt is used as the build tool. Use `sbt run` to build and run the server.

`go2` will attempt to read from the environment variables listed in `Main.scala` to establish a database connection. Be sure to configure your development environment with those variables before running.

## Docker

Configure the environment variables in `docker-compose-yml`, and run `docker compose up` to start the server. Docker will expose the service on port `8125` by default. Make sure that the network is properly configured such that the container may access the database.
