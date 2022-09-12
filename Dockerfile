# STAGE 1
FROM rust:alpine3.15 AS builder
# Install dependencies
RUN apk add musl-dev --no-cache
# Copy source code.
COPY src /src
COPY Cargo.toml Cargo.lock /
# Build rust binaries.
RUN cargo build --release --manifest-path /Cargo.toml

# STAGE 2
FROM alpine:3.15 as base
# Copy only the worker binary and execute.
COPY --from=builder /target/release/minesweeper /minesweeper
COPY ./example.txt /example.txt 
# Run binaries
CMD ["./minesweeper"]
