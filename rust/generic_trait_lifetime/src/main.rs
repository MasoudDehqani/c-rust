use generic_trait_lifetime::Summary;
use std::cmp::PartialOrd;
// use std::fmt::{Debug, Display};

#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<T, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

impl Point<f64, f64> {
    fn distance_of_x_and_y(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

enum Variants<T> {
    Variant1(T),
    Variant2,
}

struct NewsArticle {
    headline: String,
    author: String,
    content: String,
}

impl Summary for NewsArticle {
    fn summary(&self) -> String {
        format!("{} by {}", self.headline, self.author)
    }
    fn summary_with_default(&self) -> String {
        format!("Bishin binim baaa {}", self.author)
    }
}

#[derive(Debug)]
struct SocialPost {
    username: String,
    content: String,
    repost: bool,
}

impl Summary for SocialPost {
    fn summary(&self) -> String {
        format!(
            "{} by {} and is reposted: {}",
            &self.content, &self.username, &self.repost
        )
    }
}

fn main() {
    let l = largest(&[1, 0, 25, 112, 7]);
    let p = Point { x: 2, y: 2.2 };
    let p2 = Point { x: 2.2, y: 3.2 };
    let p3 = Point { x: 5.3, y: 1.1 };
    let p4 = Point { x: 9.1, y: 2.4 };
    let _p5 = p3.mixup(p4);
    let _x = p.x();
    let dis = p2.distance_of_x_and_y();
    let _v = Variants::Variant1(Point { x: 12.2, y: 1 });
    let _v2: Variants<Point<f64, i32>> = Variants::Variant2;

    println!("{l:?}");
    println!("{p:?}, {}, {}", p.x, p.y);
    println!("{dis}");

    let article = NewsArticle {
        headline: String::from("headline"),
        author: String::from("author"),
        content: String::new(),
    };

    let sum = article.summary();
    println!(
        "{sum}, {1}, {0}, {2}",
        article.headline, article.author, article.content
    );
    println!("{}", article.summary_with_default());

    let social_post = SocialPost {
        username: String::from("post username"),
        content: String::from("post content"),
        repost: true,
    };

    println!("{}", social_post.summary());
    println!("{}", social_post.summary_with_default());
    println!("{}", social_post.summary_from_summary());

    notify(&social_post);
    // notify2(&article, &social_post);
}

fn largest<T: PartialOrd>(lst: &[T]) -> &T {
    let mut largest = &lst[0];

    for l in lst {
        if l > largest {
            largest = l;
        }
    }

    largest
}

/*
    trait bound implementation as parameter with various syntax
*/

// fn notify(item: &impl Summary) -> String {
//     format!("{}", item.summary_from_summary())
// }
// fn notify<T: Summary>(item: &T) -> String {
//     format!("{}", item.summary_with_default())
// }
fn notify<T>(item: &T) -> String
where
    T: Summary,
{
    format!("{}", item.summary_from_summary())
}
// fn notify2<T>(item1: &(impl Summary + Display), item2: &(impl Summary + Display)) -> String {
//     format!(
//         "{}",
//         item1.summary_from_summary() + &item2.summary_with_default()
//     )
// }
// fn notify2<T: Summary + Display>(item1: &T, item2: &T) -> String {
//     format!(
//         "{}",
//         item1.summary_from_summary() + &item2.summary_with_default()
//     )
// }
// fn notify2<T>(item1: &T, item2: &T) -> String
// where
//     T: Summary + Display,
// {
//     format!(
//         "{}",
//         item1.summary_from_summary() + &item2.summary_with_default()
//     )
// }

/*
    trait bound as
*/
