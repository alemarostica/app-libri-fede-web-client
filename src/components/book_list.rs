use yew::{function_component, html, Callback, Html, Properties};

use super::BookItem;
use crate::models::{Book, NewBook};

#[derive(Properties, PartialEq)]
pub struct BookListProps {
    pub books: Vec<Book>,
    pub on_delete_book: Callback<String>,
    pub on_toggle_book: Callback<String>,
    pub on_edit_book: Callback<(String, NewBook)>
}

#[function_component(BookList)]
pub fn book_list(
    BookListProps {
        books,
        on_delete_book,
        on_toggle_book,
        on_edit_book
    }: &BookListProps
) -> Html {
    let books: Html = books
        .iter()
        .map(|book| html!( <BookItem book={book.clone()} on_delete_book={on_delete_book} on_toggle_book={on_toggle_book} on_edit_book={on_edit_book}/> ))
        .collect();

    html!(
        <ul id="book-list" class="book-table">
            <li class="book-header">
            <label class="column">{ "Letto" }</label>
            <label class="column">{ "Titolo" }</label>
            <label class="column">{ "Autore" }</label>
            <label class="column">{ "Anno" }</label>
            <label class="column">{ "Editore" }</label>
            <label class="column">{ "Volume" }</label>
            <label class="column">{ "Argomento" }</label>
            <label class="column">{ "Dov'é?" }</label>
            <label />
            <label />
            </li>
            {books}
        </ul>
    )
}
