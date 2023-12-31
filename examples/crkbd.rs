use egui::Color32;
use egui_keyboard::press_time::PressTimesMap;
use egui_keyboard::KeyboardWidget;

fn main() -> Result<(), eframe::Error> {
    let keyboard_layout = serde_json::from_str(
        r##"[
[{"x":3},"E",{"x":7},"I"],
[{"y":-0.9,"x":2,"c":"#5eb1e7"},"W"],
[{"y":-1,"x":4,"c":"#cccccc"},"R",{"x":5},"U",{"x":1},"O"],
[{"y":-0.9,"x":5},"T",{"x":3},"Y"],
[{"y":-0.9},"Tab","Q",{"x":11},"P","Backspace"],
[{"y":-0.3,"x":3,"c":"#5eb1e7"},"D",{"x":7},"K"],
[{"y":-0.9,"x":2},"S",{"x":1,"c":"#cccccc","n":true},"F",{"x":5,"c":"#5eb1e7","n":true},"J",{"x":1},"L"],
[{"y":-0.9,"x":5,"c":"#cccccc"},"G",{"x":3,"c":"#5eb1e7"},"H"],
[{"y":-0.9,"c":"#cccccc"},"Esc",{"c":"#5eb1e7"},"A",{"x":11,"c":"#cccccc"},":\n;","\"\n'"],
[{"y":-0.3,"x":3},"C",{"x":7},"<\n,"],
[{"y":-0.9,"x":2},"X",{"x":1},"V",{"x":5},"M",{"x":1},">\n."],
[{"y":-0.9,"x":5},"B",{"x":3},"N"],
[{"y":-0.9},"Ctrl","Z",{"x":11},"?\n/","Enter"],
[{"y":-0.6,"x":6,"c":"#8ed7b0","a":6,"h":1.5},"Space",{"x":1,"a":6,"h":1.5},"Shift"],
[{"y":-0.6,"x":4,"c":"#cccccc"},"Super",{"x":5},"Alt"],
[{"y":-0.9,"x":5},"Lower"],
[{"y":-1,"x":9,"a":6},"Raise"]
]"##,
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
    press_map: PressTimesMap,
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

        egui::CentralPanel::default().frame(frame).show(ctx, |ui| {
            ui.input(|i| {
                i.events.iter().for_each(|e| {
                    if let egui::Event::Key { key, pressed, .. } = e {
                        if *pressed {
                            self.press_map.key_press(key);
                        }
                    }
                })
            });
            self.keyboard.draw(ui, Some(&self.press_map))
        });
    }
}
