use crate::aries;
use crate::infrastructures::database::Database;
use crate::infrastructures::repositories::product_repository::ProductRepository;
use crate::interfaces::service::ProductService;
use crate::use_cases::product_use_case::ProductUseCase;
use std::sync::Arc;

pub fn inject_product_repository(db: Arc<Database>) -> Box<ProductRepository> {
    Box::new(ProductRepository { database: db })
}

pub fn inject_product_use_case(db: Arc<Database>) -> Box<ProductUseCase> {
    Box::new(ProductUseCase {
        repository: inject_product_repository(db),
    })
}

pub fn inject_product_service(
    db: Arc<Database>,
) -> aries::product_service_server::ProductServiceServer<ProductService> {
    aries::product_service_server::ProductServiceServer::new(ProductService {
        use_case: inject_product_use_case(db),
    })
}
