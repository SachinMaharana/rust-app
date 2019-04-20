# select build image
FROM rust:1.34 as build

# create a new empty shell project
RUN USER=root cargo new --bin rust-app
WORKDIR /rust-app

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN rm ./target/release/deps/rust-app*
RUN cargo build --release

# our final base
FROM rust:1.34

# copy the build artifact from the build stage
COPY --from=build /rust-app/target/release/my_project .

# set the startup command to run your binary
CMD ["./rust-app"]
