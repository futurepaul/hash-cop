use druid::widget::{Button, Either, Flex, Label, Spinner, TextBox};
use druid::Widget;
use druid::WidgetExt;

use crate::data::AppState;

pub fn ui_builder() -> impl Widget<AppState> {
    let choose_button = Button::new("Choose file to verify").on_click(AppState::choose_file);

    let hash_button = Button::new("Hash!")
        .on_click(AppState::start_hash)
        .padding(5.0);
    let hash_button_placeholder = Spinner::new().padding(5.0).center();

    let hash_result_label =
        Label::dynamic(|data: &AppState, _env: &_| format!("{}", data.hash_result))
            .padding(5.0)
            .center();

    let filename_label = Label::dynamic(|data: &AppState, _env: &_| format!("{}", data.filename))
        .padding(5.0)
        .center();

    let hash_button_or_spinner = Either::new(
        |data, _env| data.processing,
        hash_button_placeholder,
        hash_button,
    );

    let choose_manifest_button =
        Button::new("Choose manifest file").on_click(AppState::choose_manifest);

    let manifest_filename_label =
        Label::dynamic(|data: &AppState, _env: &_| format!("{}", data.manifest_filename))
            .padding(5.0)
            .center();

    let expected_label = Label::new("Expected hash");

    let textbox = TextBox::new().lens(AppState::expected_hash);

    let do_they_match_button = Button::new("Do they match?").on_click(AppState::compare_hash);

    let do_they_match_result_display =
        Label::dynamic(|data: &AppState, _env: &_| format!("{}", data.match_result))
            .padding(5.0)
            .center();

    let parse_manifest_button = Button::new("PARSE MANIFEST").on_click(AppState::parse_manifest);
    let auto_pick_expected_hash =
        Button::new("Auto pick expected hash").on_click(AppState::pick_expected_hash);

    Flex::column()
        .with_child(choose_button)
        .with_child(filename_label)
        .with_child(hash_button_or_spinner)
        .with_child(hash_result_label)
        .with_child(choose_manifest_button)
        .with_child(manifest_filename_label)
        .with_child(expected_label)
        .with_child(textbox)
        .with_child(do_they_match_button)
        .with_child(do_they_match_result_display)
        .with_child(parse_manifest_button)
        .with_child(auto_pick_expected_hash)
}
