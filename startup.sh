#!/bin/sh

# substitube environment variables into nginx config
envsubst < ./nginx.conf.template > /etc/nginx/nginx.conf
echo "Generated nginx.confg:"
cat /etc/nginx/nginx.conf

# run nginx, meilisearch, data bootstrap, and webserver in parallel
nginx
meilisearch &
sleep 15 && ./bootstrap &
./webserver --addr "0.0.0.0:4000"
