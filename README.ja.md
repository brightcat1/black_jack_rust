# black_jack_rust

Blackjackのゲームを提供するRustベースのWebアプリケーションです。

## 必要条件

- Rustの環境を構築するか、Dockerの環境を構築してください。

## 遊び方

`black_jack_rust`ディレクトリに移動してゲームを開始してください。

### Dockerを使用する場合:

1. Dockerイメージをビルド:
docker build -t <希望のコンテナ名> .

2. Dockerコンテナを実行:
docker run -p 8080:8080 <上記で指定したコンテナ名>

### Dockerを使用しない場合:

以下のコマンドを実行:
cargo run

アプリケーションを起動した後、以下のURLでアクセス:
localhost:8080

## 注意点

- このバージョンのBlackjackでは、Jack、Queen、Kingは10ポイントとして評価されます。
- 通常のBlackjackでは、Aceは1ポイントまたは11ポイントとしてカウントすることが多いですが、このゲームではAceは常に1ポイントとしてカウントされます。

## 使用されているライブラリ & ツール (Apache License 2.0に基づいています)

このセクションでは、このプロジェクトで使用されているApache License 2.0に基づくライブラリを取り上げています。

- actix-web (version 3.3.2): [GitHub Repository](https://github.com/actix/actix-web)
- actix-rt (version 1.1.1): [GitHub Repository](https://github.com/actix/actix-net)
- thiserror (version 1.0.22): [GitHub Repository](https://github.com/dtolnay/thiserror)
- askama (version 0.10.5): [GitHub Repository](https://github.com/djc/askama)

注: このプロジェクトには他のライブラリも使用されていますが、ここで紹介しているのはApache License 2.0にライセンスされているもののみです。
