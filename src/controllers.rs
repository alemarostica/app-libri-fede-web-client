use crate::{
    state::{BookAction, BookState},
    models::*,
    api,
};
use web_sys::console;
use yew::UseReducerHandle;

pub struct BookController {
    state: UseReducerHandle<BookState>,
}

impl BookController {
    pub fn new(state: UseReducerHandle<BookState>) -> BookController {
        BookController { state }
    }

    pub fn init_books(&self) {
        let books = self.state.clone();
        wasm_bindgen_futures::spawn_local(async move {
            match api::fetch_books().await {
                Ok(fetched_books) => {
                    books.dispatch(BookAction::Set(fetched_books));
                }
                Err(e) => {
                    console::error_1(&format!("Failed to fetch books: {}", e).into());
                }
            }
        });
    }

    pub fn create_book(&self, book: NewBook) {
        let books = self.state.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let response = api::create_book(&book).await.unwrap();
            books.dispatch(BookAction::Add(response))
        });
    }

    pub fn toggle_read(&self, id: String) {
        let books = self.state.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let response = api::toggle_read(id.clone()).await.unwrap();
            // TODO: Manca checkare la risposta qua, perché non uso il sistema di rows affected
            // Quindi anche l'unwrap sopra potrebbe fallire I guess
            books.dispatch(BookAction::Toggle(response))
        });
    }

    pub fn update_book(&self, id: String, book: NewBook) {
        let books = self.state.clone();
        wasm_bindgen_futures::spawn_local(async move {
            // TODO: Same as above
            let response = api::update_book(&book, id.clone()).await.unwrap();
            books.dispatch(BookAction::Update(response));
        });
    }

    pub fn delete_book(&self, id: String) {
        let books = self.state.clone();
        wasm_bindgen_futures::spawn_local(async move {
            // TODO: Same as above
            let _ = api::delete_book(id.clone()).await.unwrap();
            books.dispatch(BookAction::Delete(id.clone()));
        });
    }
}
