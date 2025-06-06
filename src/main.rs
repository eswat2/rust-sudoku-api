use actix_cors::Cors;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_files::NamedFile;

use base64::{Engine as _, engine::general_purpose};
use serde_json::json;
use std::env;
use std::path::PathBuf;
use std::time::Instant;
use sudoku::Sudoku;

const PLATFORM: &str = "platform";

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    // format!("Hello {}!", &name)
    let data = json!({ "hello": format!("{}", name) });
    HttpResponse::Ok().json(data)
}

async fn groot(_req: HttpRequest) -> actix_web::Result<NamedFile> {
    let name: &str = "index.html";
    let path: PathBuf = name.parse().unwrap();
    Ok(NamedFile::open(path)?)
}

async fn puzzle(_req: HttpRequest) -> impl Responder {
    let start = Instant::now();
    // NOTE:  api has changed from generate_unique to generate...
    //        (see docs for other api options)
    let generated = Sudoku::generate();
    let sudoku_line = generated.to_str_line();
    let time = start.elapsed().as_nanos() as u64;
    let puzzle = format!("{}", sudoku_line);
    let blanks = puzzle.matches(".").count();

    let sudoku = Sudoku::from_str_line(&puzzle).unwrap();
    let fin = Instant::now();
    let mut solved = 0;
    let mut line = "".to_string();

    // NOTE:  api changed from solve_unique to solution
    if let Some(solution) = sudoku.solution() {
        solved = fin.elapsed().as_nanos() as u64;
        line = format!("{}", solution);
    }

    let data = json!({
        "metrics" : {
            "counts": {
                "blanks": blanks,
                "clues": 81 - blanks
            },
            "nanos": {
                "generate": time,
                "solve": solved
            }
        },
        "puzzle": puzzle,
        "ref": general_purpose::STANDARD.encode(line),
        "tag": PLATFORM
    });
    // NOTE:  for debugging, print the first 3 rows of the puzzle...
    let slice: &str = &puzzle[..27];
    println!("-- {},{}:  {:?}", blanks, 81 - blanks, slice);
    HttpResponse::Ok().json(data)
}

fn get_server_port() -> u16 {
    env::var("PORT")
        .ok()
        .and_then(|port| port.parse().ok())
        .unwrap_or_else(|| 8080)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let addr = String::from("0.0.0.0");
    let port = get_server_port();

    println!("http://{}:{}", addr, port);
    println!("http://{}:{}/api/puzzle", addr, port);

    HttpServer::new(|| {
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .route("/", web::get().to(groot))
            .route("/{name}", web::get().to(greet))
            .route("/api/puzzle", web::get().to(puzzle))
            .route("/api/{name}", web::get().to(greet))
    })
    .bind((addr, port))?
    .run()
    .await
}
