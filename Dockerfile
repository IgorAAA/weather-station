# ----- build stage -----
FROM rust:1.85 as builder
WORKDIR /app

# Faster incremental builds: cache deps first
COPY Cargo.toml Cargo.lock ./
# If you use a workspace, also copy the workspace members' Cargo.toml files
COPY current-weather current-weather
COPY weather-forecast weather-forecast
COPY model model
COPY influx influx
COPY http-client http-client

# Now copy real source and build
#COPY . .
ARG BIN_NAME
RUN cargo build --release -p ${BIN_NAME}

# ----- runtime stage -----
FROM debian:stable-slim
# If you need OpenSSL/zlib/etc, install here:
# RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app

ARG BIN_NAME
COPY --from=builder /app/target/release/${BIN_NAME} /usr/local/bin/app
# If you need static files/configs:
# COPY config/ ./config/
EXPOSE 3001
CMD ["app"]
