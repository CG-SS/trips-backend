FROM alpine:latest

ARG DATABASE_URL

ENV DATABASE_URL=$DATABASE_URL

RUN apk add cargo sqlite sqlite-dev

RUN mkdir /build
WORKDIR /build

COPY migrations ./migrations
COPY src ./src
COPY Cargo.lock .
COPY Cargo.toml .

RUN cargo install diesel_cli --no-default-features --features sqlite
RUN $HOME/.cargo/bin/diesel setup && $HOME/.cargo/bin/diesel migration run

RUN cargo build --release

EXPOSE 8080

ENTRYPOINT ["/build/target/release/trips-backend"]