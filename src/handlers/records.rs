use actix_web::{web, Error, HttpResponse, Scope};
use deadpool_postgres::{Client, Pool};
use tokio_pg_mapper::FromTokioPostgresRow;

use crate::models::Record;

async fn get_all(pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let client: Client = pool.get().await.unwrap();

    let rows = client
        .query("SELECT * FROM public.records", &[])
        .await
        .unwrap();

    let tasks: Vec<Record> = rows
        .iter()
        .map(|row| Record::from_row_ref(row).unwrap())
        .collect();

    Ok(HttpResponse::Ok().json(tasks))
}

async fn get_by_id(id: web::Path<String>, pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let parsed_id = id.trim().parse::<i32>().unwrap();
    let client: Client = pool.get().await.unwrap();

    let rows = client
        .query("SELECT * FROM public.records", &[])
        .await
        .unwrap();

    let task: Option<Record> = rows
        .iter()
        .map(|row| Record::from_row_ref(row).unwrap())
        .find(|x| x.id == parsed_id);

    if task.is_none() {
        Ok(HttpResponse::NotFound().body("Not Found"))
    } else {
        Ok(HttpResponse::Ok().json(task))
    }
}

async fn create(record: web::Json<Record>, pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let client: Client = pool.get().await.unwrap();

    let rows = client
        .query(
            "INSERT INTO public.records(label)
             VALUES ($1)
             RETURNING *;",
            &[&record.label],
        )
        .await
        .unwrap();

    let new_item: Record = rows
        .iter()
        .map(|row| Record::from_row_ref(row).unwrap())
        .collect::<Vec<Record>>()
        .pop()
        .unwrap();

    Ok(HttpResponse::Ok().json(new_item))
}

pub fn get_servises() -> Scope {
    web::scope("/records")
        .service(web::resource("").route(web::get().to(get_all)))
        .service(web::resource("/{id}").route(web::get().to(get_by_id)))
        .service(web::resource("").route(web::post().to(create)))
}
