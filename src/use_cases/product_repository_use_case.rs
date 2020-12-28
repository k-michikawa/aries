use crate::domains::models::Product;
use crate::domains::repositories::ProductRepository;
use crate::interfaces::controllers::Context;
use std::io::Error;

pub trait ProductRepositoryUseCase {
    fn store(context: &Context, name: &str, price: i64) -> Result<Product, Error>;
    fn scan(context: &Context) -> Result<Vec<Product>, Error>;
    fn find_by_id(context: &Context, key: &uuid::Uuid) -> Result<Product, Error>;
    fn update(
        context: &Context,
        key: &uuid::Uuid,
        name: &str,
        price: i64,
    ) -> Result<Product, Error>;
    fn delete(context: &Context, key: &uuid::Uuid) -> Result<(), Error>;
}

pub struct ProductUseCase;

impl ProductRepositoryUseCase for ProductUseCase {
    fn store(context: &Context, name: &str, price: i64) -> Result<Product, Error> {
        ProductRepository::store(context, name, price)
    }
    fn scan(context: &Context) -> Result<Vec<Product>, Error> {
        ProductRepository::scan(context)
    }
    fn find_by_id(context: &Context, key: &uuid::Uuid) -> Result<Product, Error> {
        ProductRepository::find_by_id(context, key)
    }
    fn update(
        context: &Context,
        key: &uuid::Uuid,
        name: &str,
        price: i64,
    ) -> Result<Product, Error> {
        ProductRepository::update(context, key, name, price)
    }
    fn delete(context: &Context, key: &uuid::Uuid) -> Result<(), Error> {
        ProductRepository::delete(context, key)
    }
}
