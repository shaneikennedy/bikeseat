use std::fs;

use md::Parser;

fn main() {
    let content_dir: fs::ReadDir = fs::read_dir("content").expect("Unable to read dir 'content'");
    let mut post_titles = Vec::new();
    for item in content_dir {
        let file_name = item.as_ref().unwrap().file_name().into_string().unwrap();
        println!("content/{}", file_name);
        if file_name.split(".").last().unwrap() == "md" {
            render_post(&file_name);
            post_titles.push(file_name);
        }
    }

    let mut html_str = String::new();
    for title in post_titles {
        html_str += &format!("<li><a href='{}.html'>{}</a></li>\n", title, title);
    }
    let template =
        fs::read_to_string("templates/index.template.html").expect("could not find template file");
    let content = template.as_str().replace("{{ html_template }}", &html_str);
    let out_file = "out/index.html";
    fs::write(out_file, content).expect("unable to write to index.html");
}

fn render_post(file_name: &String) {
    let markdown: String =
        fs::read_to_string(format!("content/{}", file_name.clone())).expect("could not read file.");
    let html_str = Parser::parse_md(markdown.lines().map(|l| l.trim()).collect());
    let template =
        fs::read_to_string("templates/post.template.html").expect("could not find template file");
    let content = template.as_str().replace("{{ html_template }}", &html_str);
    let out_file = &format!("out/{}.html", file_name);
    fs::write(out_file, content).expect("unable to write to index.html");
    println!("Successfully generated {}", out_file.clone());
}
