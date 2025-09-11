FROM rust:1-alpine3.20 AS builder
RUN apk add --no-cache musl-dev gcc g++ libc-dev build-base linux-headers pkgconfig make openssl-dev libffi-dev postgresql-dev
WORKDIR /backend
COPY /backend .
RUN cargo build --release

CMD ["cargo", "run", "--release"]

FROM alpine:3.20 AS production
RUN apk add --no-cache openssl libgcc libstdc++ libpq

WORKDIR /backend
COPY --from=builder /backend/target/release/nr11 .

EXPOSE 8000
CMD ["./nr11"]