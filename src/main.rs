use yew::prelude::*;

mod components;
use components::field::Field;
use components::player::Player;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <body>
            <div id="game">
                <Player id="player1">{0}</Player>
                <Field id="field">
                    <div id="top" />
                    <div id="bottom" />
                </Field>
                <Player id="player2">{0}</Player>
            </div>
        </body>
    }
}

fn main() {
    yew::start_app::<App>();
}
