use yew::prelude::*;

#[derive(Properties, PartialEq)]
struct HeaderButtonProps {
    id: usize,
    is_selected: bool,
    text: String,
}

#[function_component(HeaderButton)]
fn header_button(
    HeaderButtonProps {
        id,
        is_selected,
        text,
    }: &HeaderButtonProps,
) -> Html {
    html! {
        <a href="#">{text}</a>
    }
}

#[function_component(Header)]
pub fn header() -> Html {
    html! {
    <nav class="menu">
        <div class="inner">
            <HeaderButton id={0} is_selected={false} text={"About"} />
            <HeaderButton id={0} is_selected={false} text={"Works"} />
        </div>
    </nav>
    }
}
