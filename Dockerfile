# "Inspired" by
# https://alexbrand.dev/post/how-to-package-rust-applications-into-minimal-docker-containers/
# Dockerfile for creating a statically-linked Rust application using docker's
# multi-stage build feature. This also leverages the docker build cache to avoid
# re-downloading dependencies if they have not changed.
FROM rust:1.44 AS build
WORKDIR /usr/src

# Download the target for static linking.
RUN apt-get update && apt-get install musl-tools -y && \
    rustup target add x86_64-unknown-linux-musl

# Copy the source and build the application.
COPY . .
RUN cargo install --target x86_64-unknown-linux-musl --path .

# Copy the statically-linked binary into a scratch container.
FROM scratch
COPY --from=build /usr/local/cargo/bin/bc .
USER 1000

EXPOSE 8088

ARG BUILD_DATE
ARG VCS_REF
LABEL \
    org.opencontainers.image.created=$BUILD_DATE \
    org.opencontainers.image.authors="https://alessiogiambrone.github.io" \
    org.opencontainers.image.url="https://hub.docker.com/r/alessiogiambrone/bc/" \
    org.opencontainers.image.source="https://github.com/alessiogiambrone/bc" \
    org.opencontainers.image.version=$VCS_REF \
    org.opencontainers.image.revision=$VCS_REF \
    org.opencontainers.image.title="bc" \
    org.opencontainers.image.description="A simple blockchain implementation in Rust"

ENTRYPOINT ["./bc"]
