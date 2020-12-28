use tonic::{Code, Request, Response, Status};

// tonicが.protoを元に自動生成するやつに抽象や型が全て入っているのでそれを持ってくる
use crate::aries;
use crate::aries::*;
use crate::interfaces::controllers::Context;
use crate::use_cases::product_repository_use_case::*;

pub struct ProductController {
    context: Context,
}

#[tonic::async_trait]
impl product_service_server::ProductService for ProductController {
    async fn post_product(
        &self,
        request: Request<PostProductRequest>,
    ) -> Result<Response<PostProductResponse>, Status> {
        println!("{:?}", request);

        let into_request = request.into_inner();
        match ProductUseCase::store(&self.context, &into_request.name, into_request.price) {
            Ok(product) => {
                let message = PostProductResponse {
                    product: Some(Product {
                        id: product.id.to_string(),
                        name: product.name,
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

        match ProductUseCase::scan(&self.context) {
            Ok(products) => {
                let message = ListProductResponse {
                    products: products
                        .iter()
                        .map(|product| Product {
                            id: product.id.to_string(),
                            name: product.name.clone(),
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

    async fn read_product(
        &self,
        request: Request<ReadProductRequest>,
    ) -> Result<Response<ReadProductResponse>, Status> {
        println!("{:?}", request);

        let into_request = request.into_inner();
        let uuid = match uuid::Uuid::parse_str(&into_request.id) {
            Ok(v) => v,
            Err(e) => return Err(Status::new(Code::Unknown, format!("{}", e))),
        };

        match ProductUseCase::find_by_id(&self.context, &uuid) {
            Ok(product) => {
                let message = ReadProductResponse {
                    product: Some(Product {
                        id: product.id.to_string(),
                        name: product.name.clone(),
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

        match ProductUseCase::update(&self.context, &uuid, &into_request.name, into_request.price) {
            Ok(product) => {
                let message = UpdateProductResponse {
                    product: Some(Product {
                        id: product.id.to_string(),
                        name: product.name.clone(),
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

        match ProductUseCase::delete(&self.context, &uuid) {
            Ok(_) => {
                let message = DeleteProductResponse {};
                Ok(Response::new(message))
            }
            Err(e) => Err(Status::new(Code::Unknown, format!("{}", e))),
        }
    }
}

impl ProductController {
    pub fn new(context: Context) -> product_service_server::ProductServiceServer<Self> {
        product_service_server::ProductServiceServer::new(ProductController { context })
    }
}
