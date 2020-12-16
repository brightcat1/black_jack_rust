use actix_web::{get, http::header, post, web, App, HttpResponse, HttpServer, ResponseError};
use askama::Template;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;
use thiserror::Error;
use strum::IntoEnumIterator;
use strum::EnumIter;
use strum_macros::ToString;

#[derive(EnumIter, ToString)]
enum Suits {
    Spade,
    Heart,
    Diamond,
    Club,
}

struct DrawCard {
    draw_card_suite: String,
    draw_card_num: u32,
}

#[derive(Template)]
#[template(path = "index.html")]
struct PlayerPointTemplate{
    sum: u32,
    card: DrawCard,
}


#[derive(Error, Debug)]
enum MyError {
    #[error("Failed to render HTML")]
    AskamaError(#[from] askama::Error),

    #[error("Failed to get connection")]
    ConnectionPoolError(#[from] r2d2::Error),

    #[error("Failed SQL execution")]
    SQLiteError(#[from] rusqlite::Error),
}

impl ResponseError for MyError {}

#[post("/start")]
async fn start_game(
    db: web::Data<r2d2::Pool<SqliteConnectionManager>>,
) -> Result<HttpResponse, MyError>{
    let conn = db.get()?;
    conn.execute("DROP TABLE IF EXISTS deck", params![]).expect("Failed to drop a table `deck`.");
    conn.execute("DROP TABLE IF EXISTS points", params![]).expect("Failed to drop a table `points`.");
    conn.execute(
        "CREATE TABLE IF NOT EXISTS deck (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            suit TEXT NOT NULL,
            number INTEGER NOT NULL
        )",
        params![],
    ).expect("Failed to create a table `deck`.");
    conn.execute(
        "CREATE TABLE IF NOT EXISTS points (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            player INTEGER NOT NULL,
            dealer INTEGER NOT NULL
        )",
        params![],
    ).expect("Failed to create a table `points`.");
    conn.execute("INSERT INTO points (player, dealer) VALUES (0, 0)", params![])?;
    for suit in Suits::iter(){
        for num in 1..=13 {
            conn.execute("INSERT INTO deck (suit, number) VALUES (?1, $2)", params![suit.to_string(), num])?;
        }
    }
    Ok(HttpResponse::SeeOther()
    .header(header::LOCATION, "/")
    .finish())
}

#[post("/draw")]
async fn draw_card(
    db: web::Data<r2d2::Pool<SqliteConnectionManager>>,
) -> Result<HttpResponse, MyError> {
    let conn = db.get()?;
    conn.execute("DELETE FROM todo WHERE id=?", params![])?;
    Ok(HttpResponse::SeeOther()
        .header(header::LOCATION, "/")
        .finish())
}

#[get("/")]
async fn index(db: web::Data<Pool<SqliteConnectionManager>>) -> Result<HttpResponse, MyError> {
    let conn = db.get()?;
    let draw = conn.prepare("SELECT suit, number FROM deck")?;
    let mut points = conn.prepare("SELECT player FROM points")?;
    let a = points.query_map(params![], |row| {
        let player: u32 = row.get(0)?;
        Ok(player)
    })?;

    let sum = match a.last(){
        None => 0,
        Some(x) => x.unwrap(),
    };
    let card = DrawCard{draw_card_suite: "None".to_string(), draw_card_num: 0, };
    let html = PlayerPointTemplate{ sum, card };
    let response_body = html.render()?;
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(response_body))
}


#[actix_rt::main]
async fn main() -> Result<(), actix_web::Error>{
    let manager = SqliteConnectionManager::file("blackjack.db");
    let pool = Pool::new(manager).expect("Failed to initialize the connection pool.");
    let conn = pool
        .get()
        .expect("Failed to get the connection from the pool.");
    conn.execute("DROP TABLE IF EXISTS deck", params![]).expect("Failed to drop a table `deck`.");
    conn.execute("DROP TABLE IF EXISTS points", params![]).expect("Failed to drop a table `points`.");
    conn.execute(
        "CREATE TABLE IF NOT EXISTS deck (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            suit TEXT NOT NULL,
            number INTEGER NOT NULL
        )",
        params![],
    ).expect("Failed to create a table `deck`.");
    conn.execute(
        "CREATE TABLE IF NOT EXISTS points (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            player INTEGER NOT NULL,
            dealer INTEGER NOT NULL
        )",
        params![],
    ).expect("Failed to create a table `points`.");
    HttpServer::new(move || {
        App::new()
        .service(index)
        .service(start_game)
        .service(draw_card)
        .data(pool.clone())
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await?;
    Ok(())
}
