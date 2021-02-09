use actix_web::{dev::ServiceRequest, web, App, Error, HttpServer};

use mysql;
use handler;


#[actix_rt::main]
async fn main() -> Result<(), actix_web::Error> {
    

    let pool = mysql::establish_connection();

    // let results = todos
    //     // .filter(published.eq(true))
    //     .limit(5)
    //     .load::<Todo>(&connection)
    //     .expect("Error loading todos");

    // println!("Displaying {} posts", results.len());
    // for todo in results {
    //     println!("{}", todo.id);
    //     println!("-----------\n");
    //     println!("{}", todo.text);
    // }

    HttpServer::new(move || {
        App::new()
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
