mod pages;
mod components;
mod router;
use components::wrappers::app;

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    yew::start_app::<app::AppWrapper>();
}
