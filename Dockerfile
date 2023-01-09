FROM rust:1.66.0 as build
COPY . /rust_docker_app
WORKDIR /rust_docker_app

RUN --mount=type=cache,target=/usr/local/cargo/registry --mount=type=cache,target=/root/target \
    cargo build --release
 
FROM debian:buster-slim as runtime
COPY --from=build /rust_docker_app/target/release/rust_docker_app .
CMD ["./rust_docker_app"]