# Use the official Rust image for building
FROM rust:latest as build
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Copy the source code
COPY src ./src

# Build the project in release mode
RUN cargo build --release 

# Use a minimal base image to run the application
FROM gcr.io/distroless/cc

# Copy the built binary from the build stage
COPY --from=build /app/target/release/fibonacci /usr/local/bin/

# Set the entrypoint to run the application
ENTRYPOINT ["/usr/local/cargo/bin/cargo", "run", "--", "--enable-feature", "${{ inputs.enable_feature }}", "--max-threshold", "${{ inputs.max_threshold }}"]
