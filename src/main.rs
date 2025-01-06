use eframe::{egui::CentralPanel, App, NativeOptions};
//use egui::frame;

#[derive(Default)]
struct Rpc {
    state: State,
} 

impl App for Rpc{
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame ) {
        CentralPanel::default().show(ctx, |ui|{
            ui.label("Hello World");
            if ui.button("Hello there").clicked() {
                println!("button clicked");
            }
            
            
            ui.text_edit_singleline(&mut self.state.t);
            
            //ui_counter(ui, );
        });
    }
}

// fn ui_counter(ui: &mut egui::Ui, counter: &mut i32) {
//     // Put the buttons and label on the same row:
//     ui.horizontal(|ui| {
//         if ui.button("-").clicked() {
//             *counter -= 1;
//         }
//         ui.label(counter.to_string());
//         if ui.button("+").clicked() {
//             *counter += 1;
//         }
//     });
// }

struct State {
    t: String,
    status: Status
}

impl Default for State{
    fn default() -> Self {
        Self {
        t: String::new(),
        status: Status::Disconnected
        }
    }
}

impl State {
    pub fn _new() -> Self {
        Self {
            t: String::new(),
            status: Status::Disconnected,
        }
    }
}

enum Status{
    Connected,
    Disconnected,
    Connecting,
    Error
}


impl Rpc {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        //cc.storage.expect("idk brokey");
        Self::default()
    }
}

fn main() -> eframe::Result {
    let win_option = NativeOptions::default();
    // run_simple_native("Hello World", win_option, move |ctx, frame|{
    //     CentralPanel::default().show(ctx, |ui|{
    //         ui.label("Hello world")
    //     });
    
    // })
    eframe::run_native("My egui App", win_option, Box::new(|cc| Ok(Box::new(Rpc::new(cc)))))
}
