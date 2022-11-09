#[macro_use]
extern crate rocket;
mod db;
use db::requests::TaskRequest;
use db::responses::Task;
use db::{create_task, get_task, get_tasks, DBResult};
use rocket::serde::json::Json;
use rocket::State;
use sqlx::{Pool, Sqlite, SqlitePool};

#[post("/tasks", format = "json", data = "<task>")]
async fn create(task: Json<TaskRequest>, pool: &State<Pool<Sqlite>>) -> DBResult<Json<Task>> {
    let id = create_task(pool, &task.name, &task.description).await?;
    let task = get_task(pool, id).await?;
    Ok(Json(task))
}

#[get("/tasks")]
async fn index(pool: &State<Pool<Sqlite>>) -> DBResult<Json<Vec<Task>>> {
    let tasks = get_tasks(pool).await?;
    Ok(Json(tasks))
}

#[get("/tasks/<id>")]
async fn detail(id: i64, pool: &State<Pool<Sqlite>>) -> DBResult<Json<Task>> {
    let task = get_task(pool, id).await?;
    Ok(Json(task))
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let pool = SqlitePool::connect("sqlite://db.sqlite3")
        .await
        .expect("Couldn't connect to sqlite database");
    let _rocket = rocket::build()
        .mount("/", routes![index, create, detail])
        .manage(pool)
        .launch()
        .await?;
    Ok(())
}
