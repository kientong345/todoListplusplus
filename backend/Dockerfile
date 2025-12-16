# Build stage
FROM rust:latest AS builder

RUN apt-get update && apt-get install -y build-essential

WORKDIR /usr/src/my-checklist

COPY . .

ENV SQLX_OFFLINE=true

RUN cargo build --release


# Production stage
FROM debian:bookworm-slim AS final

RUN apt-get update && apt-get install -y \
    libssl3 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/local/bin

COPY --from=builder /usr/src/my-checklist/target/release/my-checklist .

# copy runtime configuration files
COPY config.json .
COPY migrations ./migrations

EXPOSE 3000

CMD ["./my-checklist"]