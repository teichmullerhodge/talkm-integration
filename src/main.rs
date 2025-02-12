#![allow(non_snake_case)]


use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

use serde::Serialize;
use serde::Deserialize;


mod models;
mod talkm;



#[derive(Deserialize)]
struct PrimeQuery {
    number: u64,
}

// Efficient primality test
fn is_prime_number(n: u64) -> bool {

    if n == 2 || n == 3 {
        return true;
    }
 
    
    if n % 2 == 0 || n % 3 == 0 || n < 2 {
        return false;
    }

    let limit = f64::sqrt(n as f64) as u64;
    let mut i = 5;
    while i <= limit {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }

    return true;
}

#[get("/is-prime")]
async fn is_prime(query: web::Query<PrimeQuery>) -> impl Responder {
    let number = query.number;
    let result = is_prime_number(number);

    HttpResponse::Ok().json(serde_json::json!({
        "number": number,
        "is_prime": result
    }))
}



#[get("/status")]
async fn hello() -> impl Responder {
    println!("Status acessado!");
    HttpResponse::Ok().body("Hello world!")
}


#[derive(Deserialize)]
struct SendMessageRequest {
    message: String,
}

#[derive(Serialize)]
struct MessageResponse {
    message: String,
}


#[post("/send-message")]
async fn send_message(
    body: web::Json<SendMessageRequest>,
    req: actix_web::HttpRequest,
) -> impl Responder {

    let connectionKey = match req.headers().get("Authorization") {
        Some(value) => match value.to_str() {
            Ok(v) => v.replace("Bearer ", ""), 
            Err(_) => return HttpResponse::BadRequest().json(MessageResponse { message: "Invalid Authorization header".to_string() }),
        },
        None => return HttpResponse::BadRequest().json(MessageResponse { message: "Missing Authorization header".to_string() }),
    };

    let sampleMessage = talkm::communication::Message {
        number: "554396706748".to_string(),
        body: body.message.clone(),
        queueId: 3,
        userId: 3,
    };

    match talkm::communication::send_message(&sampleMessage, connectionKey).await {
        Ok(true) => HttpResponse::Ok().json(MessageResponse { message: "Mensagem enviada!".to_string() }),
        Ok(false) => HttpResponse::InternalServerError().json(MessageResponse { message: "Erro ao enviar mensagem".to_string() }),
        Err(err) => HttpResponse::InternalServerError().json(MessageResponse { message: format!("Erro: {}", err) }),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {


    HttpServer::new(|| {
        App::new()
        .service(hello)
        .service(send_message)
        .service(is_prime)
    })
    .bind(("0.0.0.0", 8080))?  // Garante que escutará em todas as interfaces    
    .run()
    .await

}