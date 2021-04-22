use std::time::{Instant};
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use serde_json::json;
use sudoku::Sudoku;

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    // format!("Hello {}!", &name)
    let data = json!({
        "hello": format!("{}", name)
    });
    HttpResponse::Ok().json(data)
}

async fn puzzle(_req: HttpRequest) -> impl Responder {
    let start = Instant::now();
    let generated = Sudoku::generate_unique();
    let sudoku_line = generated.to_str_line();
    let time = start.elapsed().as_nanos() as u64;
    let puzzle = format!("{}", sudoku_line);
    let blanks = puzzle.matches(".").count();

    let sudoku = Sudoku::from_str_line(&puzzle).unwrap();
    let fin = Instant::now();
    let mut solved = 0;
    let mut line = "".to_string();
    
    if let Some(solution) = sudoku.solve_unique() {
        solved = fin.elapsed().as_nanos() as u64;
        line = format!("{}", solution);
    }

    let data = json!({
        "blanks": blanks,
        "puzzle": puzzle,
        "solution": line,
        "solved": solved,
        "time": time,
        "units": "ns"
    });
    HttpResponse::Ok().json(data)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let addr = "localhost";
    let port = 8080;

    println!("http://{}:{}", addr, port);
    println!("http://{}:{}/api/puzzle", addr, port);

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
            .route("/api/puzzle", web::get().to(puzzle))
            .route("/api/{name}", web::get().to(greet))
    })
    .bind((addr, port))?
    .run()
    .await
}
