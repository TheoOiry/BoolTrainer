FROM rust as builder

RUN rustup default nightly

WORKDIR /app

COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src
COPY ./migrations ./migrations
COPY ./diesel.toml ./diesel.toml

RUN cargo build

CMD ["cargo", "run"]