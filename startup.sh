#!/bin/sh
# run meilisearch, data bootstrap, and webserver in parallel
basic-http-server --addr "0.0.0.0:${PORT:-4000}" &
/bin/meilisearch &
# sleep 5 && ./bootstrap