use std::sync::{Arc, Mutex};

use egui::Color32;
use egui_keyboard::press_time::PressTimesMap;
use egui_keyboard::KeyboardWidget;

fn main() -> Result<(), eframe::Error> {
    let keyboard_layout = serde_json::from_str(
        r#"[
  [ { "a": 7 }, "Tab", "Q", "W", "E", "R", "T", "Y", "U", "I", "O", "P", "Back Space" ],
  [ "Esc", "A", "S", "D", "F", "G", "H", "J", "K", "L", ";", "'" ],
  [ "Shift", "Z", "X", "C", "V", "B", "N", "M", ",", ".", "/", "Return" ],
  [ " ", "Ctrl", "Alt", "Super", ";", { "w": 2 }, " ", ";", ";", ";", ";", ";" ]
]"#,
    ).unwrap();
    let keyboard = KeyboardWidget::new(360. / 260., keyboard_layout);

    let options = eframe::NativeOptions {
        ..Default::default()
    };

    eframe::run_native(
        "Keyboard Heatmap",
        options,
        Box::new(|_| Box::new(App::new(keyboard))),
    )
}

struct App {
    press_map: Arc<Mutex<PressTimesMap>>,
    keyboard: KeyboardWidget,
}

impl App {
    pub fn new(keyboard: KeyboardWidget) -> Self {
        Self {
            keyboard,
            press_map: Default::default(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let frame = egui::Frame::none()
            .inner_margin(egui::style::Margin {
                left: 30.,
                right: 30.,
                top: 30.,
                bottom: 30.,
            })
            .fill(Color32::WHITE);
        let mut press_map = self.press_map.lock().unwrap();

        egui::CentralPanel::default()
            .frame(frame)
            .show(ctx, |ui| self.keyboard.draw(&mut press_map, ui));
    }
}
