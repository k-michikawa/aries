use crate::domains::models::Product;
use crate::domains::repositories::ProductRepository;
use std::io::Error;

pub trait ProductUseCaseModule {
    fn store(&self, name: &str, price: i64, seller_id: &uuid::Uuid) -> Result<Product, Error>;
    fn list(&self) -> Result<Vec<Product>, Error>;
    fn list_by_seller_id(&self, seller_id: &uuid::Uuid) -> Result<Vec<Product>, Error>;
    fn find(&self, key: &uuid::Uuid) -> Result<Product, Error>;
    fn update(&self, key: &uuid::Uuid, name: &str, price: i64) -> Result<Product, Error>;
    fn delete(&self, key: &uuid::Uuid) -> Result<(), Error>;
}

pub struct ProductUseCase {
    pub repository: Box<dyn ProductRepository>,
}

impl ProductUseCaseModule for ProductUseCase {
    fn store(&self, name: &str, price: i64, seller_id: &uuid::Uuid) -> Result<Product, Error> {
        self.repository.store(name, price, seller_id)
    }
    fn list(&self) -> Result<Vec<Product>, Error> {
        self.repository.list()
    }
    fn list_by_seller_id(&self, seller_id: &uuid::Uuid) -> Result<Vec<Product>, Error> {
        self.repository.list_by_seller_id(seller_id)
    }
    fn find(&self, key: &uuid::Uuid) -> Result<Product, Error> {
        self.repository.find(key)
    }
    fn update(&self, key: &uuid::Uuid, name: &str, price: i64) -> Result<Product, Error> {
        self.repository.update(key, name, price)
    }
    fn delete(&self, key: &uuid::Uuid) -> Result<(), Error> {
        self.repository.delete(key)
    }
}
