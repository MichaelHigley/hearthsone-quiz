FROM rust:slim-bullseye as rust-builder
WORKDIR /home/rust/src
RUN apt update
RUN apt install -y pkg-config libssl-dev build-essential
COPY src/ src/
COPY Cargo.* .
RUN cargo build --release --locked 


FROM scratch as site-builder
WORKDIR /home/node/src
COPY index.html . 


FROM debian:bullseye-slim
# install nginx
RUN apt update
RUN apt install -y nginx openssl curl libssl-dev
COPY nginx.conf nginx.conf.template
# simple webserver
RUN apt install -y python3
# Install Meilisearch latest version from the script
RUN curl -L https://install.meilisearch.com | sh
# copy bootstrap executable from rust-builder
COPY --from=rust-builder /home/rust/src/target/release/sound-quiz ./bootstrap
COPY --from=site-builder /home/node/src/index.html .
COPY startup.sh .

ENV RUST_BACKTRACE=full
ENV RUST_LOG=info
ENV MEILI_HTTP_ADDR=0.0.0.0:7700

ENV PORT=${PORT:-80}

ENTRYPOINT ["/bin/sh", "startup.sh"]
