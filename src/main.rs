use scraper::{Html, Selector};
use tokio::runtime::Runtime;

fn main() {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        let title = page_title("https://www.rust-lang.org").await;

        println!("title:::{}", title.unwrap().trim());
    })
}

async fn page_title(url: &str) -> Option<String> {
    let res = reqwest::get(url).await.ok()?;

    let res_text = res.text().await.ok()?;

    let document = Html::parse_document(&res_text);

    let selector = Selector::parse("title").ok()?;
    // println!("selector:: {selector:?}");

    let mut doc_selec = document.select(&selector);
    // println!("doc_selec:: {doc_selec:?}");

    let doc_selec_next = doc_selec.next();
    println!("doc_selec_next:: {:?}", doc_selec_next);

    let title = doc_selec_next.map(|t| t.inner_html());
    println!("title:: {:?}", (title));

    title
}
