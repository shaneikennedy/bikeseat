use actix_files;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use md::Renderer;
use std::fs;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    bootstrap_content();
    HttpServer::new(|| {
        App::new()
            .service(home)
            .service(blog_post)
            .service(actix_files::Files::new("static", "./static").show_files_listing())
    })
    .bind(("127.0.0.1", 7878))?
    .run()
    .await
}

#[get("/{post_slug}")]
async fn blog_post(post_slug: web::Path<String>) -> impl Responder {
    let contents = match fs::read_to_string(format!("out/{}.html", post_slug)) {
        Ok(file) => file,
        Err(_) => fs::read_to_string("static/404.html").unwrap(),
    };

    HttpResponse::Ok().body(contents)
}

#[get("/")]
async fn home() -> impl Responder {
    let contents = match fs::read_to_string("out/index.html") {
        Ok(file) => file,
        Err(_) => fs::read_to_string("static/404.html").unwrap(),
    };

    HttpResponse::Ok().body(contents)
}

fn bootstrap_content() {
    let content_dir: fs::ReadDir = fs::read_dir("content").expect("Unable to read dir 'content'");
    let mut post_titles = Vec::new();
    for item in content_dir {
        let file_name = item.as_ref().unwrap().file_name().into_string().unwrap();
        println!("content/{}", file_name);
        if file_name.split(".").last().unwrap() == "md" {
            Renderer::render_post(file_name.clone());
            post_titles.push(file_name);
        }
    }
    Renderer::render_home(post_titles);
}
