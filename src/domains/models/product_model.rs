use crate::schema::m_products;

#[derive(Queryable, Debug, Clone)]
pub struct Product {
    pub id: uuid::Uuid,
    pub name: String,
    pub price: i64,
    pub seller_id: uuid::Uuid,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub is_deleted: bool,
}

#[derive(Insertable, AsChangeset, Debug, Clone)]
#[table_name = "m_products"]
pub struct NewProduct<'a> {
    pub name: &'a str,
    pub price: i64,
    pub seller_id: &'a uuid::Uuid,
}
