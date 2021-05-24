FROM rust as builder

RUN rustup default nightly

RUN USER=root cargo new --bin bt_api
WORKDIR /bt_api
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build

RUN rm src/*.rs

COPY ./src ./src
COPY ./migrations ./migrations
COPY ./diesel.toml ./diesel.toml

RUN rm ./target/debug/deps/bt_api*
RUN cargo build

FROM buildpack-deps:stretch

COPY --from=builder /bt_api/target/debug/bt_api /app/

ENTRYPOINT [ "/app/bt_api" ]
