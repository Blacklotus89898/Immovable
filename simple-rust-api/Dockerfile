# ==========================================
# STAGE 1: Build
# ==========================================
FROM rust:latest AS builder

# Install MUSL tools + OpenSSL dev libs
RUN apt-get update && apt-get install -y \
    musl-tools \
    pkg-config \
    libssl-dev

# Add MUSL target
RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /app

COPY Cargo.toml ./
COPY src ./src

# Tell openssl-sys where OpenSSL is
ENV OPENSSL_DIR=/usr
ENV OPENSSL_STATIC=1

# Build static binary
RUN cargo build --release --target x86_64-unknown-linux-musl

# ==========================================
# STAGE 2: Runtime
# ==========================================
FROM alpine:3.19

RUN apk add --no-cache ca-certificates

RUN adduser -D appuser
USER appuser

WORKDIR /app

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/simple-rust-api .

EXPOSE 3000

CMD ["./simple-rust-api"]