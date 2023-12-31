use egui::epaint::Hsva;
use egui::Color32;

pub fn sigmoid(times: u32) -> f32 {
    let times: f64 = times as f64 / 20.;
    let res = ((1. / (1. + (-times).exp()) - 0.5) * 2.) as f32;
    res
}

pub fn hue_from_rgb(r: u8, g: u8, b: u8) -> f32 {
   let r = r as f32 / 255.0;
   let g = g as f32 / 255.0;
   let b = b as f32 / 255.0;

   let max = r.max(g.max(b));
   let min = r.min(g.min(b));
   let delta = max - min;

   let hue = match max {
       _ if max == min => 0.0,
       _ if max == r => (((g - b) / delta) % 6.0) * 60.0,
       _ if max == g => (((b - r) / delta) + 2.0) * 60.0,
       _ => (((r - g) / delta) + 4.0) * 60.0,
   };

   let hue = if hue < 0.0 {
       hue + 360.0
   } else {
       hue
   };

   hue / 360.
}

/// Decides color of the key by hue (as a theme) and times key pressed
pub fn get_color(hue: f32, times: u32) -> Color32 {
    let k = (0.3 - 0.98) / 1.;
    let s = sigmoid(times);
    let v = k * s * s * s * s + 0.98;
    let srgb = Hsva::new(hue, s, v, 1.).to_srgb();
    Color32::from_rgb(srgb[0], srgb[1], srgb[2])
}

pub fn get_strike_color(color: Color32) -> Color32 {
    let mut hsv = Hsva::from_srgb([color.r(), color.g(), color.b()]);
    hsv.v -= 0.12;
    let srgb = hsv.to_srgb();
    Color32::from_rgb(srgb[0], srgb[1], srgb[2])
}

#[test]
fn test_get_color() {
    let times_vec: Vec<u32> = vec![0, 1, 10, 100, 1000];
    times_vec.iter().for_each(|&times| {
        let _color = get_color(210. / 360., times);
    });
}
