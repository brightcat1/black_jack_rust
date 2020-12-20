# black_jack_rust
## Prerequisite
Rust environment construction or Docker environment construction is required
## How to play
- You can do this game by doing following command in black_jack_rust directory.
If you will use docker to run this game, please run
```
docker build -t <container-name you want> .
```
Then, run
```
docker run -p 8080:8080 <container-name you want>
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
Jack, Queen, King are interpreted as 10 points in this game, but Ace can be originally selected as 10 or 1, but in this game it is interpreted as only 1 point.
