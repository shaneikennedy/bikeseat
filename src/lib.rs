use regex::Regex;
use std::fs;

const H1: &str = "# ";
const H2: &str = "## ";
const H3: &str = "### ";
const H4: &str = "#### ";
const H5: &str = "##### ";
const H6: &str = "###### ";
const LIST_BULLET: &str = "* ";
const BLOCKQUOTE: &str = "> ";

const BACKTICKS: &str = "```";

const BOLD_TEXT: (&str, &str) = (r"\*\*([\s\S]+)\*\*", r"<b>$1</b>");
const ITALIC_TEXT: (&str, &str) = (r"\*([\s\S]+)\*", r"<i>$1</i>");
const ITALIC_TEXT_2: (&str, &str) = (r"_([\s\S]+)_", r"<i>$1</i>");
const CODE: (&str, &str) = (r"`([\s a-zA-z0-9<>\\/~-]+)`", r"<code>$1</code>");
const ANCHOR: (&str, &str) = (
    r"\[([\s\S]+)\]\(([A-Za-z0-9 ~\-:\/.]+)\)",
    r#"<a href="$2">$1</a>"#,
);
const IMAGE: (&str, &str) = (
    r"!\[([\s\S]+)\]\(([A-Za-z0-9 ~\-:\/.]+)\)",
    r#"<img src="$2" alt="$1">"#,
);

const BLOCK_ELEMENTS: &[&str] = &[BACKTICKS];
const LINE_ELEMENTS: &[&str] = &[H1, H2, H3, H4, H5, H6, LIST_BULLET, BLOCKQUOTE];
const STYLE_ELEMENTS: &[(&str, &str)] =
    &[BOLD_TEXT, ITALIC_TEXT, ITALIC_TEXT_2, CODE, IMAGE, ANCHOR];

pub struct Parser {}

impl Parser {
    pub fn parse_md(lines: Vec<&str>) -> String {
        let mut html_str = String::new();
        let mut in_a_block = false;
        let mut block_html = String::new();
        for line in lines {
            if Parser::is_block_element(line) {
                if !in_a_block {
                    block_html += "<div><code>"; // implement find block type
                } else {
                    block_html += "</code></div>";
                    html_str += &format!("{}\n", block_html);
                    block_html = String::new();
                }
                in_a_block = !in_a_block;
                continue;
            }

            if !in_a_block {
                let mut html_line: String;
                if Parser::is_line_element(line) {
                    html_line = format!("<div>{}</div>", &Parser::parse_line(line));
                } else if line == "" {
                    html_line = "<br>".to_string();
                } else {
                    html_line = format!("<div>{}</div>", line);
                }
                for (style_re, replace) in STYLE_ELEMENTS {
                    html_line = Parser::format_line(&html_line, style_re, replace);
                }
                html_str += &format!("<div>{}</div>", html_line);
            } else {
                block_html += &format!("<div>{}</div>", line);
            }
        }
        html_str
    }

    fn is_line_element(line: &str) -> bool {
        for el in LINE_ELEMENTS {
            if line.starts_with(el) {
                return true;
            }
        }
        return false;
    }

    fn is_block_element(line: &str) -> bool {
        for el in BLOCK_ELEMENTS {
            if line.starts_with(el) {
                return true;
            }
        }
        return false;
    }

    fn format_line(line: &str, regex: &&str, replace: &&str) -> String {
        let re = Regex::new(regex).unwrap();
        re.replace_all(&line, *replace).to_string()
    }

    fn parse_line(line: &str) -> String {
        if line.starts_with(H1) {
            let content = line.split_once(H1).unwrap();
            return format!("<h1>{}</h1>", content.1);
        } else if line.starts_with(H2) {
            let content = line.split_once(H2).unwrap();
            return format!("<h2>{}</h2>", content.1);
        } else if line.starts_with(H3) {
            let content = line.split_once(H3).unwrap();
            return format!("<h3>{}</h3>", content.1);
        } else if line.starts_with(H4) {
            let content = line.split_once(H4).unwrap();
            return format!("<h4>{}</h4>", content.1);
        } else if line.starts_with(H5) {
            let content = line.split_once(H5).unwrap();
            return format!("<h5>{}</h5>", content.1);
        } else if line.starts_with(H6) {
            let content = line.split_once(H6).unwrap();
            return format!("<h6>{}</h6>", content.1);
        } else if line.starts_with(LIST_BULLET) {
            let content = line.split_once(LIST_BULLET).unwrap();
            return format!("<li>{}</li>", content.1);
        } else if line.starts_with(BLOCKQUOTE) {
            let content = line.split_once(BLOCKQUOTE).unwrap();
            return format!("<blockquote>{}</blockquote>", content.1);
        }
        return String::new();
    }
}

#[cfg(test)]
mod parser_tests {
    use super::*;
    #[test]
    fn parse_line_h1() {
        let html = Parser::parse_line("# this is an h1");
        assert_eq!("<h1>this is an h1</h1>", html);
    }

    #[test]
    fn parse_line_h2() {
        let html = Parser::parse_line("## this is an h2");
        assert_eq!("<h2>this is an h2</h2>", html);
    }
    #[test]
    fn parse_line_h3() {
        let html = Parser::parse_line("### this is an h3");
        assert_eq!("<h3>this is an h3</h3>", html);
    }
    #[test]
    fn parse_line_h4() {
        let html = Parser::parse_line("#### this is an h4");
        assert_eq!("<h4>this is an h4</h4>", html);
    }
    #[test]
    fn parse_line_h5() {
        let html = Parser::parse_line("##### this is an h5");
        assert_eq!("<h5>this is an h5</h5>", html);
    }
    #[test]
    fn parse_line_h6() {
        let html = Parser::parse_line("###### this is an h6");
        assert_eq!("<h6>this is an h6</h6>", html);
    }

    #[test]
    fn parse_line_bullet_point() {
        let html = Parser::parse_line("* this is an li");
        assert_eq!("<li>this is an li</li>", html);
    }

    #[test]
    fn parse_line_blockquote() {
        let html = Parser::parse_line("> this is a blockquote");
        assert_eq!("<blockquote>this is a blockquote</blockquote>", html);
    }

    #[test]
    fn format_anchor_tag() {
        let html = Parser::format_line("[hello world!](https://google.com)", &ANCHOR.0, &ANCHOR.1);
        assert_eq!("<a href=\"https://google.com\">hello world!</a>", html);
    }

    #[test]
    fn format_italic_text() {
        let html = Parser::format_line("*hello*", &ITALIC_TEXT.0, &ITALIC_TEXT.1);
        assert_eq!("<i>hello</i>", html);
    }

    #[test]
    fn format_italic_text2() {
        let html = Parser::format_line("_hello_", &ITALIC_TEXT_2.0, &ITALIC_TEXT_2.1);
        assert_eq!("<i>hello</i>", html);
    }

    #[test]
    fn format_inline_code() {
        let html = Parser::format_line("`hello`", &CODE.0, &CODE.1);
        assert_eq!("<code>hello</code>", html);
    }

    #[test]
    fn format_bold_text() {
        let html = Parser::format_line("**hello**", &BOLD_TEXT.0, &BOLD_TEXT.1);
        assert_eq!("<b>hello</b>", html);
    }

    #[test]
    fn format_bold_italic_anchor_text() {
        let mut html_line: String = "***[hello](https://google.com)***".to_string();
        for (style_re, replace) in STYLE_ELEMENTS {
            html_line = Parser::format_line(&html_line, style_re, replace);
        }
        assert_eq!(
            "<b><i><a href=\"https://google.com\">hello</a></i></b>",
            html_line
        );
    }

    #[test]
    fn format_img() {
        let html = Parser::format_line("![alt](image-url)", &IMAGE.0, &IMAGE.1);
        assert_eq!("<img src=\"image-url\" alt=\"alt\">", html);
    }
}

pub struct Renderer {}

impl Renderer {
    pub fn render_home(posts: Vec<String>) {
        let mut html_str = String::new();
        for post in posts {
            let post_name = post.split(".").nth(0).unwrap();
            html_str += &format!("<li><a href='{}'>{}</a></li>\n", post_name, post_name);
        }
        let template = fs::read_to_string("templates/index.template.html").unwrap();
        let content = template.as_str().replace("{{ html_template }}", &html_str);
        let out_file = "out/index.html";
        fs::write(out_file, content).expect("unable to write to index.html");
    }

    pub fn render_post(post: String) {
        let markdown: String =
            fs::read_to_string(format!("content/{}", post.clone())).expect("could not read file.");
        let html_str = Parser::parse_md(markdown.lines().map(|l| l.trim()).collect());
        let template = fs::read_to_string("templates/post.template.html").unwrap();
        let content = template.as_str().replace("{{ html_template }}", &html_str);
        let out_file = &format!("out/{}.html", post.split(".").nth(0).unwrap());
        fs::write(out_file, content).expect("unable to write to index.html");
        println!("Successfully generated {}", out_file.clone());
    }
}
