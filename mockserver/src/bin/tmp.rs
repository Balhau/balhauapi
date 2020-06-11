
use rweb::*;
use serde::Serialize;

#[get("/")]
fn index() -> String {
    String::from("content type will be 'text/plain' as you return String")
}

#[derive(Debug, Serialize, Schema)]
struct Product {
    id: String,
    price: usize,
}

#[get("/products")]
fn products() -> Json<Vec<Product>> {
    unimplemented!("content type will be 'application/json', and type of openapi schema will be array")
}

#[get("/product/{id}")]
fn product(id: String) -> Json<Product> {
    // See Component section below if you want to give a name to type.
    unimplemented!("content type will be 'application/json', and type of openapi schema will be object")
}

/// By default, doc comments on the function will become description of the operation.
#[get("/sum/{a}/{b}")]
#[openapi(description = "But what if implementation details is written on it?")]
fn sum(a: usize, b: usize) -> String {
    (a + b).to_string()
}

#[tokio::main]
async fn main() {
    let (_spec, filter) = openapi::spec()
    .build(||{
           index()
           .or(products())
           .or(product())
           .or(sum())
    });

    //serve(filter);
    // Use the code below to run server.
    //
     serve(filter).run(([127, 0, 0, 1], 3030)).await;
}