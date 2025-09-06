/*
    A trait object points to both an instance of a type implementing our
    specified trait and a table used to look up trait methods on that
    type at runtime.

    trait objects' specific purpose is to allow abstraction across common
    behavior

    - trait object -> flexibility at runtime, safety at compile time
    - generics -> static dispatch (at compile time)
    - trait object -> dynamic dispatch (at runtime via vtable lookup)
    - vtable -> virtual method table
    - trait objects can be used in the place of both generic types or
    concrete types

    - for example a vector cannot include elements of different types. there
    are some workaround for this. we can define an enum like Various
    and make its variants carry various types. But this workaround needs
    all of its instance (variants) to be defined at compile time. if we
    need to store values with different types which all of the type's
    variants are not clear at compile time we should use trait objects

    - monomorphization performed on generics by compiler but it cannot
    be done for trait objects
    - trait object incur some runtime cost
*/

use oop::{Button, Gui, Image};

struct Screen {
    components: Vec<Box<dyn Gui>>,
}

impl Screen {
    fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

struct Link {
    href: String,
}

impl Gui for Link {
    fn draw(&self) {
        println!("href: {}", self.href)
    }
}

#[derive(Debug)]
enum Various {
    Text(String),
    Number(i32),
    ArcSize(usize),
}

impl std::fmt::Display for Various {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Various::Text(s) => write!(f, "{s}"),
            Various::Number(n) => write!(f, "{n}"),
            Various::ArcSize(a) => write!(f, "{a}"),
        }
    }
}

fn main() {
    let mut list: Vec<Box<dyn FnOnce()>> = Vec::new();

    list.push(Box::new(|| println!("some text")));
    list.push(Box::new(|| println!("some text")));

    let heterogenous_like_vector = vec![
        Various::Text(String::new()),
        Various::Number(12),
        Various::ArcSize(30_usize),
    ];

    heterogenous_like_vector
        .iter()
        .for_each(|e| println!("{e}"));

    let components = vec![
        Box::new(Link {
            href: "h".to_string(),
        }) as Box<dyn Gui>,
        Box::new(Image {
            width: 20,
            height: 25,
        }),
        Box::new(Button {
            text: String::from("button text"),
            width: 13,
            height: 7,
        }),
    ];

    let screen = Screen { components };

    screen.run();
}
