use druid::WidgetExt;
use druid::{
    theme::UI_FONT_BOLD,
    widget::{Button, Either, Flex, Label, LabelText, MainAxisAlignment, Spinner, TextBox},
    Color, Data, Env, Key, WidgetPod,
};
use druid::{widget::CrossAxisAlignment, Widget};

use crate::{
    data::AppState,
    theme::{COLOR_GRAY_TEXT, MONO_FONT},
};

pub fn labeled_value<T: Data>(
    label: &str,
    value: impl Into<LabelText<T>>,
    is_busy: Option<fn(&T, &Env) -> bool>,
    color: Option<fn(&mut Env, &T)>,
) -> impl Widget<T> {
    let value_label = if let Some(color_fn) = color {
        Label::new(value).with_font(MONO_FONT).env_scope(color_fn)
    } else {
        Label::new(value).with_font(MONO_FONT).env_scope(|_, _| {})
    };

    let spinner = Spinner::new().padding(5.0).center();

    // TODO: wish this wasn't so picky about same return types for if else branches
    let value_or_spinner = if let Some(busy_fn) = is_busy {
        Either::new(busy_fn, spinner, value_label)
    } else {
        Either::new(|_, _| false, spinner, value_label)
    };

    Flex::row()
        .with_child(
            Label::new(label)
                .with_font(MONO_FONT)
                .with_text_color(COLOR_GRAY_TEXT),
        )
        .with_child(value_or_spinner)
}

pub fn software_picker() -> impl Widget<AppState> {
    let header = Label::new("Software").with_font(UI_FONT_BOLD);

    let filename_label = |data: &AppState, _env: &_| {
        if data.filename.is_empty() {
            "...".into()
        } else {
            format!("{}", data.filename)
        }
    };

    let filename = labeled_value("filename: ", filename_label, None, None);

    let hash_result_label = |data: &AppState, _env: &_| {
        if data.hash_result.is_empty() {
            "...".into()
        } else {
            format!("{}", data.hash_result)
        }
    };

    let hash_result = labeled_value(
        "hash: ",
        hash_result_label,
        Some(|data: &AppState, _env| data.busy_hashing),
        None,
    );

    let pick_button = Button::new("Pick").on_click(AppState::choose_file);

    let left = Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(header)
        .with_spacer(10.)
        .with_child(filename)
        .with_child(hash_result);

    // TODO: if I add a flex spacer to this column it blows up to infinity
    let right = Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::End)
        .with_child(pick_button);

    Flex::row()
        .with_child(left)
        .with_flex_spacer(1.)
        .with_child(right)
        .padding(10.)
        .background(Color::BLACK)
        .rounded(8.)
}

pub fn manifest_picker() -> impl Widget<AppState> {
    let header = Label::new("Manifest").with_font(UI_FONT_BOLD);

    let filename_label = |data: &AppState, _env: &_| {
        if data.manifest_filename.is_empty() {
            "...".into()
        } else {
            format!("{}", data.manifest_filename)
        }
    };

    let software_hash = |data: &AppState, _env: &_| {
        if data.expected_hash.is_empty() {
            "...".into()
        } else {
            format!("{}", data.expected_hash)
        }
    };

    let filename = labeled_value("filename: ", filename_label, None, None);
    let software = labeled_value(
        "lists software: ",
        AppState::lists_software_label,
        None,
        None,
    );

    let green_hash = |env: &mut Env, data: &AppState| {
        if data.hash_result == data.expected_hash {
            env.set(druid::theme::LABEL_COLOR, Color::rgb8(0x00, 0xc9, 0x44));
        }
    };

    // TODO: if you load the manifest first this doesn't work
    let software_hash = labeled_value("hash: ", software_hash, None, Some(green_hash));

    let pick_button = Button::new("Pick").on_click(AppState::choose_manifest);

    let left = Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(header)
        .with_spacer(10.)
        .with_child(filename)
        .with_child(software)
        .with_child(software_hash);

    // TODO: if I add a flex spacer to this column it blows up to infinity
    let right = Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::End)
        .with_child(pick_button);

    Flex::row()
        .with_child(left)
        .with_flex_spacer(1.)
        .with_child(right)
        .padding(10.)
        .background(Color::BLACK)
        .rounded(8.)
}

pub fn ui_builder() -> impl Widget<AppState> {
    Flex::column()
        .with_child(software_picker())
        .with_spacer(10.)
        .with_child(manifest_picker())
        .with_flex_spacer(1.)
        .padding(20.)
}
