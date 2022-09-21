// mod components;
// mod content;
// mod generator;
mod pages;
mod components;
mod router;
use components::layouts::public;

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    yew::start_app::<public::LayoutPublic>();
}
