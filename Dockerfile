FROM ekidd/rust-musl-builder:stable as builder

RUN USER=root cargo new --bin nun-db-prometheus-exporter
WORKDIR ./nun-db-prometheus-exporter

COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs

ADD . ./

RUN rm ./target/x86_64-unknown-linux-musl/release/deps/nun*
RUN cargo build --release

FROM alpine:3.12.4
COPY --from=builder /home/rust/src/nun-db/target/x86_64-unknown-linux-musl/release/prometheus-exporter /usr/bin/prometheus-exporter

ENTRYPOINT  [ "prometheus-exporter" ]
