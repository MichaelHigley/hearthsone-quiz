FROM rust:alpine as rust-builder
WORKDIR /home/rust/src
RUN apk --no-cache add musl-dev openssl-dev
COPY src/ src/
COPY Cargo.* .
RUN cargo build --release


FROM node:alpine as node-builder
WORKDIR /home/node/src
COPY index.html . 


FROM getmeili/meilisearch 
# install nginx
RUN apk --no-cache add nginx
COPY nginx.conf nginx.conf.template
# simple webserver
RUN apk --no-cache add python3
# copy bootstrap executable from rust-builder
COPY --from=rust-builder /home/rust/src/target/release/sound-quiz ./bootstrap
COPY --from=node-builder /home/node/src/index.html .
COPY startup.sh .

ENV RUST_LOG=info
ENV MEILI_HTTP_ADDR=0.0.0.0:7700

ENV PORT=${PORT:-80}

ENTRYPOINT ["/bin/sh", "startup.sh"]