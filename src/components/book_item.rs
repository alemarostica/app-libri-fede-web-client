use yew::{classes, function_component, html, Callback, Properties, Html, use_state};
use web_sys::{console, wasm_bindgen::JsCast};

use crate::models::{Book, NewBook};

#[derive(Properties, PartialEq)]
pub struct BookItemProps {
    pub book: Book,
    pub on_delete_book: Callback<String>,
    pub on_toggle_book: Callback<String>,
    pub on_edit_book: Callback<(String, NewBook)>,
}

// TODO: l'edit del campo volume va modificato perché non riesco a mettere null adesso
// TODO: stile dell'edit
// TODO: fare in modo che quando si preme edit non scompaia la row list

#[function_component(BookItem)]
pub fn book(BookItemProps { book, on_delete_book, on_toggle_book , on_edit_book}: &BookItemProps) -> Html {
    // State to toggle between view and edit mode
    let is_editing = use_state(|| false);

    let form_data_state = use_state(|| NewBook {
        title: book.title.clone(),
        author: book.author.clone(),
        year: book.year,
        house: book.house.clone(),
        volume: book.volume,
        topic: book.topic.clone(),
        location: book.location.clone(),
        read: book.read,
    });

    let list_item_class = match book.read {
        true => Some("read"),
        false => Some("unread")
    };
    
    // Callback to open the form
    let on_edit_click = {
        let is_editing = is_editing.clone();
        Callback::from(move |_| {
            console::log_1(&"Edit button clicked".into());
            is_editing.set(true);
        })
    };

    let on_form_submit = {
        let on_edit_book = on_edit_book.clone();
        let book_id = book.id.clone();
        let form_data_state = form_data_state.clone();
        let is_editing = is_editing.clone();
        Callback::from(move |e: yew::events::SubmitEvent| {
            e.prevent_default();

            on_edit_book.emit((book_id.clone(), (*form_data_state).clone()));
            is_editing.set(false);
        })
    };

    let on_cancel_edit = {
        let is_editing = is_editing.clone();
        Callback::from(move |_| {
            is_editing.set(false);
        })
    };

    let on_delete_click = {
        let book = book.clone();
        let on_delete_book = on_delete_book.clone();
        move |_| on_delete_book.emit(book.id.clone())
    };

    let on_toggle = {
        let book = book.clone();
        let on_toggle_book = on_toggle_book.clone();
        move |_| on_toggle_book.emit(book.id.clone())
    };

    if *is_editing {
        html! {
            <li class="edit-form-row">
                <form onsubmit={on_form_submit}>
                <input
                type="text"
                placeholder="Title"
                value={form_data_state.title.clone()}
                oninput={
                    let form_data_state = form_data_state.clone();
                    Callback::from(move |e: yew::events::InputEvent| {
                        let mut data = (*form_data_state).clone();
                        let input = e.target().unwrap().dyn_into::<web_sys::HtmlInputElement>().unwrap();
                        data.title = input.value();
                        form_data_state.set(data); 
                    })
                }
            />
                <input
                type="text"
                placeholder="Author"
                value={form_data_state.author.clone()}
                oninput={
                    let form_data_state = form_data_state.clone();
                    Callback::from(move |e: yew::events::InputEvent| {
                        let mut data = (*form_data_state).clone();
                        let input = e.target().unwrap().dyn_into::<web_sys::HtmlInputElement>().unwrap();
                        data.author = input.value();
                        form_data_state.set(data);
                    })
                }
            />
                <input
                type="number"
                placeholder="Year"
                value={form_data_state.year.to_string()}
                oninput={
                    let form_data_state = form_data_state.clone();
                    Callback::from(move |e: yew::events::InputEvent| {
                        let mut data = (*form_data_state).clone();
                        let input = e.target().unwrap().dyn_into::<web_sys::HtmlInputElement>().unwrap();
                        if let Ok(year) = input.value().parse::<u16>() {
                            data.year = year;
                        }
                        form_data_state.set(data);
                    })
                }
            />
                <input
                type="text"
                placeholder="House"
                value={form_data_state.house.clone()}
                oninput={
                    let form_data_state = form_data_state.clone();
                    Callback::from(move |e: yew::events::InputEvent| {
                        let mut data = (*form_data_state).clone();
                        let input = e.target().unwrap().dyn_into::<web_sys::HtmlInputElement>().unwrap();
                        data.house = input.value();
                        form_data_state.set(data);
                    })
                }
            />
                <input
                type="number"
                placeholder="Volume"
                value={form_data_state.volume.map(|v| v.to_string()).unwrap_or_default()}
                oninput={
                    let form_data_state = form_data_state.clone();
                    Callback::from(move |e: yew::events::InputEvent| {
                        let mut data = (*form_data_state).clone();
                        let input = e.target().unwrap().dyn_into::<web_sys::HtmlInputElement>().unwrap();
                        let val_str = input.value();
                        data.volume = val_str.parse::<u16>().ok();
                        form_data_state.set(data);
                    })
                }
            />
                <input
                type="text"
                placeholder="Topic"
                value={form_data_state.topic.clone()}
                oninput={
                    let form_data_state = form_data_state.clone();
                    Callback::from(move |e: yew::events::InputEvent| {
                        let mut data = (*form_data_state).clone();
                        let input = e.target().unwrap().dyn_into::<web_sys::HtmlInputElement>().unwrap();
                        data.topic = input.value();
                        form_data_state.set(data);
                    })
                }
            />
                <input
                type="text"
                placeholder="Location"
                value={form_data_state.location.clone()}
                oninput={
                    let form_data_state = form_data_state.clone();
                    Callback::from(move |e: yew::events::InputEvent| {
                        let mut data = (*form_data_state).clone();
                        let input = e.target().unwrap().dyn_into::<web_sys::HtmlInputElement>().unwrap();
                        data.location = input.value();
                        form_data_state.set(data);
                    })
                }
            />
                <button type="submit" class="confirm-button">{ "Confirm" }</button>
                <button type="button" class="cancel-button" onclick={on_cancel_edit}>{ "Annulla" }</button>
                </form>
            </li>
        }
    } else {
        html! {
            <li class={classes!(list_item_class, "book-row")}>
                <input type="checkbox" checked={book.read} onchange={on_toggle} />
                <label>{&book.title}</label>
                <label>{&book.author}</label>
                <label>{&book.year.to_string()}</label>
                <label>{&book.house}</label>
                <label>{match &book.volume {Some(val) => val.to_string(), None => "Singolo".into() }}</label>
                <label>{&book.topic}</label>
                <label>{&book.location}</label>
                <button class="edit" onclick={on_edit_click}>{"Modifica"}</button>
                <button class="delete" onclick={on_delete_click}>{"Elimina"}</button>
            </li>
        }
    }
}
