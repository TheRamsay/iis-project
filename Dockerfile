FROM rust:1.80 AS builder

WORKDIR /usr/src/app

# Copy the Cargo.toml and Cargo.lock to cache dependencies
COPY Cargo.toml Cargo.lock ./

# Copy the source code and fetch dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build --release

# Remove the placeholder source code and copy the actual source
RUN rm -rf src
COPY . .

# Build the application in release mode
RUN cargo build --release

# Stage 2: Create a minimal runtime environment
FROM debian:bullseye-slim AS runtime

# Install required system dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Set the working directory for the runtime
WORKDIR /app

# Copy the built binary from the builder stage
COPY --from=builder /usr/src/app/target/release/api .

# Expose the application's port (adjust as needed)
EXPOSE 8000

# Run the application
CMD ["./api"]
