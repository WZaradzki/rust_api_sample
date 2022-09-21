use yew::{html, Children, Component, Context, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct ListProps {
    #[prop_or_default]
    pub children: Children,
}

pub struct LayoutAdmin;

impl Component for LayoutAdmin {
    type Message = ();
    type Properties = ListProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="admin-layout">
                { for ctx.props().children.iter() }
            </div>
        }
    }
}