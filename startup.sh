#!/bin/sh
# run meilisearch, data bootstrap, and webserver in parallel
meilisearch &
sleep 15 && ./bootstrap &
./webserver --addr "0.0.0.0:${PORT:-80}"