use std::fs;

use md::Renderer;

fn main() {
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
