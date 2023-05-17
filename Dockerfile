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
COPY ./go2/src ./src

# build our code
RUN rm ./target/release/deps/go2*
RUN cargo build --release

# new base, slimmer, no toolchains
FROM debian:bullseye-slim
COPY --from=builder /go2/target/release/go2 .

CMD [ "./go2" ]