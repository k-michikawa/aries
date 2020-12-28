use crate::domains::models::{NewProduct, Product};
use crate::interfaces::controllers::Context;
use crate::schema::m_products;
use crate::schema::m_products::dsl;
use crate::use_cases::product_repository_use_case::ProductRepositoryUseCase;
use diesel::{expression_methods::*, prelude::*};
use std::io::{Error, ErrorKind};

pub struct ProductRepository;

// repositoryはuse caseに依存するので抽象を持ってきて実装をもたせる()
impl ProductRepositoryUseCase for ProductRepository {
    fn store(context: &Context, product_name: &str, product_price: i64) -> Result<Product, Error> {
        let connection = context.database.get_connection()?;
        let new_product = NewProduct {
            name: product_name,
            price: product_price,
        };

        connection
            .transaction::<_, diesel::result::Error, _>(|| {
                diesel::insert_into(m_products::table)
                    .values(&new_product)
                    .get_result::<Product>(&connection)
            })
            .map_err(|e| Error::new(ErrorKind::Other, e))
    }
    fn scan(context: &Context) -> Result<Vec<Product>, Error> {
        let connection = context.database.get_connection()?;

        dsl::m_products
            .load::<Product>(&connection)
            .map_err(|e| Error::new(ErrorKind::Other, e))
    }
    fn find_by_id(context: &Context, key: &uuid::Uuid) -> Result<Product, Error> {
        let connection = context.database.get_connection()?;

        dsl::m_products
            .find(key)
            .get_result::<Product>(&connection)
            .map_err(|e| Error::new(ErrorKind::Other, e))
    }
    fn update(
        context: &Context,
        key: &uuid::Uuid,
        product_name: &str,
        product_price: i64,
    ) -> Result<Product, Error> {
        let connection = context.database.get_connection()?;

        connection
            .transaction::<_, diesel::result::Error, _>(|| {
                let source = dsl::m_products.find(key);
                diesel::update(source)
                    .set((
                        m_products::name.eq_all(product_name),
                        m_products::price.eq_all(product_price),
                    ))
                    .get_result::<Product>(&connection)
            })
            .map_err(|e| Error::new(ErrorKind::Other, e))
    }
    fn delete(context: &Context, key: &uuid::Uuid) -> Result<(), Error> {
        let connection = context.database.get_connection()?;

        let response = connection
            .transaction::<_, diesel::result::Error, _>(|| {
                let source = dsl::m_products.find(key);
                diesel::update(source)
                    .set(m_products::is_deleted.eq(true))
                    .execute(&connection)
            })
            .map_err(|e| Error::new(ErrorKind::Other, e));
        match response {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}
