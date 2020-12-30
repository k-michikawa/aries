use crate::domains::models::Product;
use std::io::Error;

pub trait ProductRepository: Sync + Send {
    fn store(&self, product_name: &str, product_price: i64) -> Result<Product, Error>;
    fn scan(&self) -> Result<Vec<Product>, Error>;
    fn find_by_id(&self, key: &uuid::Uuid) -> Result<Product, Error>;
    fn update(
        &self,
        key: &uuid::Uuid,
        product_name: &str,
        product_price: i64,
    ) -> Result<Product, Error>;
    fn delete(&self, key: &uuid::Uuid) -> Result<(), Error>;
}
