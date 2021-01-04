use crate::domains::models::Product;
use std::io::Error;

pub trait ProductRepository: Sync + Send {
    fn store(&self, name: &str, price: i64, seller_id: &uuid::Uuid) -> Result<Product, Error>;
    fn list(&self) -> Result<Vec<Product>, Error>;
    fn list_by_seller_id(&self, seller_id: &uuid::Uuid) -> Result<Vec<Product>, Error>;
    fn find(&self, key: &uuid::Uuid) -> Result<Product, Error>;
    fn update(&self, key: &uuid::Uuid, name: &str, price: i64) -> Result<Product, Error>;
    fn delete(&self, key: &uuid::Uuid) -> Result<(), Error>;
}
