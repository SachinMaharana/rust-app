FROM ekidd/rust-musl-builder AS builder

ADD . ./

RUN sudo chown -R rust:rust /home/rust

RUN cargo build --release

# Final Build
FROM alpine:latest

RUN apk --no-cache add ca-certificates

COPY --from=builder \
    /home/rust/src/target/x86_64-unknown-linux-musl/release/rust-app \
    /usr/local/bin/

CMD /usr/local/bin/rust-app
