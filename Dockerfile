FROM rust:1.89.0-alpine3.22 AS builder
RUN apk add --no-cache build-base
WORKDIR /app
COPY ./app .
RUN cargo build --release

FROM alpine:3.22.1
WORKDIR /app
COPY --from=builder /app/target/release/app .
EXPOSE 3000
CMD ["./app"]
