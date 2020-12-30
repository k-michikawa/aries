# aries

Rust で tonic と diesel 使って gRPC で CRUD するやつ

Dockerfile に diesel migration 書いてないので注意

検証用に grpcurl 入れておくと良いかも

mac: `brew install grpcurl`

example: `grpcurl -plaintext -import-path ./proto -proto product.proto -d '{"name": "aries", "price": 100}' localhost:9000 aries.ProductService/PostProduct`

migration ツールのインストール

- `cargo install diesel_cli --no-default-features --features "postgres"`

その他コマンド

- run-development
  `docker-compose up -d`

- compile-production
  `docker build -t aries_prod .`

- migration
  環境変数 or .env ファイル作って`DATABASE_URL=postgres://postgres:password@localhost:5432/aries-db`を設定する
  `diesel migration run`
