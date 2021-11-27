use yew::prelude::*;

pub struct App {
    // link: ComponentLink<Self>,
}

pub enum Msg {
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        App { 
            // link 
        }
    }

    fn view(&self) -> Html {
        html! {
        <div class="bg-red-100">
        {"Hello red world!"}
        </div>
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }
}
