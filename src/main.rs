use std::fs;

use md::Parser;

fn main() {
    let markdown: String = fs::read_to_string("content/index.md").expect("could not read file.");
    let html_str = Parser::parse_md(markdown.lines().map(|l| l.trim()).collect());
    let template =
        fs::read_to_string("templates/post.template.html").expect("could not find template file");
    let content = template.as_str().replace("{{ html_template }}", &html_str);
    fs::write("index.html", content).expect("unable to write to index.html");
}
