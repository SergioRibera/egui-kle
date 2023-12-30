use std::sync::{Arc, Mutex};

use egui::Color32;
use egui_keyboard::press_time::PressTimesMap;
use egui_keyboard::KeyboardWidget;

fn main() -> Result<(), eframe::Error> {
    let keyboard_layout = serde_json::from_str(
        r#"[
[{"r":10,"rx":1,"y":-0.1,"x":2},"E"],
[{"y":-0.65,"x":1},"W",{"x":1},"R"],
[{"y":-0.75},"Q"],
[{"y":-0.9,"x":4},"T"],
[{"y":-0.7,"x":2},"D"],
[{"y":-0.65,"x":1},"S",{"x":1},"F"],
[{"y":-0.75},"A"],
[{"y":-0.9,"x":4},"G"],
[{"y":-0.7,"x":2},"C"],
[{"y":-0.65,"x":1},"x",{"x":1},"V"],
[{"y":-0.75},"Z"],
[{"y":-0.9,"x":4},"B"],
[{"y":-0.75,"x":5,"h":1.5},"Ctrl"],
[{"y":-0.95,"x":2},"super"],
[{"y":-0.65,"x":1},"Tab",{"x":1},"Shift"],
[{"y":-0.75},"Esc"],
[{"y":-0.9,"x":4},"Bksp"],
[{"r":-10,"rx":7,"ry":0.965,"y":-0.2,"x":2},"I"],
[{"y":-0.65,"x":1},"U",{"x":1},"O"],
[{"y":-0.75,"x":4},"P"],
[{"y":-0.9},"y"],
[{"y":-0.7,"x":2},"K"],
[{"y":-0.65,"x":1},"J",{"x":1},"L"],
[{"y":-0.75,"x":4},":\n;"],
[{"y":-0.9},"H"],
[{"y":-0.7,"x":2},"<\n,"],
[{"y":-0.65,"x":1},"M",{"x":1},">\n."],
[{"y":-0.75,"x":4},"?\n/"],
[{"y":-0.9},"N"],
[{"y":-0.75,"x":-1,"h":1.5},"Alt"],
[{"y":-0.95,"x":2},"_\n-"],
[{"y":-0.65,"x":1},"fn",{"x":1},"\"\n'"],
[{"y":-0.75,"x":4},"Enter"],
[{"y":-0.9},"Space"]
]"#
    )
    .unwrap();
    let keyboard = KeyboardWidget::new(220. / 360., keyboard_layout);

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
            .show(ctx, |ui| {
                if ui.input(|i| i.key_down(egui::Key::Tab)) {
                    press_map.key_press(egui_keyboard::Keycode::Tab);
                }
                self.keyboard.draw(&mut press_map, ui)
            });
    }
}
