use std::sync::{Arc, Mutex};

use egui::Color32;
use egui_keyboard::press_time::PressTimesMap;
use egui_keyboard::KeyboardWidget;

fn main() -> Result<(), eframe::Error> {
    let keyboard_layout = serde_json::from_str(
        r#"[
[{"x":3},"E",{"x":7},"I"],
[{"y":-0.9,"x":2},"W"],
[{"y":-1,"x":4},"R",{"x":5},"U",{"x":1},"O"],
[{"y":-0.9,"x":5},"T",{"x":3},"Y"],
[{"y":-0.9},"Tab","Q",{"x":11},"P","Bksp"],
[{"y":-0.3,"x":3},"D",{"x":7},"K"],
[{"y":-0.9,"x":2},"S",{"x":1,"n":true},"F",{"x":5,"n":true},"J",{"x":1},"L"],
[{"y":-0.9,"x":5},"G",{"x":3},"H"],
[{"y":-0.9},"Esc","A",{"x":11},":\n;","\"\n'"],
[{"y":-0.3,"x":3},"C",{"x":7},"<\n,"],
[{"y":-0.9,"x":2},"X",{"x":1},"V",{"x":5},"M",{"x":1},">\n."],
[{"y":-0.9,"x":5},"B",{"x":3},"N"],
[{"y":-0.9},"Ctrl","Z",{"x":11},"?\n/","Enter"],
[{"y":-0.2,"x":4},"Super",{"x":5},"Alt"],
[{"x":5,"y":-0.9},"Lower"],
[{"y":-1.5,"x":6,"a":7,"h":1.5}," "],
[{"y":-0.5,"x":9},"Raise"],
[{"y":-1.5,"x":8,"a":4,"h":1.5},"Shift"]
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
