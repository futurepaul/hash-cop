use druid::LocalizedString;
use druid::{AppLauncher, WindowDesc};

mod data;
mod delegate;
mod hash;
mod manifest_parser;
mod view;

fn main() {
    let main_window = WindowDesc::new(view::ui_builder()).title(LocalizedString::new("Hash Cop"));
    let app = AppLauncher::with_window(main_window);
    app.delegate(delegate::Delegate {})
        .launch(data::AppState::new())
        .expect("launch failed");
}
