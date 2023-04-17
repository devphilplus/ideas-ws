use log::{
    info,
    debug,
    error
};

use actix_web::{
    HttpResponse, 
    Responder,
    web
};

use serde::{
    Serialize,
    Deserialize
};
use serde_json::json;

use crate::endpoints::{
    ApiResponse
};
use crate::classes::user::CurrentUser;



#[derive(Debug, Serialize, Deserialize)]
struct EmployeeAddRequest {
    pub tenant_id: uuid::Uuid,
    pub people_id: uuid::Uuid,
    pub given_name: String,
    pub middle_name: String,
    pub family_name: String,
    pub prefix: String,
    pub suffix: String,
    pub gender_id: i32,
    pub ethnicity_id: i32,
    pub marital_state_id: i32
}



pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::resource("add")
                .route(web::get().to(employee_add_get))
                .route(web::post().to(employee_add_post))
        )
    ;
}


async fn employee_add_get() -> impl Responder {
    info!("employee_add_get()");

    return HttpResponse::Ok().body("Service is up. version: 1.0.0.0.dev");
}


async fn employee_add_post(
    user: CurrentUser,
    people: web::Data<people::people::People>,
    employees: web::Data<hr::employees::Employees>,
    params: web::Json<EmployeeAddRequest>
) -> impl Responder {
    info!("employee_add_post()");
    debug!("params: {:?}", params);

    match people.add(
        &user.tenant_id(),
        &params.people_id,
        &params.given_name,
        &params.middle_name,
        &params.family_name,
        &params.prefix,
        &params.suffix,
        &params.gender_id,
        &params.ethnicity_id,
        &params.marital_state_id
    ).await {
        Err(e) => {
            error!("unable to ")
        }
        Ok(_) => {
            info!("employee_add_post"); 
        }
    }

    return HttpResponse::Ok()
        .json(ApiResponse {
            success: false,
            message: String::from("Service is up. version: 1.0.0.0.dev"),
            data: None
        });
}