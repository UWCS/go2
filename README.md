# go2

`go2` is a simpler URL shortener and redirection service for creating short links. It's a successor to [uwcs-go](https://github.com/UWCS/uwcs-go), designed to be more streamlined.

## Overview

`go2` is written in Rust.

- [`axum`](https://github.com/tokio-rs/axum) is used as an http framework, build on top of [tokio](https://github.com/tokio-rs/tokio) and [hyper](https://github.com/hyperium/hyper)
- [`sqlx`](https://github.com) is used for database queries

## Development

`cargo run` to build and start the server

`go2` will attempt to read from the environment variables listed in `.env.example` to establish a database connection, both at compile time ([for sqlx](https://github.com/launchbadge/sqlx#compile-time-verification)) and when running. Be sure to configure your development environment with those variables before running, which can be done by coping `.env.example` to `.env`.

## Docker

Configure the environment variables in `docker-compose.yml`, and run `docker compose up` to start the server. Docker will expose the service on port `8125` by default. Make sure that the network is properly configured such that the container may access the database.
