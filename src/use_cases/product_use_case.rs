use crate::domains::models::Product;
use crate::domains::repositories::ProductRepository;
use std::io::Error;

pub trait ProductUseCaseModule {
    fn store(&self, name: &str, price: i64) -> Result<Product, Error>;
    fn scan(&self) -> Result<Vec<Product>, Error>;
    fn find_by_id(&self, key: &uuid::Uuid) -> Result<Product, Error>;
    fn update(&self, key: &uuid::Uuid, name: &str, price: i64) -> Result<Product, Error>;
    fn delete(&self, key: &uuid::Uuid) -> Result<(), Error>;
}

pub struct ProductUseCase {
    pub repository: Box<dyn ProductRepository>,
}

impl ProductUseCaseModule for ProductUseCase {
    fn store(&self, name: &str, price: i64) -> Result<Product, Error> {
        self.repository.store(name, price)
    }
    fn scan(&self) -> Result<Vec<Product>, Error> {
        self.repository.scan()
    }
    fn find_by_id(&self, key: &uuid::Uuid) -> Result<Product, Error> {
        self.repository.find_by_id(key)
    }
    fn update(&self, key: &uuid::Uuid, name: &str, price: i64) -> Result<Product, Error> {
        self.repository.update(key, name, price)
    }
    fn delete(&self, key: &uuid::Uuid) -> Result<(), Error> {
        self.repository.delete(key)
    }
}
