use clap::Parser;
use ego_tree::iter::Edge;
use html_parser::{Dom,Result,Node};
use scraper::ElementRef;
use ego_tree::NodeRef;
use scraper::Node::Text;
use std::io::Read;
use std::str;
use std::io;
use htmlentity::entity;
use regex::Regex;
use scraper::{Html, Selector};

#[derive(Parser)]
#[clap(about, version, author)]
struct Args {
    #[clap(short, long)]
    name: String,

    #[clap(short, long, default_value_t = 1)]
    count: u8,
}

fn main() -> Result<()> {
    // let mut buffer = String::new();
    let mut html = String::new();
    let mut stdin = io::stdin(); // We get `Stdin` here.
    stdin.read_to_string(&mut html)?;

    // remove any html 4 doctype from html using regex ignoring case
    let re = Regex::new(r"(?i)<!doctype[^>]*>").unwrap();
    html = re.replace_all(&html, "").to_string();

    // add html5 doctype to html
    html = format!("<!doctype html>\n{}", html);

    let doc = Html::parse_document(&html.replace("\n", ""));

    let mut s = String::new();

    let mut is_body = false;
    let mut consecutive_block = false;


    let re = Regex::new(r"\s+").unwrap();
    for edge in doc.root_element().traverse() {
        match edge {
            Edge::Open(node) if !node.has_children() => {
                if !is_body {
                    continue
                }
                if let scraper::Node::Text(t) = node.value() {
                    let c = re.replace_all(t, " ");
                    if s.chars().last() == Some('\n') {
                        s.push_str(c.trim_start());
                    } else {
                        s.push_str(&c)
                    }
                }
                consecutive_block = false;
            },
            Edge::Open(node) => {
                if let scraper::Node::Element(el) = node.value() {
                    if el.name() == "body" {
                        is_body = true
                    }
                    if !is_body {
                        continue
                    }

                    match el.name() {
                        "a" => {
                            s.push_str("\x1B]8;;");
                            if let Some(href) = el.attr("href") {
                                s.push_str(href);
                            }
                            s.push_str("\x07\x1B[4m\x1B[1;34m");
                            consecutive_block = false;
                        },
                        "strong" | "b" => {
                            s.push_str("\x1b[1m\x1B[1;33m");
                            consecutive_block = false;
                        },
                        "img" => {
                            if let Some(src) = el.attr("src") {
                                s.push_str("\x1B]8;;");
                                s.push_str(src);
                                s.push_str("\x07\x1B[4m\x1B[1;34m");
                                s.push_str("img");
                                s.push_str("\x1B[24m\x1B[1;0m\x1B]8;;\x07")
                            }
                            consecutive_block = false;
                        }
                        "li" => {
                            s.push_str("\n- ");
                        }
                        "article" | "aside" | "blockquote" | "details" | "dialog" | "dd" | "div" | "dl" | "dt" | "fieldset" | "figcaption" | "figure" | "footer" | "form" | "h1" | "h2" | "h3" | "h4" | "h5" | "h6" | "header" | "hgroup" | "hr" | "main" | "nav" | "ol" | "p" | "pre" | "section" | "table" | "ul" | "tr"  => {
                            s = s.trim_end().to_string();
                            s.push('\n');
                            s.push('\n');
                            consecutive_block = true;
                        }
                        _ => {
                            consecutive_block = false;
                        }
                    }
                }
            },
            Edge::Close(node) => {
                if let scraper::Node::Element(el) = node.value() {
                    match el.name() {
                        "a" => {
                            s.push_str("\x1B[24m\x1B[1;0m\x1B]8;;\x07")
                        },
                        "strong" | "b" => {
                            s.push_str("\x1b[22m\x1B[1;0m");
                        },
                        "br" => {
                            s = s.trim_end().to_string();
                            s.push_str("\n");
                            consecutive_block = false;
                        }
                        "hr" => {
                            s.push_str("\n");
                            s.push_str("-".repeat(80).as_str());
                            s.push_str("\n");
                            consecutive_block = false;
                        }
                        "address" | "article" | "aside" | "blockquote" | "details" | "dialog" | "dd" | "div" | "dl" | "dt" | "fieldset" | "figcaption" | "figure" | "footer" | "form" | "h1" | "h2" | "h3" | "h4" | "h5" | "h6" | "header" | "hgroup" | "main" | "nav" | "ol" | "p" | "pre" | "section" | "table" | "ul" | "tr"  => {
                            consecutive_block = false;
                        }
                        _ => {
                        }
                    }
                }
            }
        }
        // println!("{:?}\n\n\n", el);
    }

    // let s = testPrint(doc.root_element());


    // let dom = Dom::parse(&html)?;
    // let mut s = String::new();
    // for c in dom.children {
    //     match render(&c) {
    //         Ok(text) => s.push_str(&text),
    //         Err(e) => eprintln!("{}", e)
    //     }
    // }
    println!("{}", s.trim());

    Ok(())
}
