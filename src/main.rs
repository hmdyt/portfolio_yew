use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <h1>{ "Hello Worlda" }</h1>
            <p> {"test"} </p>
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
