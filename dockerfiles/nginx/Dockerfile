FROM georgjung/nginx-brotli AS builder
COPY ./dockerfiles/nginx/custom.conf /etc/nginx/conf.d/custom.conf
COPY ./dockerfiles/nginx/nginx.conf /etc/nginx/nginx.conf

FROM georgjung/nginx-brotli AS production
COPY ./dockerfiles/nginx/prod.custom.conf /etc/nginx/conf.d/custom.conf
COPY ./dockerfiles/nginx/prod.default.conf /etc/nginx/conf.d/default.conf
COPY ./dockerfiles/nginx/nginx.conf /etc/nginx/nginx.conf
