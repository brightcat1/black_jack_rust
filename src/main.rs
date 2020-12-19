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
    suit: String,
    num: u32,
}

struct GameData {
    player: u32,
    dealer: u32,
    stand: u32,
}

#[derive(Template)]
#[template(path = "index.html")]
struct BlackJackTemplate{
    game_data: GameData,
    card: DrawCard,
    result: String,
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
    conn.execute("DROP TABLE IF EXISTS drawn_card", params![]).expect("Failed to drop a table `drawn_card`.");
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
    conn.execute(
        "CREATE TABLE IF NOT EXISTS drawn_card (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            suit TEXT NOT NULL,
            number INTEGER NOT NULL
        )",
        params![],
    ).expect("Failed to create a table `drawn_card`.");
    conn.execute("INSERT INTO points (player, dealer) VALUES (0, 0)", params![])?;
    conn.execute("INSERT INTO drawn_card (suit, number) VALUES ('None', 0)", params![])?;
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
    let mut drawn_card = conn.prepare("SELECT * FROM deck ORDER BY RANDOM() LIMIT 1;")?;
    let mut points = conn.prepare("SELECT player FROM points")?;
    let points_table_data = points.query_map(params![], |row| {
        let player = row.get(0)?;
        Ok(player)
    })?;
    let sum = match points_table_data.last(){
        None => 0,
        Some(x) => x.unwrap(),
    };
    let drawn_card_table_data = drawn_card.query_map(params![], |row| {
        let suit = row.get(0)?;
        let num = row.get(1)?;
        Ok(DrawCard{suit, num})
    })?;
    let card = match drawn_card_table_data.last(){
        None => DrawCard{suit: "None".to_string(), num: 0},
        Some(x) => x.unwrap(),
    };
    conn.execute("INSERT INTO drawn_card (suit, number) VALUES (?1, $2)", params![card.suit, card.num])?;
    conn.execute("UPDATE points SET player = (?) WHERE id = 1", params![sum + card.num])?;
    conn.execute("DELETE FROM todo WHERE id=?", params![])?;
    Ok(HttpResponse::SeeOther()
        .header(header::LOCATION, "/")
        .finish())
}

#[get("/")]
async fn index(db: web::Data<Pool<SqliteConnectionManager>>) -> Result<HttpResponse, MyError> {
    let conn = db.get()?;
    let mut draw = conn.prepare("SELECT suit, number,  FROM drawn_card")?;
    let mut points = conn.prepare("SELECT player, dealer, stand_flag FROM points")?;
    let game_data_row = points.query_map(params![], |row| {
        let player = row.get(0)?;
        let dealer = row.get(1)?;
        let stand = row.get(2)?;
        Ok(GameData{player, dealer, stand})
    })?;
    let drawn_card_table_data = draw.query_map(params![], |row| {
        let suit = row.get(0)?;
        let num = row.get(1)?;
        Ok(DrawCard{suit, num})
    })?;

    let game_data = match game_data_row.last(){
        None => GameData{player: 0, dealer: 0, stand: 0},
        Some(x) => x.unwrap(),
    };
    let card = match drawn_card_table_data.last(){
        None => DrawCard{suit: "None".to_string(), num: 0},
        Some(x) => x.unwrap(),
    };
    let result = if game_data.player == 21{
        "BlackJack! You Win!".to_string()
    } else if game_data.stand == 1 && game_data.player > game_data.dealer{
        "You Win!".to_string()
    } else if game_data.stand == 1 && game_data.player < game_data.dealer{
        "You Lose!".to_string()
    } else if game_data.stand == 1 && game_data.player == game_data.dealer{
        "Draw!".to_string()
    } else {
        "None".to_string()
    };
    let html = BlackJackTemplate{ game_data, card, result };
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
    conn.execute("DROP TABLE IF EXISTS drawn_card", params![]).expect("Failed to drop a table `drawn_card`.");
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
    conn.execute(
        "CREATE TABLE IF NOT EXISTS drawn_card (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            suit TEXT NOT NULL,
            number INTEGER NOT NULL,
            stand_flag INTEGER NOT NULL,
        )",
        params![],
    ).expect("Failed to create a table `drawn_card`.");
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
