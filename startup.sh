#!/bin/sh

# run meilisearch, data bootstrap, and webserver in parallel
basic-http-server &
/bin/meilisearch &
sleep 5 && ./target/release/bootstrap