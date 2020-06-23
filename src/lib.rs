use wasm_bindgen::prelude::*;

use regex::Regex;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn parse(fragment: &str, base_url: &str) -> String {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();

    let fragment = Html::parse_fragment(fragment);

    let mut v: Vec<Contribution> = Vec::new();

    let mut images: Vec<String> = Vec::new();

    let tubu_i = Regex::new(r"su\d{7}\.(jpg|gif|png)").unwrap();
    let tubu = Regex::new(r"su\d{7}\.[[:alnum:]]{1,5}").unwrap();

    match fragment
        .select(&Selector::parse(".thre>a>img").unwrap())
        .next()
    {
        Some(expr) => images.push(format!(
            "{}{}",
            base_url.to_string(),
            expr.value().attr("src").unwrap()
        )),
        None => (),
    };

    let quote = match fragment
        .select(&Selector::parse(".thre>blockquote").unwrap())
        .next()
    {
        Some(expr) => {
            let mut quote = expr.inner_html();

            for cap in tubu_i.captures_iter(&quote) {
                images.push(format!(
                    "{}{}",
                    "http://www.nijibox5.com/futabafiles/tubu/src/".to_string(),
                    &cap[0]
                ));
            }

            quote = tubu
                .replace_all(
                    &quote,
                    "<a href='http://www.nijibox5.com/futabafiles/tubu/src/$0'>$0</a>",
                )
                .to_string();
            quote
        }
        None => "".to_string(),
    };

    let name = match fragment
        .select(&Selector::parse(".thre>.cnm").unwrap())
        .next()
    {
        Some(expr) => expr.inner_html(),
        None => "".to_string(),
    };

    let title = match fragment
        .select(&Selector::parse(".thre>.csb").unwrap())
        .next()
    {
        Some(expr) => expr.inner_html(),
        None => "".to_string(),
    };

    let sod = match fragment
        .select(&Selector::parse(".thre>.sod").unwrap())
        .next()
    {
        Some(expr) => expr.inner_html(),
        None => "".to_string(),
    };

    let mut id = "".to_string();
    let date = match fragment
        .select(&Selector::parse(".thre>.cnw").unwrap())
        .next()
    {
        // "ID:.{8}"があったら
        Some(d) => match Regex::new(r"^(.*)\s(ID:.{8})")
            .unwrap()
            .captures(&d.inner_html())
        {
            Some(expr) => {
                // idに代入
                id = expr.get(2).unwrap().as_str().to_string();
                // dateにはidより前の部分を代入
                expr.get(1).unwrap().as_str().to_string()
            }
            // なければdateにそのまま代入
            None => d.inner_html(),
        },
        None => "".to_string(),
    };

    v.push(Contribution {
        quote,
        images,
        name,
        title,
        id,
        sod,
        date,
    });

    // tableタグを走査
    for table in fragment.select(&Selector::parse("table").unwrap()) {
        let mut images: Vec<String> = Vec::new();

        match table.select(&Selector::parse("img").unwrap()).next() {
            Some(expr) => images.push(format!(
                "{}{}",
                base_url.to_string(),
                expr.value().attr("src").unwrap()
            )),
            None => (),
        };

        let quote = match table.select(&Selector::parse("blockquote").unwrap()).next() {
            Some(expr) => {
                let mut quote = expr.inner_html();

                for cap in tubu_i.captures_iter(&quote) {
                    images.push(format!(
                        "{}{}",
                        "http://www.nijibox5.com/futabafiles/tubu/src/".to_string(),
                        &cap[0]
                    ));
                }

                quote = tubu
                    .replace_all(
                        &quote,
                        "<a href='http://www.nijibox5.com/futabafiles/tubu/src/$0'>$0</a>",
                    )
                    .to_string();
                quote
            }
            None => continue, // なければスキップ
        };

        let name = match table.select(&Selector::parse(".cnm").unwrap()).next() {
            Some(expr) => expr.inner_html(),
            None => "".to_string(),
        };

        let title = match table.select(&Selector::parse(".csb").unwrap()).next() {
            Some(expr) => expr.inner_html(),
            None => "".to_string(),
        };

        let sod = match table.select(&Selector::parse(".sod").unwrap()).next() {
            Some(expr) => expr.inner_html(),
            None => "".to_string(),
        };

        let mut id = "".to_string();
        let date = match table.select(&Selector::parse(".cnw").unwrap()).next() {
            // "ID:.{8}"があったら
            Some(d) => match Regex::new(r"^(.*)\s(ID:.{8})")
                .unwrap()
                .captures(&d.inner_html())
            {
                Some(expr) => {
                    // idに代入
                    id = expr.get(2).unwrap().as_str().to_string();
                    // dateにはidより前の部分を代入
                    expr.get(1).unwrap().as_str().to_string()
                }
                // なければdateにそのまま代入
                None => d.inner_html(),
            },
            None => "".to_string(),
        };

        v.push(Contribution {
            quote,
            images,
            name,
            title,
            sod,
            id,
            date,
        });
    }

    let thread = Thread { contributions: v };

    serde_json::to_string(&thread).unwrap()
}

#[derive(Serialize, Deserialize, Debug)]
struct Contribution {
    quote: String,
    images: Vec<String>,
    name: String,
    title: String,
    id: String,
    sod: String,
    date: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Thread {
    contributions: Vec<Contribution>,
}
