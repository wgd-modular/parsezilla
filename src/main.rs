use actix_web::dev::Service;
use actix_web::{App, HttpResponse, HttpServer, Responder, post, web};
use clap::{Arg, Command};
use colored::Colorize;
use futures_util::FutureExt;
use postal::{Context, InitOptions, ParseAddressOptions};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

struct ParseZilla {
    ctx: Context,
}

impl ParseZilla {
    fn new() -> Self {
        let mut ctx = Context::new();
        ctx.init(InitOptions {
            expand_address: false,
            parse_address: true,
        })
        .expect("Failed to init parsezilla context");

        ParseZilla { ctx }
    }

    fn parse(&mut self, address: &str) -> Vec<(String, String)> {
        let mut opts = ParseAddressOptions::new();
        let comps = self
            .ctx
            .parse_address(address, &mut opts)
            .expect("Failed to parse address");
        comps
            .map(|c| (c.label.to_string(), c.value.to_string()))
            .collect()
    }
}

#[derive(Deserialize)]
struct ParseRequest {
    address: String,
}

#[derive(Serialize)]
struct ComponentResponse {
    component: String,
    value: String,
}

#[post("/parse")]
async fn parse_address_handler(
    data: web::Data<AppState>,
    req: web::Json<ParseRequest>,
) -> impl Responder {
    let mut pz = data.parsezilla.lock().unwrap();
    let result = pz.parse(&req.address);
    let response: Vec<ComponentResponse> = result
        .into_iter()
        .map(|(component, value)| ComponentResponse {
            component,
            value: capitalize_words(value),
        })
        .collect();

    HttpResponse::Ok().json(response)
}

fn capitalize_words(input: String) -> String {
    input
        .split_whitespace()
        .map(|word| {
            let mut c = word.chars();
            match c.next() {
                Some(first) => first.to_uppercase().chain(c).collect(),
                None => String::new(),
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}

struct AppState {
    parsezilla: Mutex<ParseZilla>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let api_key = std::env::var("PARSEZILLA_API_KEY")
        .expect("PARSEZILLA_API_KEY environment variable not set");

    let matches = Command::new("Parsezilla API")
        .version("1.0")
        .about("Address parsing API using Parsezilla")
        .arg(
            Arg::new("port")
                .short('p')
                .long("port")
                .default_value("8080")
                .value_parser(clap::value_parser!(u16))
                .help("Sets the port for the server"),
        )
        .arg(
            Arg::new("bind")
                .short('b')
                .long("bind")
                .default_value("127.0.0.1")
                .help("Sets the bind address for the server"),
        )
        .get_matches();

    let port = *matches.get_one::<u16>("port").unwrap_or(&8080);
    let bind_address: String = matches
        .get_one::<String>("bind")
        .cloned()
        .unwrap_or_else(|| "127.0.0.1".to_string());

    let pz = ParseZilla::new();
    let shared_state = web::Data::new(AppState {
        parsezilla: Mutex::new(pz),
    });

    println!(
        "{} {}",
        "🦖 Parsezilla is roaring at".green(),
        format!("http://{}:{}", bind_address, port).red()
    );

    HttpServer::new(move || {
        let api_key_clone = api_key.clone();
        App::new()
            .wrap_fn(move |req, srv| {
                let authorized = req
                    .headers()
                    .get("x-api-key")
                    .and_then(|h| h.to_str().ok())
                    .map(|key| key == api_key_clone)
                    .unwrap_or(false);
                if authorized {
                    srv.call(req).boxed_local()
                } else {
                    let response = HttpResponse::Unauthorized().body("Invalid API key");
                    Box::pin(async { Ok(req.into_response(response.map_into_boxed_body())) })
                }
            })
            .app_data(shared_state.clone())
            .service(parse_address_handler)
    })
    .bind((bind_address.as_str(), port))?
    .run()
    .await
}
