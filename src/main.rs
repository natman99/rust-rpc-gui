use core::panic;
use std::str::FromStr;

use eframe::{egui::CentralPanel, App, NativeOptions, egui::RichText, egui::Color32, egui::CursorIcon};
use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient};
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
        
            let _response= client_buttons(ui, self);
            
            token_widget(ui, self);
            
            //ui_counter(ui, );
        });
    }
}

fn client_buttons(ui: &mut egui::Ui, rpc: &mut Rpc) {

    ui.horizontal(|ui|{
        if ui.button(RichText::new("Start").color(Color32::DARK_BLUE)).clicked() {
            start_client(rpc);
        };
    
        if ui.button(RichText::new("Stop").color(Color32::DARK_RED)).clicked() {
            todo!("u fool")
        };
    });

    
}

fn token_widget(ui: &mut egui::Ui, rpc: &mut Rpc) -> egui::Response{

    match rpc.state.status {
        Status::Disconnected => ui.text_edit_singleline(&mut rpc.state.token),
        Status::Error { error: ErrorType::MissingToken } => ui.text_edit_singleline(&mut rpc.state.token).highlight(),
        _ => ui.text_edit_singleline(&mut rpc.state.token)

    }

    //ui.text_edit_singleline(&mut rpc.state.token).on_hover_cursor(CursorIcon::NotAllowed);

}
    



fn start_client(rpc: &mut Rpc){
    match rpc.state.status {
        Status::Disconnected => {
            let client = DiscordIpcClient::new(&rpc.state.token);
            if &rpc.state.token == "".to_string().trim(){
                rpc.state.status = Status::Error { error : ErrorType::MissingToken };
            };
            let mut client: DiscordIpcClient = match client {
                Ok(client) => client,
                Err(_error) => {
                    rpc.state.status = Status::Error { error : ErrorType::MissingToken };
                    panic!("token brokey")
                }
            };
            
        },
        _ => todo!(),
        
    }
}


struct State {
    token: String,
    status: Status,
}

impl Default for State{
    fn default() -> Self {
        Self {
        token: String::new(),
        status: Status::Disconnected,
        }
    }
}

impl State {
    pub fn _new() -> Self {
        Self {
            token: String::from_str("token here").unwrap(),
            status: Status::Disconnected,
        }
    }
}

enum Status{
    Connected { client: DiscordIpcClient },
    Disconnected,
    Connecting,
    Error { error: ErrorType},
}

enum ErrorType {
    MissingToken,
    Unkown,
    None
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
