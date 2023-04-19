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
    pub gender_id: i16,
    pub ethnicity_id: i16,
    pub marital_state_id: i16
}


#[derive(Debug, Serialize, Deserialize)]
struct TenantEmployeesRequest {
    pub tenant_id: uuid::Uuid
}



pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::resource("add")
                .route(web::get().to(employee_add_get))
                .route(web::post().to(employee_add_post))
        )
        .service(
            web::resource("fetch")
                .route(web::get().to(employee_fetch_get))
                .route(web::post().to(employee_fetch_post))
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

    if let Ok(p) = people.by_id(&params.people_id).await {
        if let Ok(_) = employees.add(
            &user.tenant_id(), 
            &params.people_id,
            &params.people_id
        ).await {
            info!("added employee record");
        } else {
            return HttpResponse::Ok()
                .json(ApiResponse::new(
                    false,
                    "unable to add employee record",
                    None
                ));
        }
    } else {
        // add people record
        if let Ok(_) =  people.add(
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
            if let Ok(_) = employees.add(
                &user.tenant_id(), 
                &params.people_id,
                &params.people_id
            ).await {
                info!("added employee record");
            } else {
                return HttpResponse::Ok()
                    .json(ApiResponse::new(
                        false,
                        "unable to add employee record",
                        None
                    ));
            }
        }
    }

    return HttpResponse::Ok()
        .json(ApiResponse::new(
            true,
            "added employee record",
            None
        ));
}


async fn employee_fetch_get() -> impl Responder {
    info!("employee_fetch_get()");

    return HttpResponse::Ok().body("Service is up. version: 1.0.0.0.dev");
}


async fn employee_fetch_post(
    user: CurrentUser,
    employees: web::Data<hr::employees::Employees>,
    params: web::Json<TenantEmployeesRequest>
) -> impl Responder {
    info!("employee_fetch_post()");
    debug!("params: {:?}", params);

    match employees.fetch(&params.tenant_id).await {
        Err(e) => {
            error!("unable to fetch employee records: {:?}", e);
            return HttpResponse::InternalServerError()
                .json(ApiResponse::new(
                    false,
                    "unable to fetch employee records",
                    None
                ));
        }
        Ok(employees) => {
            return HttpResponse::Ok()
                .json(ApiResponse::new(
                    true,
                    "successfully retrieve employee records",
                    Some(json!({
                        "employees": employees
                    }))
                ))
        }
    }
}