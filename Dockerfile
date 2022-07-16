FROM rust:slim as builder
WORKDIR /home/rust/src
RUN apt update && apt install -y libssl-dev pkg-config
# temporarily using default web server
RUN cargo install basic-http-server
COPY src/ src/
COPY Cargo.* .
RUN cargo build --release

FROM debian:stable-slim
# meilisearch rust sdk only compatible with v0.27.0
RUN apt update && apt install -y curl
RUN curl -LsSf 'https://github.com/meilisearch/meilisearch/releases/download/v0.27.0/meilisearch.deb' > meilisearch.deb
RUN apt install -y ./meilisearch.deb
# install nginx
RUN apt install -y nginx gettext-base
# copy executables from builder
COPY --from=builder /home/rust/src/target/release/bootstrap .
COPY --from=builder /usr/local/cargo/bin/basic-http-server ./webserver
COPY startup.sh .
COPY index.html .
COPY nginx.conf nginx.conf.template

ENV RUST_LOG=info
ENV MEILI_HTTP_ADDR=0.0.0.0:7700
ENV PORT=${PORT:-80}

ENTRYPOINT ["/bin/sh", "startup.sh"]