FROM ekidd/rust-musl-builder:stable as builder

RUN USER=root cargo new --bin nun-db-prometheus-exporter
WORKDIR ./nun-db-prometheus-exporter

COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release --jobs 4
RUN rm src/*.rs

ADD . ./

RUN rm ./target/x86_64-unknown-linux-musl/release/deps/nun*
RUN cargo build --release

FROM alpine:3.15

RUN apk add libressl-dev
COPY --from=builder /home/rust/src/nun-db/target/x86_64-unknown-linux-musl/release/nun-db-prometheus-exporter /usr/bin/nun-db-prometheus-exporter

ENTRYPOINT  [ "nun-db-prometheus-exporter" ]
