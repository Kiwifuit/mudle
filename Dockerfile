FROM rust:1.64 as build

# Copy files to /build
WORKDIR /build
COPY . .

RUN rustup target add x86_64-unknown-linux-musl

# Test and Build
RUN cargo test
RUN cargo build --release --target x86_64-unknown-linux-musl

# Copy executable to final image
FROM alpine:latest

# Copy executable
WORKDIR /app
COPY --from=build /build/target/x86_64-unknown-linux-musl/release/mtui .

# Add resources to the app
COPY res res


VOLUME [ "/app/res" ]

# Run the tui
ENTRYPOINT [ "./mtui" ]