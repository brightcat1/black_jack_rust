FROM rust:1.48

WORKDIR /black_jack
COPY Cargo.toml Cargo.toml
COPY ./src ./src
COPY ./templates ./templates
RUN cargo build --release
RUN cargo install --path .
CMD ["black_jack_rust"]