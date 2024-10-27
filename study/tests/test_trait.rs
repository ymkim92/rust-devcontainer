use std::io::{self, Write};

pub fn write_data<W: Write>(mut writer: W, data: &str) -> io::Result<()> {
    writer.write_all(data.as_bytes())
}

// Usage in library code
pub fn process_and_output<W: Write>(output: W, data: &str) -> io::Result<()> {
    write_data(output, data)
}

// Define a trait
trait Printable {
    fn format(&self) -> String;
    fn print<W: Write>(&self, output: W) -> io::Result<()> {
        process_and_output(output, &self.format())
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
fn print_item<T: Printable, W: Write>(item: &T, output: W) -> io::Result<()>{
    item.print(output)
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

        // Use the trait methods directly``
        assert!(book.print(std::io::stdout()).is_ok());
        assert!(article.print(std::io::stdout()).is_ok());

        // Use the function that accepts any Printable item
        assert!(print_item(&book, std::io::stdout()).is_ok());
        assert!(print_item(&article, std::io::stdout()).is_ok());
        
        let mut book_buffer1 = Vec::new();
        let mut book_buffer2 = Vec::new();
        book.print(&mut book_buffer1).unwrap();        
        print_item(&book, &mut book_buffer2).unwrap();        
        assert_eq!(book_buffer1, book_buffer2);

        let mut article_buffer1 = Vec::new();
        let mut article_buffer2 = Vec::new();
        article.print(&mut article_buffer1).unwrap();        
        print_item(&article, &mut article_buffer2).unwrap();        
        assert_eq!(article_buffer1, article_buffer2);
    }
}
