use futures::future::{self, Either, join};
use reqwest;
use scraper::Html;
use std::{pin::pin, time::Duration};
use tokio::{
    self,
    runtime::Runtime,
    sync::mpsc::{UnboundedReceiver, UnboundedSender, unbounded_channel},
    task::spawn as spawn_task,
    time::sleep,
};

/*
  | `tokio::sync::mpsc` | `std::sync::mpsc` |
  | ------------------- | ----------------- |
  | `channel`           | `sync_channel`    |
  | `unbounded_channel` | `channel`         |
*/

pub fn get_one_title_and_print() {
    let args: Vec<String> = std::env::args().collect();
    let url = &args[1];

    let runtime = Runtime::new().unwrap();
    runtime.block_on(async {
        let maybe_title = get_title(url).await;
        match maybe_title {
            Some(title) => println!("The title is : {title}"),
            None => println!("The url {url} had no title"),
        }
    });
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

pub async fn race_between_two_title() {
    let args: Vec<String> = std::env::args().collect();
    let url1 = &args[1];
    let url2 = &args[2];

    match select(get_title(url1), get_title(url2)).await {
        Either::Left(mut title1) => println!(
            "The title is : {}",
            title1.get_or_insert("NO TITLE".to_string())
        ),
        Either::Right(mut title2) => println!(
            "The title is : {}",
            title2.get_or_insert("NO TITLE".to_string())
        ),
    };
}

pub fn counter_with_task() {
    let runtime = Runtime::new().unwrap();

    runtime.block_on(async {
        let handle = spawn_task(async {
            for i in 0..10 {
                sleep(Duration::from_millis(500)).await;
                println!("{i} from spawned task");
            }
        });

        for j in 0..5 {
            sleep(Duration::from_millis(500)).await;
            println!("{j} from main")
        }

        handle.await.unwrap();
    });
}
