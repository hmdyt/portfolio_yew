use yew::prelude::*;

#[derive(Debug)]
struct HeaderButton {
    is_selected: bool,
}

#[derive(Properties, PartialEq, Debug)]
struct HeaderButtonProps {
    is_selected: bool,
    text: String,
}

enum Msg {
    Click,
}

impl Component for HeaderButton {
    type Message = Msg;
    type Properties = HeaderButtonProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            is_selected: ctx.props().is_selected,
        }
    }
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Click => {
                self.is_selected = !self.is_selected;
            }
        };
        true
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let click_callback = ctx.link().callback(|_| Msg::Click);
        html! {
            <a href="#" onclick={click_callback}>{format!("{} {}", ctx.props().text, self.is_selected)}</a>
        }
    }
}

#[function_component(Header)]
pub fn header() -> Html {
    html! {
    <nav class="menu">
        <div class="inner">
            <HeaderButton is_selected={false} text={"About"} />
            <HeaderButton is_selected={false} text={"Works"} />
        </div>
    </nav>
    }
}
