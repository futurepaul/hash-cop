use druid::{Color, Data, Env, FontDescriptor, FontFamily, FontWeight, Key};

pub const TEXT_BASE: Key<f64> = Key::new("junction.text_base");
pub const TEXT_4XL: Key<f64> = Key::new("junction.text_4xl");
pub const TEXT_XS: Key<f64> = Key::new("junction.text_xs");

pub const MONO_FONT: Key<FontDescriptor> = Key::new("junction.mono_font");

const BASE_SIZE: f64 = 18.;
const XL_SIZE: f64 = 24.;

const GRAY_TEXT: Color = Color::rgb8(0xb8, 0xb8, 0xb8);
pub const COLOR_GRAY_TEXT: Key<Color> = Key::new("junction.red");

pub fn set_env<T: Data>() -> impl Fn(&mut Env, &T) + 'static {
    |env, _state| {
        env.set(COLOR_GRAY_TEXT, GRAY_TEXT);
        env.set(TEXT_4XL, XL_SIZE);
        env.set(TEXT_BASE, BASE_SIZE);
        env.set(TEXT_XS, 12.);
        // FIXME: this will panic without Inter!
        env.set(
            druid::theme::UI_FONT,
            FontDescriptor::new(FontFamily::new_unchecked("Inter")).with_size(BASE_SIZE),
        );
        env.set(
            druid::theme::UI_FONT_BOLD,
            FontDescriptor::new(FontFamily::new_unchecked("Inter"))
                .with_weight(FontWeight::BOLD)
                .with_size(XL_SIZE),
        );
        env.set(
            MONO_FONT,
            FontDescriptor::new(FontFamily::new_unchecked("IBM Plex Mono"))
                .with_weight(FontWeight::LIGHT)
                .with_size(BASE_SIZE),
        );
    }
}
