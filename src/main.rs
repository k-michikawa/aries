#[macro_use]
extern crate diesel;

mod domains;
mod infrastructures;
mod interfaces;
mod schema;
mod use_cases;
// .protoファイルを.rsで表現したものを読み込んでおく
pub mod aries {
    tonic::include_proto!("aries");
}

use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 環境変数の読み込み
    let listen_address = env::var("LISTEN_ADDRESS").expect("LISTEN_ADDRESS must be set");
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Databaseのコネクションプールを持ったstruct作る
    let database = infrastructures::database::Database::new(&database_url, 5);

    // injectorみたいの作ってもよいかも
    let context = interfaces::controllers::Context { database };

    // ルーティング(増えたら切り出してもよいかも)
    infrastructures::tonic_server::get_server()
        .add_service(interfaces::controllers::ProductController::new(context))
        .serve(listen_address.parse().unwrap())
        .await?;
    Ok(())
}