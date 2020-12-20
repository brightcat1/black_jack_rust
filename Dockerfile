FROM rust:1.48 AS builder

WORKDIR /black_jack_rust
COPY Cargo.toml Cargo.toml
RUN mkdir src
RUN echo "fn main(){}" > src/main.rs
RUN cargo build --release
COPY ./src ./src
COPY ./templates ./templates
RUN rm -f target/release/deps/black_jack_rust*
RUN cargo build --release

FROM debian:10.7
COPY --from=builder /black_jack_rust/target/release/black_jack_rust /usr/local/bin/black_jack_rust
CMD ["black_jack_rust"]