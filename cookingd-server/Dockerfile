FROM messense/rust-musl-cross:aarch64-musl as builder
ENV SQLX_OFFLINE=true
ENV CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_LINKER=aarch64-linux-gnu-gcc
ENV CC=aarch64-linux-gnu-gcc
WORKDIR /cookingd-server

# Copy source code from previous stage
COPY . .
# Generate info for caching dependencies

# Build application
RUN rustup target add aarch64-unknown-linux-musl
RUN cargo build --release --target aarch64-unknown-linux-musl

# Create a new stage with a minimal image
FROM scratch
COPY --from=builder /cookingd-server/target/aarch64-unknown-linux-musl/release/cookingd-server /cookingd-server
ENTRYPOINT ["/cookingd-server"]
EXPOSE 3000