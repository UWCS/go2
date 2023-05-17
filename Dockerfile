FROM rust:1 as builder

# create empty projects
RUN USER=root cargo new --bin go2
WORKDIR /go2

# copy package manifest in 
COPY Cargo.toml Cargo.lock /go2

# build to cache deps
RUN cargo build --release
RUN rm src/*.rs

# copy source code in
COPY ./dcspkg-server/src ./src

# build our code
RUN rm ./target/release/deps/dcspkg*
RUN cargo build --release

# new base, slimmer, no toolchains
FROM debian:bullseye-slim
COPY --from=builder /dcspkg-server/target/release/dcspkg_server .

CMD [ "./dcspkg_server" ]