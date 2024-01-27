use colors_transform::{Color, Rgb};
use rand::prelude::*;

const SATURATION_VARIATION: f32 = 20.0;
const BRIGHTNESS_VARIATION: f32 = 0.02;

pub fn vary_color(color: ggez::graphics::Color) -> ggez::graphics::Color {
    let rbg = Rgb::from(color.r, color.g, color.b)
        .saturate(thread_rng().gen_range(-SATURATION_VARIATION..SATURATION_VARIATION))
        .lighten(thread_rng().gen_range(-BRIGHTNESS_VARIATION..BRIGHTNESS_VARIATION));

    (rbg.get_red(), rbg.get_green(), rbg.get_blue()).into()
}
