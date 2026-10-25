FROM alpine:3.24.3 AS frontend
WORKDIR /src/frontend
RUN apk add npm
COPY src/frontend/package.json src/frontend/package-lock.json ./
RUN npm ci
COPY src/frontend/ ./
RUN npx vite build

FROM rust:alpine3.24 AS backend
RUN rustup toolchain install nightly --profile minimal
WORKDIR /src/backend
COPY src/backend/src ./src
COPY src/backend/Cargo.lock ./
COPY src/backend/Cargo.toml ./
RUN cargo +nightly build --release

FROM alpine:3.24.3 AS final
RUN apk add ca-certificates

WORKDIR /app
COPY --from=backend /src/backend/target/release/backend ./navalyze
COPY --from=frontend /src/frontend/dist ./dist
COPY docker-entrypoint.sh /usr/local/bin/docker-entrypoint.sh

VOLUME /data
EXPOSE 8080
ENTRYPOINT ["docker-entrypoint.sh"]
CMD ["-b", "0.0.0.0:8080"]
