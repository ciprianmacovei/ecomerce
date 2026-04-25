FROM rust:latest AS builder
WORKDIR /backend
COPY /backend .

RUN cargo build --release

EXPOSE 8000
CMD ["cargo", "run", "--release"]

FROM alpine:3.20 AS production
RUN apk add --no-cache openssl libgcc libstdc++ libpq

WORKDIR /backend
COPY --from=builder /backend/target/release/nr11 .

EXPOSE 8000
CMD ["./nr11"]
