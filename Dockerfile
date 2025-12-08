# Compilación
FROM rust:1.75 as builder

WORKDIR /app

COPY Cargo.toml Cargo.lock* ./
COPY src ./src
COPY migrations ./migrations

RUN cargo build --release

# Imagen final
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/integrador /usr/local/bin/

EXPOSE 3000

CMD ["integrador"]
