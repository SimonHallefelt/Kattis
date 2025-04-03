use scraper::{Html, Selector};
use reqwest::blocking::get;
mod algoritmer;

fn main() {
    println!("Hello, world!");

    //get page
    let url_str = "https://open.kattis.com/problems?page=1&order=title_link&f_solved=on&f_partial-score=off&f_tried=off&f_untried=off&f_language=-1";
    let response = get(url_str).unwrap().text().unwrap();
    let doc_body = Html::parse_document(&response);

    //get table
    let class = Selector::parse(".table2").unwrap();
    let mut s_table = doc_body.select(&class);
    let table = s_table.next().unwrap();
    /* let titles = table.text().collect::<Vec<_>>();
    println!("{:?}", titles);
    println!(""); */



    //get tbody for solvde problems
    let s_tbody = Selector::parse("tbody").unwrap();
    for tbody in table.select(&s_tbody) {
        let titles = tbody.text().collect::<Vec<_>>();
        println!("{:?}", titles);
        println!("");
        println!("");
        println!("");
    }





    println!("Goodbye, world!");
}

/* fn main() {
    let response = get("https://news.ycombinator.com").unwrap().text().unwrap();
    let doc_body = Html::parse_document(&response);
    let title = Selector::parse(".titleline").unwrap();
    for title in doc_body.select(&title) {
        let titles = title.text().collect::<Vec<_>>();
        println!("{:?}", titles);
    }
} */