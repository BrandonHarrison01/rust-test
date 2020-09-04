use seed::prelude::*;
use seed::*;

#[derive(Default)]
struct Model {
    text_to_show: String,
}

#[derive(Clone)]
enum Msg {
    ChangeText(String),
}

fn update(msg: Msg, model: &mut Model, _orders: &mut impl Orders<Msg>) {
    use Msg::*;

    match msg {
        ChangeText(new_text) => model.text_to_show = new_text,
    }
}

fn view(model: &Model) -> Node<Msg> {
    div![
        img![
            attrs!{At::Src => "LVLogo_small.png"}
        ],
        input![
            attrs! {
                At::Placeholder => "Enter some text..."
            },
            input_ev(Ev::Input, Msg::ChangeText),
        ],
        button![
            ev(Ev::Click, |_| log!("clicked")),
            "Save"
        ],
        div![&model.text_to_show],
        p!["test"]
    ]
}

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model::default()
}

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
