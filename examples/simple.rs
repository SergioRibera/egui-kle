use egui::Color32;
use egui_kle::KeyboardWidget;

fn main() -> Result<(), eframe::Error> {
    let keyboard_layout = serde_json::from_str(
        r#"[
[ { "a": 7 }, "Tab", "Q", "W", "E", "R", "T", "Y", "U", "I", "O", "P", "Back Space" ],
[ "Esc", "A", "S", "D", "F", "G", "H", "J", "K", "L", ";", "'" ],
[ "Shift", "Z", "X", "C", "V", "B", "N", "M", ",", ".", "/", "Return" ],
[ " ", "Ctrl", "Alt", "Super", ";", { "w": 2 }, " ", ";", ";", ";", ";", ";" ]
]"#,
    )
    .unwrap();
    let keyboard = KeyboardWidget::new(keyboard_layout);

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
    keyboard: KeyboardWidget,
}

impl App {
    pub fn new(keyboard: KeyboardWidget) -> Self {
        Self { keyboard }
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

        egui::CentralPanel::default()
            .frame(frame)
            .show(ctx, |ui| self.keyboard.draw(ui, None));
    }
}
