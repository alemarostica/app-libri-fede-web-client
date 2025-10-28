use std::{rc::Rc};
use yew::Reducible;
use crate::models::Book;

pub enum BookAction {
    Set(Vec<Book>),
    Add(Book),
    Delete(String),
    Toggle(Book),
    Update(Book),
}

pub struct BookState {
    pub books: Vec<Book>,
}

impl Default for BookState {
    fn default() -> Self {
        Self { books: vec![] }
    }
}


impl Reducible for BookState {
    type Action = BookAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let next_books = match action {
            BookAction::Set(books) => books,
            BookAction::Add(book) => {
                let mut books = self.books.clone();
                books.push(book);
                books
            },
            BookAction::Delete(id) => {
                let mut books = self.books.clone();
                books.retain(|book| book.id != id);
                books
            },
            BookAction::Toggle(updated_book) => {
                let mut books = self.books.clone();
                if let Some(index) = books.iter().position(|b| b.id == updated_book.id) {
                    books[index] = updated_book;
                }
                books
            }
            BookAction::Update(updated_book) => {
                let mut books = self.books.clone();
                if let Some(index) = books.iter().position(|b| b.id == updated_book.id) {
                    books[index] = updated_book;
                }
                books
            }
        };

        Self { books: next_books }.into()
    }
}
