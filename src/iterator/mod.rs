//! # Iterator Pattern
//! Iterator pattern is a behavioral design pattern that provides a way to access the elements of an aggregate object sequentially without exposing its underlying representation.

pub mod book;
pub mod bookshelf;
pub use book::Book;
pub use bookshelf::Bookshelf;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_book_shelf_iterator() {
        let mut bookshelf = Bookshelf::new();
        bookshelf.add_book(Book::new("The Brothers Karamazov", "Fyodor Dostoevsky"));
        bookshelf.add_book(Book::new("Crime and Punishment", "Fyodor Dostoevsky"));
        bookshelf.add_book(Book::new("War and Peace", "Leo Tolstoy"));

        let mut iter = bookshelf.into_iter();
        assert_eq!(iter.next().unwrap().get_name(), "The Brothers Karamazov");
        assert_eq!(iter.next().unwrap().get_name(), "Crime and Punishment");
        assert_eq!(iter.next().unwrap().get_name(), "War and Peace");

        for book in bookshelf.into_iter() {
            println!(
                "Book name: {}\nAuthor name: {}",
                book.get_name(),
                book.get_author()
            );
        }
    }
}
