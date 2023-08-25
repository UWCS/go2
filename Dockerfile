FROM rust:1-alpine as builder

# create empty projects
RUN cargo new --bin go2
WORKDIR /go2
# copy package manifest in 
COPY Cargo.toml Cargo.lock /go2/


RUN apk add --no-cache musl-dev
# build to cache deps
RUN cargo build --release
RUN rm src/*.rs

# install tailwind cli
# from https://github.com/Angatar/tailwindcss/blob/main/Dockerfile
RUN apk add --no-cache wget jq
ARG TARGETPLATFORM
RUN wget -O tailwindcss https://github.com/tailwindlabs/tailwindcss/releases/download/$(wget -q -O - https://api.github.com/repos/tailwindlabs/tailwindcss/releases/latest | jq -r ".tag_name")/tailwindcss-$(echo $TARGETPLATFORM|sed 's/linux\//linux-/'|sed 's/arm\/v[-,7,6]/armv7/'|sed 's/amd64/x64/') \
    && chmod u+x tailwindcss

# copy source code (and sqlx stuff) (and templates) (and static (css is build by cargo)) in
COPY ./src ./src
COPY ./.sqlx ./.sqlx
COPY ./migrations ./migrations
COPY ./templates ./templates
COPY ./static ./static
COPY tailwind.config.js tailwind.config.js
COPY build.rs build.rs

# build our code
RUN rm ./target/release/deps/go2*
RUN cargo build --release

# new base, slimmer, no toolchains
FROM scratch
COPY --from=builder /go2/target/release/go2 .
# static folder is built into binary

CMD [ "./go2" ]