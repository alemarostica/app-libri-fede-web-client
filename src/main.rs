mod models;
mod state;
mod api;
mod controllers;
mod components;

use std::rc::Rc;
use web_sys::wasm_bindgen::JsCast;
use yew::prelude::*;
use gloo_storage::{Storage};

use state::*;
use controllers::*;
use components::*;

use crate::models::NewBook;

#[function_component]
fn App() -> Html {
    let books = use_reducer(BookState::default);
    let books_controller = Rc::new(BookController::new(books.clone()));
    let theme_state = use_state(|| "light".to_string());

    // Effect to read the theme preference from local storage
    {
        let theme_state = theme_state.clone();
        use_effect_with((), move |_| {
            if let Ok(Some(saved_theme)) = gloo_storage::LocalStorage::get("theme-preference") {
                theme_state.set(saved_theme);
            } else if let Ok(media_query) = web_sys::window().unwrap().match_media("(prefers-color-scheme: dark)") {
                if let Some(mql) = media_query {
                    if mql.matches() {
                        theme_state.set("dark".to_string());
                    }
                }  
            }
            || ()
        });
    }

    // Effect to apply the theme class to the html element whenever the theme state changes
    {
        let theme_state = theme_state.clone();
        use_effect_with(theme_state, move |theme_state| {
            let document = gloo_utils::document();
            let html_element = document.document_element().unwrap().dyn_into::<web_sys::HtmlElement>().unwrap();
            if theme_state.as_str() == "dark" {
                html_element.class_list().remove_1("light").unwrap();
                html_element.class_list().add_1("dark").unwrap();
            } else {
                html_element.class_list().remove_1("dark").unwrap();
                html_element.class_list().add_1("light").unwrap();
            }
            || ()
        });
    }

    {
        let books_controller = books_controller.clone();
        use_effect_with((), move |_| {
            books_controller.init_books();
            || ()
        });
    }

    /*
    let on_create_book = {
        let books_controller = books_controller.clone();
        Callback::from(move |title: String| {
            books_controller.create_book(placeholer);
        })
    };
    */

    let on_delete_book = {
        let books_controller = books_controller.clone();
        Callback::from(move |id: String| {
            books_controller.delete_book(id);
        })
    };

    let on_toggle_book = {
        let books_controller = books_controller.clone();
        Callback::from(move |id: String| {
            books_controller.toggle_read(id);
        })
    };

    let on_edit_book = {
        let books_controller = books_controller.clone();
        Callback::from(move |(id, book): (String, NewBook)| {
            books_controller.update_book(id, book);
        })
    };

    // Callback to toggle the theme
    let on_toggle_theme = {
        let theme_state = theme_state.clone();
        Callback::from(move |_: MouseEvent| {
            let new_theme = if theme_state.as_str() == "light" { "dark" } else { "light" };
            theme_state.set(new_theme.to_string());
            gloo_storage::LocalStorage::set("theme-preference", new_theme).unwrap();
        })
    };

    html!(
        <div class="container">
            <header class="header">
            <label></label>
            <h1>{ "OpenDumbLibrary v0.1" }</h1>
        // Theme toggle
            <button onclick={on_toggle_theme} class="toggle-button">{ "Tema" }</button>
            </header>
            <div>
                <BookList
                    books={books.books.clone()}
                    on_delete_book={on_delete_book}
                    on_toggle_book={on_toggle_book}
                    on_edit_book={on_edit_book}
                />
            </div>
        </div>
    )
}

fn main() {
    yew::Renderer::<App>::new().render();
}
