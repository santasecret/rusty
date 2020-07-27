FROM rust:1.45.0-alpine3.11 AS builder

# Add compilation target for later scratch container
ENV RUST_TARGETS="x86_64-unknown-linux-musl"
ENV PACKAGE_NAME="rusty"

# Creating a placeholder project
RUN set -ex \
  && apk add --no-cache \
    musl-dev \
  && cd / \
  && USER=root cargo new $PACKAGE_NAME
WORKDIR /$PACKAGE_NAME

# moving deps info to cache dependencies
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --target $RUST_TARGETS --release
RUN set -ex \
  && rm target/x86_64-unknown-linux-musl/release/deps/rust* \
  && rm src/*.rs

# Replacing with actual src
COPY ./src ./src

# To compile only code changes
RUN cargo build --target $RUST_TARGETS --release

# This creates a TINY container with only the executable!
FROM scratch
ENV PACKAGE_NAME="rusty"
COPY --from=builder /$PACKAGE_NAME/target/x86_64-unknown-linux-musl/release/$PACKAGE_NAME .
USER 1000
CMD ["./rusty"]

