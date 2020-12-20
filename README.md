# black_jack_rust
## Prerequisite
Rust environment construction or Docker environment construction is required
## How to play
- You can do this game by doing following command in black_jack_rust directory.
If you will use docker to run this game, please run
```
docker build -t <container name you want> .
```
Then, run
```
docker run -p 8080:8080 <ame container name as declared at build time above>
```
If you will not use docker to run this game, please run
```
cargo run
```
After running above command, you can play this game by accessing
```
localhost:8080
```
## Note
Jack, Queen, King are interpreted as 10 points in this game.
Ace can add 1 or 10 points according to the original rules of blackjack, but in this game only 1 point will be added.
