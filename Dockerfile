FROM rustlang/rust:nightly AS builder

RUN USER=root cargo new --bin united_traders
WORKDIR /united_traders

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN rm ./target/release/deps/united_traders*
RUN cargo build --release

FROM debian:buster-slim

RUN apt-get update && \
    apt-get --assume-yes install \
        make \
        libpq5 \
        libpq-dev \
        -qqy \
        --no-install-recommends
RUN apt-get update && apt-get -y install ca-certificates libssl-dev && rm -rf /var/lib/apt/lists/*

COPY --from=builder /united_traders/target/release/united_traders /united_traders/united_traders
WORKDIR /united_traders/
EXPOSE 8088


CMD ["/united_traders/united_traders"]