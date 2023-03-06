FROM rust:slim as rust-builder
WORKDIR /home/rust/src
RUN apt update && apt install -y libssl-dev pkg-config
# temporarily using default web server
RUN cargo install basic-http-server
COPY src/ src/
COPY Cargo.* .
RUN cargo build --release


FROM node:alpine as node-builder
WORKDIR /home/node/src
COPY index.html . 


FROM debian:stable-slim
# meilisearch rust sdk only compatible with v0.27.0
RUN apt update && apt install -y curl
RUN curl -LsSf 'https://github.com/meilisearch/meilisearch/releases/download/v0.27.0/meilisearch.deb' > meilisearch.deb
RUN apt install -y ./meilisearch.deb
# install nginx
RUN apt install -y nginx gettext-base
COPY nginx.conf nginx.conf.template
# copy executables from rust-builder
COPY --from=rust-builder /home/rust/src/target/release/sound-quiz .
COPY --from=rust-builder /usr/local/cargo/bin/basic-http-server ./webserver
COPY --from=node-builder /home/node/src/index.html .
COPY startup.sh .

ENV RUST_LOG=info
ENV MEILI_HTTP_ADDR=0.0.0.0:7700

ENV PORT=${PORT:-80}

ENTRYPOINT ["/bin/sh", "startup.sh"]