use actix_web::{web, Error, HttpResponse, Responder};
use chrono;
use diesel::dsl::{delete, insert_into};
use diesel::prelude::*;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use serde::{Deserialize, Serialize};
use std::vec::Vec;

use models::todos::Todo;
use models::users::{NewUser, User};
use mysql::MysqlPool;
use schema::schema::{todos, users};

#[derive(Debug, Serialize, Deserialize)]
pub struct InputUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

pub async fn get_todos() -> impl Responder {
    format!("hello from get todos")
}

// Handler for GET /users
pub async fn get_users(db: web::Data<MysqlPool>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || get_all_users(db))
        .await
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

// Handler for GET /users/{id}
pub async fn get_user_by_id(
    db: web::Data<MysqlPool>,
    user_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || db_get_user_by_id(db, user_id.into_inner()))
            .await
            .map(|user| HttpResponse::Ok().json(user))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}

// Handler for POST /users
pub async fn add_user(
    db: web::Data<MysqlPool>,
    item: web::Json<InputUser>,
) -> Result<HttpResponse, Error> {
    Ok(web::block(move || add_single_user(db, item))
        .await
        .map(|user| HttpResponse::Created().json(user))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

// Handler for DELETE /users/{id}
pub async fn delete_user(
    db: web::Data<MysqlPool>,
    user_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || delete_single_user(db, user_id.into_inner()))
            .await
            .map(|user| HttpResponse::Ok().json(user))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}

fn get_all_users(pool: web::Data<MysqlPool>) -> Result<Vec<User>, diesel::result::Error> {
    let conn = pool.get().unwrap();
    let items = users::table.load::<User>(&conn)?;
    Ok(items)
}

fn db_get_user_by_id(
    pool: web::Data<MysqlPool>,
    user_id: i32,
) -> Result<User, diesel::result::Error> {
    let conn = pool.get().unwrap();
    users::table.find(user_id).get_result::<User>(&conn)
}

fn add_single_user(
    db: web::Data<MysqlPool>,
    item: web::Json<InputUser>,
) -> Result<User, diesel::result::Error> {
    let conn = db.get().unwrap();
    let new_user = NewUser {
        first_name: &item.first_name,
        last_name: &item.last_name,
        email: &item.email,
        created_at: chrono::Local::now().naive_local(),
    };
    insert_into(users::table).values(&new_user).execute(&conn)?;

    users::table.order(users::id.desc()).first(&conn)
}

fn delete_single_user(
    db: web::Data<MysqlPool>,
    user_id: i32,
) -> Result<usize, diesel::result::Error> {
    let conn = db.get().unwrap();
    let count = delete(users::table.find(user_id)).execute(&conn)?;
    Ok(count)
}
