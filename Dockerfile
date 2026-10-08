FROM node:22-bookworm-slim AS frontend
WORKDIR /src/frontend
COPY src/frontend/package.json src/frontend/package-lock.json ./
RUN npm ci
COPY src/frontend/ ./
RUN npx vite build

FROM rust:1-bookworm AS backend
RUN rustup toolchain install nightly --profile minimal
WORKDIR /src/backend
COPY src/backend/ ./
RUN cargo +nightly build --release

FROM debian:bookworm-slim
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=backend /src/backend/target/release/backend ./navalyze
COPY --from=frontend /src/frontend/dist ./dist
COPY settings.toml ./settings.toml
COPY docker-entrypoint.sh /usr/local/bin/docker-entrypoint.sh

VOLUME /data
EXPOSE 8080
ENTRYPOINT ["docker-entrypoint.sh"]
CMD ["-b", "0.0.0.0:8080"]
