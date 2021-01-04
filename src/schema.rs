table! {
    m_products (id) {
        id -> Uuid,
        name -> Text,
        price -> Int8,
        seller_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        is_deleted -> Bool,
    }
}
