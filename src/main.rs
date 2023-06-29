

use actix_web::{get, App, HttpServer, web::Path, Responder};
use rhai::Engine;



#[actix_web::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(|| {
        App::new()
            .service(add)
            .service(sub)
            .service(mul)
    })
    .bind("127.0.0.1:8080")
    .unwrap()
    .run()
    .await
}

#[get("/add/{a}/{b}")]
async fn add(path:Path<(i64,i64)>) -> impl Responder {
    let (a,b) = path.into_inner();
    let mut engine = Engine::new();
    engine.register_fn("a", move || a);
    engine.register_fn("b", move || b);
    let result = engine.eval_file::<i64>("src/add.rhai".into()).unwrap();
    format!("{result}")
}

#[get("/sub/{a}/{b}")]
async fn sub(path:Path<(i64,i64)>) -> impl Responder {
    let (a,b) = path.into_inner();

    let mut engine = Engine::new();
    engine.register_fn("a", move || a);
    engine.register_fn("b", move || b);
    let result = engine.eval_file::<i64>("src/sub.rhai".into()).unwrap();
    format!("{result}")
}
#[get("/mul/{a}/{b}")]
async fn mul(path:Path<(i64,i64)>) -> impl Responder {
    let (a,b) = path.into_inner();

    let mut engine = Engine::new();
    engine.register_fn("a", move || a);
    engine.register_fn("b", move || b);
    let result = engine.eval_file::<i64>("src/mul.rhai".into()).unwrap();
    format!("{result}")
}
