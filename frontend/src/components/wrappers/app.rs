use crate::components::layouts::public::LayoutPublic;
use crate::components::layouts::admin::LayoutAdmin;
use crate::router::routes::{switch, Route};
use yew::prelude::*;
use yew_router::prelude::*;

pub struct AppWrapper;
impl Component for AppWrapper {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let admin = "public";
        if admin == "public" {
            html! {
                <BrowserRouter>
                    <LayoutPublic>
                         <main>
                            <Switch<Route> render={Switch::render(switch)} />
                        </main>
                    </LayoutPublic>
                </BrowserRouter>
            } 
        } else {
            html! {
                <BrowserRouter>
                    <LayoutAdmin>
                         <main>
                            <Switch<Route> render={Switch::render(switch)} />
                        </main>
                    </LayoutAdmin>
                </BrowserRouter>
            } 
        }
       
    }
}
