ARG TARGETPLATFORM
ARG TARGETARCH

FROM rust:slim-bookworm AS build 
ARG TARGETARCH
RUN apt-get update && apt-get install -y \
    musl-dev \
    libssl-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

RUN mkdir app
WORKDIR /app
COPY . .
RUN /bin/bash -c 'AARCH="${TARGETARCH/amd64/x86_64}";RUST_TARGET="${AARCH/arm64/aarch64}-unknown-linux-musl";rustup target add ${RUST_TARGET}'
RUN cargo check

RUN /bin/bash -c 'AARCH="${TARGETARCH/amd64/x86_64}";RUST_TARGET="${AARCH/arm64/aarch64}-unknown-linux-musl";cargo build --release --target ${RUST_TARGET}'
FROM alpine:3.22
ARG TARGETARCH
ARG AARCH="${TARGETARCH/amd64/x86_64}"
ARG RUST_TARGET="${AARCH/arm64/aarch64}-unknown-linux-musl"
COPY --from=build /app/target/${RUST_TARGET}/release/ctf-bot /usr/local/bin/ctf-bot
ENTRYPOINT ["/usr/local/bin/ctf-bot"]