FROM rust:slim as build
RUN USER=root cargo new --bin builddir
WORKDIR /builddir

ADD Cargo.* /builddir/
RUN cargo build --release && rm src/*.rs
COPY ./src ./src

# build for release
RUN cargo clean && cargo build --release

FROM alpine:latest
COPY --from=build /builddir/target/release/rust-app /app
CMD ["/app"]