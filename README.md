# aries

### Description

Rust で tonic と diesel 使って gRPC で CRUD するやつ

Rust 1.48.0

検証用に grpcurl 入れておくと良いかも

mac: `brew install grpcurl`

example:

```sh
$ grpcurl -plaintext \
          -import-path ./proto \
          -proto product.proto \
          -d '{"name": "aries", "price": 100, "seller_id": "940f5b36-621a-48ea-af39-55647befafbc"}'
          localhost:9000 \
          aries.ProductService/PostProduct
```

### tools

migration ツールのインストール

```sh
$ cargo install diesel_cli --no-default-features --features "postgres"
```

migration の注意点

- 接続先の設定が必要
- 環境変数 or .env ファイルを作って`DATABASE_URL=postgres://postgres:password@localhost:5432/aries-db`を設定しておく

protodep のインストール

```sh
$ go get github.com/stormcat24/protodep
```

OR

```sh
$ wget https://github.com/stormcat24/protodep/releases/download/0.0.8/protodep_darwin_amd64.tar.gz
$ tar -xf protodep_darwin_amd64.tar.gz
$ mv protodep /usr/local/bin/
```

AFTER

```sh
$ ssh-add ~/.ssh/id_rsa
```

### run

起動するまで

1. rustup とかで Rust の環境作っておく
2. protodep 落としてくる
3. `$ protodep up`
4. `$ docker-compose run -d --service-ports aries-db`
5. `$ diesel migration run`
6. `$ docker stop /* 4で立ち上げた container name */`
7. `$ docker-compose up`
