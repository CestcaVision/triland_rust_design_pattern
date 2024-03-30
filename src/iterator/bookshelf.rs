use crate::iterator::book::Book;

pub struct Bookshelf {
    books: Vec<Book>,
}

impl Bookshelf {
    pub fn new() -> Bookshelf {
        Bookshelf { books: Vec::new() }
    }
    pub fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }
    pub fn get_books(&self) -> &Vec<Book> {
        &self.books
    }
}
pub struct BookshelfIterator<'a> {
    bookshelf: &'a Bookshelf,
    index: usize,
}

impl<'a> Iterator for BookshelfIterator<'a> {
    type Item = &'a Book;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.bookshelf.books.len() {
            let result = &self.bookshelf.books[self.index];
            self.index += 1;
            Some(result)
        } else {
            None
        }
    }
}

impl<'a> IntoIterator for &'a Bookshelf {
    type Item = &'a Book;
    type IntoIter = BookshelfIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        BookshelfIterator {
            bookshelf: self,
            index: 0,
        }
    }
}
