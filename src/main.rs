use actix_web::http::header;
use actix_web::{web, App, HttpServer, HttpResponse, HttpRequest};
use rand::distributions::Alphanumeric;
use rand::{Rng, rngs::OsRng};
use std::collections::HashMap;
use std::sync::Mutex;

struct AppState {
    url_mapping: Mutex<HashMap<String, String>>,
}

async fn shorten_url(data: web::Data<AppState>, req: HttpRequest, url: web::Json<String>) -> HttpResponse {
    let mut url_mapping = data.url_mapping.lock().unwrap();

    let short_url: String = OsRng
        .sample_iter(&Alphanumeric)
        .take(6)
        .map(char::from)
        .collect();

    // Get the base URL from the request
    let base_url = format!("{}://{}", req.connection_info().scheme(), req.connection_info().host());

    url_mapping.insert(short_url.clone(), url.into_inner());

    // Create the full shortened URL
    let full_shortened_url = format!("{}/{}", base_url, short_url);

    HttpResponse::Ok().json(full_shortened_url)
}

async fn redirect(data: web::Data<AppState>, info: web::Path<String>) -> HttpResponse {
    let url_mapping = data.url_mapping.lock().unwrap();

    if let Some(original_url) = url_mapping.get(&info.into_inner()) {
        return HttpResponse::TemporaryRedirect()
            .append_header((header::LOCATION, original_url.clone()))
            .finish();
    }

    HttpResponse::NotFound().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        url_mapping: Mutex::new(HashMap::new()),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(web::resource("/shorten").route(web::post().to(shorten_url)))
            .service(web::resource("/{info}").route(web::get().to(redirect)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
