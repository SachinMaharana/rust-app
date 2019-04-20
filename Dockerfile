From rust:slim as build
RUN USER=root cargo new --bin builddir
workdir /builddir

add Cargo.* /builddir/
run cargo build --release && rm src/*.rs
COPY ./src ./src

# build for release
RUN cargo clean && cargo build --release

From bitnami/minideb:stretch
