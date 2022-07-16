FROM rust:alpine as builder
WORKDIR /home/rust/src
RUN apk --no-cache add musl-dev openssl-dev
COPY . .
RUN cargo build --release
RUN cargo install basic-http-server

FROM getmeili/meilisearch:latest
COPY --from=builder /home/rust/src/target/release/bootstrap .
COPY --from=builder /usr/local/cargo/bin/basic-http-server .
COPY startup.sh .
COPY index.html .

ENV RUST_LOG=info

ENTRYPOINT ["/bin/sh", "startup.sh"]