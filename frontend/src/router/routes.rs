use yew::prelude::*;
use yew_router::prelude::*;
use crate::pages::home::Home;

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    // #[at("/posts/:id")]
    // Post { id: u64 },
    // #[at("/posts")]
    // Posts,
    // #[at("/authors/:id")]
    // Author { id: u64 },
    // #[at("/authors")]
    // Authors,
    #[at("/")]
    Home,
    // #[not_found]
    // #[at("/404")]
    // NotFound,
}
pub fn switch(routes: &Route) -> Html {
    match routes.clone() {
        // Route::Post { id } => {
        //     html! { <Post seed={id} /> }
        // }
        // Route::Posts => {
        //     html! { <PostList /> }
        // }
        // Route::Author { id } => {
        //     html! { <Author seed={id} /> }
        // }
        // Route::Authors => {
        //     html! { <AuthorList /> }
        // }
        Route::Home => {
            html! { <Home /> }
        } // Route::NotFound => {
          //     html! { <PageNotFound /> }
          // }
    }
}