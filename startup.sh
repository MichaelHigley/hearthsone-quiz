#!/bin/sh

envsubst < ./nginx.conf.template > /etc/nginx/nginx.conf && cat /etc/nginx/nginx.conf && nginx

# run meilisearch, data bootstrap, and webserver in parallel
meilisearch &
sleep 15 && ./bootstrap &
./webserver --addr "0.0.0.0:4000"
