use tonic::{Code, Request, Response, Status};

// tonicが.protoを元に自動生成するやつに抽象や型が全て入っているのでそれを持ってくる
use crate::aries;
use crate::aries::*;
use crate::use_cases::product_use_case::*;

pub trait ProductServiceModule: product_service_server::ProductService {}

pub struct ProductService {
    pub use_case: Box<ProductUseCase>,
}

// 本当は Module for Service としたいが、supertraitとsubtraitは別物なので・・・
#[tonic::async_trait]
impl product_service_server::ProductService for ProductService {
    async fn post_product(
        &self,
        request: Request<PostProductRequest>,
    ) -> Result<Response<PostProductResponse>, Status> {
        println!("{:?}", request);

        let into_request = request.into_inner();
        match &self.use_case.store(&into_request.name, into_request.price) {
            Ok(product) => {
                let message = PostProductResponse {
                    product: Some(Product {
                        id: product.id.to_string(),
                        name: product.name.to_owned(),
                        price: product.price,
                        created_at: product.created_at.timestamp(),
                        updated_at_oneof: None,
                    }),
                };
                Ok(Response::new(message))
            }
            Err(e) => Err(Status::new(Code::Unknown, format!("{}", e))),
        }
    }

    async fn list_product(
        &self,
        request: Request<ListProductRequest>,
    ) -> Result<Response<ListProductResponse>, Status> {
        println!("{:?}", request);

        match &self.use_case.scan() {
            Ok(products) => {
                let message = ListProductResponse {
                    products: products
                        .iter()
                        .map(|product| Product {
                            id: product.id.to_string(),
                            name: product.name.to_owned(),
                            price: product.price,
                            created_at: product.created_at.timestamp(),
                            updated_at_oneof: product.updated_at.map(|updated_at| {
                                aries::product::UpdatedAtOneof::UpdatedAt(updated_at.timestamp())
                            }),
                        })
                        .collect::<Vec<Product>>(),
                };
                Ok(Response::new(message))
            }
            Err(e) => Err(Status::new(Code::Unknown, format!("{}", e))),
        }
    }

    async fn find_product(
        &self,
        request: Request<FindProductRequest>,
    ) -> Result<Response<FindProductResponse>, Status> {
        println!("{:?}", request);

        let into_request = request.into_inner();
        let uuid = match uuid::Uuid::parse_str(&into_request.id) {
            Ok(v) => v,
            Err(e) => return Err(Status::new(Code::Unknown, format!("{}", e))),
        };

        match &self.use_case.find_by_id(&uuid) {
            Ok(product) => {
                let message = FindProductResponse {
                    product: Some(Product {
                        id: product.id.to_string(),
                        name: product.name.to_owned(),
                        price: product.price,
                        created_at: product.created_at.timestamp(),
                        updated_at_oneof: product.updated_at.map(|updated_at| {
                            aries::product::UpdatedAtOneof::UpdatedAt(updated_at.timestamp())
                        }),
                    }),
                };
                Ok(Response::new(message))
            }
            Err(e) => Err(Status::new(Code::Unknown, format!("{}", e))),
        }
    }

    async fn update_product(
        &self,
        request: Request<UpdateProductRequest>,
    ) -> Result<Response<UpdateProductResponse>, Status> {
        println!("{:?}", request);

        let into_request = request.into_inner();
        let uuid = match uuid::Uuid::parse_str(&into_request.id) {
            Ok(v) => v,
            Err(e) => return Err(Status::new(Code::Unknown, format!("{}", e))),
        };

        match &self
            .use_case
            .update(&uuid, &into_request.name, into_request.price)
        {
            Ok(product) => {
                let message = UpdateProductResponse {
                    product: Some(Product {
                        id: product.id.to_string(),
                        name: product.name.to_owned(),
                        price: product.price,
                        created_at: product.created_at.timestamp(),
                        updated_at_oneof: product.updated_at.map(|updated_at| {
                            aries::product::UpdatedAtOneof::UpdatedAt(updated_at.timestamp())
                        }),
                    }),
                };
                Ok(Response::new(message))
            }
            Err(e) => Err(Status::new(Code::Unknown, format!("{}", e))),
        }
    }

    async fn delete_product(
        &self,
        request: Request<DeleteProductRequest>,
    ) -> Result<Response<DeleteProductResponse>, Status> {
        println!("{:?}", request);

        let into_request = request.into_inner();
        let uuid = match uuid::Uuid::parse_str(&into_request.id) {
            Ok(v) => v,
            Err(e) => return Err(Status::new(Code::Unknown, format!("{}", e))),
        };

        match &self.use_case.delete(&uuid) {
            Ok(_) => {
                let message = DeleteProductResponse {};
                Ok(Response::new(message))
            }
            Err(e) => Err(Status::new(Code::Unknown, format!("{}", e))),
        }
    }
}
