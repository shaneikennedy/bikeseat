use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::str::from_utf8;

use md::Renderer;
use md::ThreadPool;
use regex::Regex;

fn main() {
    bootstrap_content();
    start_webserver();
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

fn start_webserver() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let home = b"GET / HTTP/1.1\r\n";
    let re = Regex::new(r"GET /(?P<route>[a-zA-Z0-9-]+) HTTP/1.1\r\n").unwrap();
    let req = from_utf8(&buffer).unwrap();
    let (status_line, filename) = if buffer.starts_with(home) {
        ("HTTP/1.1 200 OK", "index")
    } else if re.is_match(req) {
        let route = re.captures(req).unwrap().name("route").unwrap().as_str();
        ("HTTP/1.1 200 OK", route)
    } else {
        ("HTTP/1.1 404 NOT FOUND", "static/404.html")
    };

    let contents = match fs::read_to_string(format!("out/{}.html", filename)) {
        Ok(file) => file,
        Err(_) => fs::read_to_string("static/404.html").unwrap(),
    };

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
