use yew::prelude::*;

pub struct Home;
impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="tile is-ancestor is-vertical">
                <div class="tile is-child hero">
                    <div class="hero-body container pb-0">
                        <h1 class="title is-1">{ "DOMIN GDANSK" }</h1>
                    </div>
                </div>
            </div>
        }
    }
}
// impl Home {
//     fn view_info_tiles(&self) -> Html {
//         html! {
//             <>
//                 <div class="tile is-child">
//                     <h1>{"DOMIN GDANSK"}</h1>
//                 </div>
//             </>
//         }
//     }
// }
