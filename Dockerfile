FROM rust:1-bookworm as builder

WORKDIR /usr/src/app
COPY . .
# Will build and cache the binary and dependent crates in release mode
RUN --mount=type=cache,target=/usr/local/cargo,from=rust:latest,source=/usr/local/cargo \
    --mount=type=cache,target=target \
    cargo build --release && mv ./target/release/scrum_poker ./scrum_poker

# Runtime image
FROM debian:bookworm-slim

# Run as "app" user
RUN useradd -ms /bin/bash app

USER app
WORKDIR /app

# Get compiled binaries from builder's cargo install directory
COPY --from=builder /usr/src/app/scrum_poker /app/scrum_poker
COPY --chown=app:app .env /app/.env
COPY --chown=app:app assets /app/assets

# Run the app
CMD ./scrum_poker