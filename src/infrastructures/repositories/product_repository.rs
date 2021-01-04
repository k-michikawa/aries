use crate::domains::{models, repositories};
use crate::infrastructures::database::{Database, DatabaseModule};
use crate::schema::m_products;
use crate::schema::m_products::dsl;
use diesel::{expression_methods::*, prelude::*};
use std::io::{Error, ErrorKind};
use std::sync::Arc;

pub trait ProductRepositoryModule: repositories::ProductRepository {}

pub struct ProductRepository {
    pub database: Arc<Database>,
}

// 砲塔は Module for Repository としたいがsupertraitとsubtraitは別なので・・・
impl repositories::ProductRepository for ProductRepository {
    fn store(
        &self,
        name: &str,
        price: i64,
        seller_id: &uuid::Uuid,
    ) -> Result<models::Product, Error> {
        let connection = &self.database.get_connection()?;
        let new_product = models::NewProduct {
            name,
            price,
            seller_id,
        };

        connection
            .transaction::<_, diesel::result::Error, _>(|| {
                diesel::insert_into(m_products::table)
                    .values(&new_product)
                    .get_result::<models::Product>(connection)
            })
            .map_err(|e| Error::new(ErrorKind::Other, e))
    }
    fn list(&self) -> Result<Vec<models::Product>, Error> {
        let connection = &self.database.get_connection()?;

        dsl::m_products
            .filter(m_products::is_deleted.eq(false))
            .load::<models::Product>(connection)
            .map_err(|e| Error::new(ErrorKind::Other, e))
    }
    fn list_by_seller_id(&self, seller_id: &uuid::Uuid) -> Result<Vec<models::Product>, Error> {
        let connection = &self.database.get_connection()?;

        dsl::m_products
            .filter(m_products::is_deleted.eq(false))
            .filter(m_products::seller_id.eq(seller_id))
            .load::<models::Product>(connection)
            .map_err(|e| Error::new(ErrorKind::Other, e))
    }
    fn find(&self, key: &uuid::Uuid) -> Result<models::Product, Error> {
        let connection = &self.database.get_connection()?;

        dsl::m_products
            .find(key)
            .get_result::<models::Product>(connection)
            .map_err(|e| Error::new(ErrorKind::Other, e))
    }
    fn update(
        &self,
        key: &uuid::Uuid,
        product_name: &str,
        product_price: i64,
    ) -> Result<models::Product, Error> {
        let connection = &self.database.get_connection()?;

        connection
            .transaction::<_, diesel::result::Error, _>(|| {
                let source = dsl::m_products.find(key);
                diesel::update(source)
                    .set((
                        m_products::name.eq_all(product_name),
                        m_products::price.eq_all(product_price),
                    ))
                    .get_result::<models::Product>(connection)
            })
            .map_err(|e| Error::new(ErrorKind::Other, e))
    }
    fn delete(&self, key: &uuid::Uuid) -> Result<(), Error> {
        let connection = &self.database.get_connection()?;

        let response = connection
            .transaction::<_, diesel::result::Error, _>(|| {
                let source = dsl::m_products.find(key);
                diesel::update(source)
                    .set(m_products::is_deleted.eq(true))
                    .execute(connection)
            })
            .map_err(|e| Error::new(ErrorKind::Other, e));
        match response {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}
