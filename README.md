# black_jack_rust

A Rust-based web application offering a game of Blackjack.

## Prerequisite

- Ensure you have set up the Rust environment or have a Docker environment ready.

## How to play

Navigate to the `black_jack_rust` directory to start the game.

### Using Docker:

1. Build the Docker image:
docker build -t <desired_container_name> .

2. Run the Docker container:
docker run -p 8080:8080 <same_container_name_as_above>

### Without Docker:

Execute the following command:
cargo run

After starting the application, access it at:
localhost:8080

## Note

- In this version of Blackjack, Jack, Queen, and King are valued at 10 points.
- While traditionally in Blackjack an Ace can be worth 1 or 11 points, in this game, an Ace is always valued at 1 point.

## Libraries & Tools (Licensed under Apache License 2.0)

This section highlights libraries licensed under the Apache License 2.0 used in this project:

- actix-web (version 3.3.2): [GitHub Repository](https://github.com/actix/actix-web)
- actix-rt (version 1.1.1): [GitHub Repository](https://github.com/actix/actix-net)
- thiserror (version 1.0.22): [GitHub Repository](https://github.com/dtolnay/thiserror)
- askama (version 0.10.5): [GitHub Repository](https://github.com/djc/askama)

Note: There are other libraries used in this project, but only those with Apache License 2.0 are highlighted here for specific mention.
