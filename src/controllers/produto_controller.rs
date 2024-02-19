use rocket::http::Status;
use rocket::response::status::NotFound;
use rocket::request::FromParam;
use rocket::serde::json::Json;
use rocket::State;
use rocket_okapi::{openapi, openapi_get_routes};

use crate::adapter::api::error_handling::ErrorResponse;
use crate::adapter::api::request_guards::authentication_guard::AuthenticatedUser;
use crate::core::application::use_cases::gerenciamento_de_produtos_use_case::{ProdutoUseCase, CreateProdutoInput, UpdateProdutoInput};
use crate::entities::produto::Produto;
use crate::adapter::api::request_guards::admin_guard::AdminUser;

#[openapi(tag = "Produtos")]
#[get("/")]
async fn get_produto(
    produto_use_case: &State<ProdutoUseCase>,
    _logged_user_info: AuthenticatedUser,
) -> Result<Json<Vec<Produto>>, Status> {
    let produtos = produto_use_case.get_produtos().await?;
    Ok(Json(produtos))
}

#[openapi(tag = "Produtos")]
#[get("/<id>")]
async fn get_produto_by_id(
    produto_use_case: &State<ProdutoUseCase>,
    id: usize,
    _logged_user_info: AuthenticatedUser,
) -> Result<Json<Produto>, Status> {
    let produto = produto_use_case.get_produto_by_id(id).await?;
    Ok(Json(produto))
}

#[openapi(tag = "Produtos")]
#[post("/", data = "<produto_input>")]
async fn create_produto(
    produto_use_case: &State<ProdutoUseCase>,
    produto_input: Json<CreateProdutoInput>,
    _logged_user_info: AuthenticatedUser,
) -> Result<Json<Produto>, Status> {
    let produto_input = produto_input.into_inner();
    let produto = produto_use_case.create_produto(produto_input).await?;
    Ok(Json(produto))
}

#[openapi(tag = "Produtos")]
#[put("/<id>", data = "<produto_input>")]
async fn update_produto(
    produto_use_case: &State<ProdutoUseCase>,
    produto_input: Json<CreateProdutoInput>,
    id: usize,
    _logged_user_info: AuthenticatedUser,
) -> Result<Json<Produto>, Status> {
    let produto_input = produto_input.into_inner();
    let produto = produto_use_case.update_produto(id, produto_input).await?;
    Ok(Json(produto))
}

#[openapi(tag = "Produtos")]
#[delete("/<id>")]
async fn delete_produto(
    produto_use_case: &State<ProdutoUseCase>,
    id: usize,
    _logged_user_info: AdminUser,
) -> Result<Json<String>, Status> {
    produto_use_case.delete_produto(id).await?;
    Ok(Json("success".to_string()))
}

pub fn routes() -> Vec<rocket::Route> {
    openapi_get_routes![get_produto, get_produto_by_id, create_produto, update_produto, delete_produto]
}

#[catch(404)]
fn produto_not_found() -> Json<ErrorResponse> {
    let error = ErrorResponse {
        msg: "Produto não encontrado!".to_string(),
        status: 404,
    };
    Json(error)
}

pub fn catchers() -> Vec<rocket::Catcher> {
    catchers![produto_not_found]
}