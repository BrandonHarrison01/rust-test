use seed::prelude::*;
use seed::*;
use ulid::Ulid;
use std::collections::BTreeMap;
use std::mem;

#[derive(Default)]
struct Model {
    todos: BTreeMap<Ulid, Todo>,
    text_to_show: String,
}

struct Todo {
    id: Ulid,
    title: String
}

#[derive(Clone)]
enum Msg {
    ChangeText(String),
    CreateTodo,
    ClearAll,
    RemoveTodo(Ulid)
}

fn update(msg: Msg, model: &mut Model, _orders: &mut impl Orders<Msg>) {
    use Msg::*;

    match msg {
        ChangeText(new_text) => model.text_to_show = new_text,
        CreateTodo => {
            let title = model.text_to_show.trim();
            if not(title.is_empty()) {
                let id = Ulid::new();
                model.todos.insert(id, Todo {
                    id,
                    title: title.to_owned()
                });
                model.text_to_show.clear();
            }
        },
        ClearAll => {
            log!("clearing all...")
        }
        RemoveTodo(id) => {
            model.todos.remove(&id);
        }
    }
}

fn view(model: &Model) -> Node<Msg> {
    div![
        img![
            attrs!{At::Src => "LVLogo_small.png"}
        ],
        input![
            attrs! {
                At::Placeholder => "Enter some text...",
                At::Value => model.text_to_show
            },
            input_ev(Ev::Input, Msg::ChangeText),
        ],
        button![C!["save-btn"],
            ev(Ev::Click, |_| Msg::CreateTodo),
            "Save"
        ],
        button![C!["clear-btn"],
            ev(Ev::Click, |_| Msg::ClearAll),
            "Clear"
        ],
        ul![
            model.todos.values().map(|todo| {
                let id = todo.id;

                li![
                    label![&todo.title],
                    button![C!["delete-btn"],
                        ev(Ev::Click, move |_| Msg::RemoveTodo(id)),
                        "X"
                    ],
                    
                ]
            })
        ],
    ]
}

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model:: default()
}

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
