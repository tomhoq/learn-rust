>use std::any::Any;

#[derive(Debug)]
enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Audiobook { title: String },
    Podcast(u32),
    Placeholder,
}

impl Media {
    fn description(&self) -> String {
        match self {
            Media::Book { title, author } => format!("Book: {} by {}", title, author),
            Media::Movie { title, director } => {
                format!("Movie: {} directed by {}", title, director)
            }
            Media::Audiobook { title } => format!("Audiobook: {}", title),
            Media::Podcast(id) => format!("Number: {}", id),
            Media::Placeholder => format!("Placeholder"),
            _ => String::from("No type"),
        }
    }
}

#[derive(Debug)]
struct Catalog {
    items: Vec<Media>,
}

impl Catalog {
    fn new() -> Self {
        Catalog { items: vec![] }
    }

    fn add(&mut self, media: Media) {
        self.items.push(media);
    }
}

fn print_media(media: Media) {
    println!("{:#?}", media)
}

fn main() {
    let media = Media::Audiobook {
        title: String::from("Ti2tee"),
    };

    let movie = Media::Movie {
        title: String::from("Good Movie"),
        director: String::from("Mr Jenkins"),
    };

    let book = Media::Book {
        title: String::from("Good Movie"),
        author: String::from("Mr Jenkins"),
    };

    let mut catalog = Catalog::new();
    catalog.add(movie);
    catalog.add(book);
    catalog.add(media);
    println!("{:#?}", catalog);
}
