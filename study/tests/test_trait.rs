// Define a trait
trait Printable {
    fn format(&self) -> String;
    fn print(&self) {
        println!("{}", self.format());
    }
}

// Define some structs
struct Book {
    title: String,
    author: String,
    pages: u32,
}

struct Article {
    headline: String,
    author: String,
}

// Implement the Printable trait for Book
impl Printable for Book {
    fn format(&self) -> String {
        format!("Book: '{}' by {}, {} pages", self.title, self.author, self.pages)
    }
}

// Implement the Printable trait for Article
impl Printable for Article {
    fn format(&self) -> String {
        format!("Article: '{}' by {}", self.headline, self.author)
    }
}

// Function that uses the Printable trait
// 2 different but equivalent way of definition
// fn print_item(item: &impl Printable) {
fn print_item<T: Printable>(item: &T) {
    item.print();
}

#[cfg(test)]
mod trait_test {
    use super::*;

    #[test]
    fn test_print_by_trait() {
        let book = Book {
            title: String::from("The Rust Programming Language"),
            author: String::from("Steve Klabnik and Carol Nichols"),
            pages: 560,
        };

        let article = Article {
            headline: String::from("Rust 2021 Edition Released"),
            author: String::from("Rust Team"),
        };

        // Use the trait methods directly
        book.print();
        article.print();

        // Use the function that accepts any Printable item
        print_item(&book);
        print_item(&article);
    }
}