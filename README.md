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

## API Auth

API access is controlled by a token which must be provided in an authorisation header as a bearer token. The token is read from an environment variable when starting the server, and **this must be set and kept secret in production**. A random API token is generated if one is not set, the value of which is logged. This **should only be used for development purposes**

## `sqlx` & working with the database

sqlx is far too clever, and it's query macros connect to the database **at compile time** to verify that your SQL is correct. It reads the `DATABASE_URL` from `.env` to do this, so make sure this is set or you will get compile errors. The easiest way to do this is to develop using docker, as the compose stack will spin up a database for you. 

Alternatively, the `sqlx-data.json` file contains info generated from the database against which queries can be checked. If updating the database schema or queries, ths must be regenerated. Install the sqlx cli and run `cargo sqlx prepare` to do this. 

Migrations in `/migrations` are embedded in the binary and run on startup. New migrations will need to be added if you make any changes that affect the database schema. Make sure to regeneate the `sqlx-data.json` too.