FROM alpine AS builder
WORKDIR /usr/src/app
COPY . .
RUN apk add --no-cache musl-dev openssl-dev build-base rust cargo
RUN cargo fetch && cargo build --release

FROM alpine
RUN apk add --no-cache openssl musl libgcc
COPY --from=builder /usr/src/app/target/release/gaia-auth /usr/local/bin/gaia-auth
CMD ["gaia-auth"]
EXPOSE 4000