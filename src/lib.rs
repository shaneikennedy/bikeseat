use regex::Regex;
use std::fs;
use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

const H1: &str = "# ";
const H2: &str = "## ";
const H3: &str = "### ";
const H4: &str = "#### ";
const H5: &str = "##### ";
const H6: &str = "###### ";
const LIST_BULLET: &str = "* ";
const BLOCKQUOTE: &str = "> ";

const BACKTICKS: &str = "```";

const BOLD_TEXT: (&str, &str) = (r"\*\*([0-9A-Za-z]+)\*\*", r"<b>$1</b>");
const ITALIC_TEXT: (&str, &str) = (r"\*([0-9A-Za-z]+)\*", r"<i>$1</i>");
const ITALIC_TEXT_2: (&str, &str) = (r"_([0-9A-Za-z]+)_", r"<i>$1</i>");
const CODE: (&str, &str) = (r"`([A-Za-z0-9 ~-]+)`", r"<code>$1</code>");

const BLOCK_ELEMENTS: &[&str] = &[BACKTICKS];
const LINE_ELEMENTS: &[&str] = &[H1, H2, H3, H4, H5, H6, LIST_BULLET, BLOCKQUOTE];
const STYLE_ELEMENTS: &[(&str, &str)] = &[BOLD_TEXT, ITALIC_TEXT, ITALIC_TEXT_2, CODE];

pub struct Parser {}

impl Parser {
    pub fn parse_md(lines: Vec<&str>) -> String {
        let mut html_str = String::new();
        let mut in_a_block = false;
        let mut block_html = String::new();
        for line in lines {
            if Parser::is_block_element(line) {
                if !in_a_block {
                    block_html += "<code>"; // implement find block type
                } else {
                    block_html += "</code>";
                    html_str += &format!("{}\n", block_html);
                    block_html = String::new();
                }
                in_a_block = !in_a_block;
                continue;
            }

            if !in_a_block {
                let mut html_line: String;
                if Parser::is_line_element(line) {
                    html_line = format!("{}", &Parser::parse_line(line));
                } else if line == "" {
                    html_line = "".to_string();
                } else {
                    html_line = format!("<div>{}</div>", line);
                }
                for (style_re, replace) in STYLE_ELEMENTS {
                    let re = Regex::new(style_re).unwrap();
                    html_line = re.replace_all(&html_line, *replace).to_string();
                }
                html_str += &format!("{}\n", html_line);
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
    fn parse_md() {
        let content = "\
        # this is an h1
        ## this is an h2
        ### this is an h3
        #### this is an h4
        ##### this is an h5
        ###### this is an h6
        * this is a li
        > this is a blockquote
        this is nothing
        "
        .lines()
        .map(|l| l.trim())
        .collect();
        let html_str = Parser::parse_md(content);
        let expected_html = "\
        <h1>this is an h1</h1>
        <h2>this is an h2</h2>
        <h3>this is an h3</h3>
        <h4>this is an h4</h4>
        <h5>this is an h5</h5>
        <h6>this is an h6</h6>
        <li>this is a li</li>
        <blockquote>this is a blockquote</blockquote>
        <div>this is nothing</div>
        "
        .lines()
        .map(|l| l.trim_start())
        .fold(String::new(), |acc, e| acc + e + "\n");
        assert_eq!(expected_html, html_str);
    }
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
}

pub struct Renderer {}

impl Renderer {
    pub fn render_home(posts: Vec<String>) {
        let mut html_str = String::new();
        for post in posts {
            html_str += &format!("<li><a href='{}.html'>{}</a></li>\n", post, post);
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
        let out_file = &format!("out/{}.html", post);
        fs::write(out_file, content).expect("unable to write to index.html");
        println!("Successfully generated {}", out_file.clone());
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");

                    job();
                }
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down.");
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}
