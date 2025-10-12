use futures::future::{self, Either};
use reqwest;
use scraper::Html;
use std::pin::pin;
use tokio::{self, runtime::Runtime};

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    let url1 = &args[1];
    let url2 = &args[2];

    // let maybe_title_1 = get_title(url1).await;
    // let maybe_title_2 = get_title(url2).await;

    match select(get_title(url1), get_title(url2)).await {
        Either::Left(mut title1) => println!(
            "The title is : {}",
            title1.get_or_insert("NO TITLE".to_string())
        ),
        Either::Right(mut title2) => println!(
            "The title is : {}",
            title2.get_or_insert("NO TITLE".to_string())
        ),
    }

    // match maybe_title {
    //     Some(title) => println!("The title is : {title}"),
    //     None => println!("The url {url} had no title"),
    // }

    // let runtime = Runtime::new().unwrap();
    // runtime.block_on(async {
    //     let maybe_title = get_title(url).await;
    //     match maybe_title {
    //         Some(title) => println!("The title is : {title}"),
    //         None => println!("The url {url} had no title"),
    //     }
    // });
}

async fn get_title(url: &str) -> Option<String> {
    let response = reqwest::get(url).await.unwrap();
    let response_text = response.text().await.unwrap();
    Html::parse_document(&response_text)
        .select(&scraper::Selector::parse("title").unwrap())
        .nth(0)
        .map(|t| t.inner_html())
}

async fn select<A, B, F1, F2>(f1: F1, f2: F2) -> Either<A, B>
where
    F1: Future<Output = A>,
    F2: Future<Output = B>,
{
    let f1 = pin!(f1);
    let f2 = pin!(f2);

    match future::select(f1, f2).await {
        Either::Left((a, _)) => Either::Left(a),
        Either::Right((b, _)) => Either::Right(b),
    }
}
