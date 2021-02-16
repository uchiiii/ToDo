use actix_web::{dev::ServiceRequest, web, App, Error, HttpServer};
use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
use actix_web_httpauth::extractors::AuthenticationError;
use actix_web_httpauth::middleware::HttpAuthentication;

use handler;
use mysql;
use auth;

async fn validator(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, Error> {
    let config = req
        .app_data::<web::Data<Config>>()
        .map(|data| data.get_ref().clone())
        .unwrap_or_else(Default::default);

    match auth::validate_token(credentials.token()).await {
        Ok(res) => {
            if res == true {
                Ok(req)
            } else {
                Err(AuthenticationError::from(config).into())
            }
        }
        Err(_) => Err(AuthenticationError::from(config).into()),
    }
}

#[actix_rt::main]
async fn main() -> Result<(), actix_web::Error> {
    let pool = mysql::establish_connection();
    let authenticate = HttpAuthentication::bearer(validator);

    HttpServer::new(move || {
        App::new()
            .wrap(authenticate.clone())
            .data(pool.clone())
            .route("/todos", web::get().to(handler::get_todos))
            .route("/users", web::get().to(handler::get_users))
            .route("/users/{id}", web::get().to(handler::get_user_by_id))
            .route("/users", web::post().to(handler::add_user))
            .route("/users/{id}", web::delete().to(handler::delete_user))
    })
    // .data(pool.clone())
    .bind("0.0.0.0:3000")?
    .run()
    .await?;
    Ok(())
}
