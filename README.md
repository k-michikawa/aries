# aries

Rust で tonic と diesel 使って gRPC で CRUD するやつ

Dockerfile に diesel migration 書いてないので注意

検証用に grpcurl 入れておくと良いかも

mac: `brew install grpcurl`

example: `grpcurl -plaintext -import-path ./proto -proto product.proto -d '{"name": "aries", "price": 100}' localhost:9000 aries.ProductService/PostProduct`

- run-development
  `docker-compose up -d`

- compile-production
  `docker build -t aries_prod .`
