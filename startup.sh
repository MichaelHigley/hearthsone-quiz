#!/bin/sh

# substitube environment variables into nginx config
cat nginx.conf.template | sed "s/PORT/$PORT/g" > /etc/nginx/nginx.conf
echo "Generated nginx.confg:"
cat /etc/nginx/nginx.conf

# run nginx, meilisearch, data bootstrap, and webserver in parallel
nginx
./meilisearch &
sleep 5 && ./bootstrap &
python3 -m http.server 4000
