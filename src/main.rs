use portfolio_yew::components::header::Header;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <Header/>
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
