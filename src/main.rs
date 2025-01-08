use std::thread;
use eframe::{egui::CentralPanel, App, NativeOptions, egui::RichText, egui::Color32, egui::CursorIcon};
use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient};
use egui::debug_text;

//use egui::frame;

#[derive(Default)]
struct Rpc {
    state: State,
} 

impl App for Rpc{
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame ) {
        CentralPanel::default().show(ctx, |ui|{
            ui.label("Hello World");
            // if ui.button("Hello there").clicked() {
            //     println!("button clicked");
            // }
        
            let _response= client_buttons(ui, self);
            
            token_widget(ui, self);
            
            activity_type_widget(ui, self);

            //ui_counter(ui, );
        });
    }
}

fn client_buttons(ui: &mut egui::Ui, rpc: &mut Rpc) {

    ui.horizontal(|ui|{
        

        let start_button = match &rpc.state.status {
            Status::Connected { client } =>  ui.add_enabled(false, egui::Button::new(RichText::new("Start").color(Color32::DARK_BLUE))),
            Status::Disconnected =>  ui.add_enabled(true, egui::Button::new(RichText::new("Start").color(Color32::DARK_BLUE))),
            Status::Error { error: ErrorType::MissingToken } => ui.add_enabled(true, egui::Button::new(RichText::new("Start").color(Color32::DARK_BLUE))),
            _  => ui.add_enabled(false, egui::Button::new(RichText::new("Start").color(Color32::DARK_BLUE)))
        };

        let stop_button = match &rpc.state.status {
            Status::Connected { client } =>  ui.add_enabled(true, egui::Button::new(RichText::new("Stop").color(Color32::DARK_RED))),
            Status::Disconnected =>  ui.add_enabled(false, egui::Button::new(RichText::new("Stop").color(Color32::DARK_RED))),
            Status::Error { error: ErrorType::MissingToken } => ui.add_enabled(false, egui::Button::new(RichText::new("Stop").color(Color32::DARK_RED))),
            _  => ui.add_enabled(true, egui::Button::new(RichText::new("Stop").color(Color32::DARK_RED)))
        };


        if start_button.clicked() {
            //dbg!(&rpc.state.status);
            if rpc.state.token.trim() == "" {
                rpc.state.status = Status::Error { error: ErrorType::MissingToken };
                println!("missing token. setting status");
                return;
            } else{
                match &rpc.state.status {
                    Status::Error { error } => {
                        
                        rpc.state.status = Status::Disconnected
                        
                    },
                    _ => ()
                    
                }
                
            }
            //dbg!(&rpc.state.status);
            match &rpc.state.status {
                Status::Disconnected => {
                    rpc.state.status = Status::Connecting;
                    //dbg!(&rpc.state.status);
                    match start_client(rpc){
                        Ok(mut client) => {
                            
                            match client.connect() {
                                Ok(_) => (),
                                Err(_) => { let _ = client.close(); ()}
        
                            }
        
                            client.set_activity(activity::Activity::new()
                                        .state("foo")
                                        .details("bar")
                                        
                                    ).expect("wow it broke");
                            rpc.state.status = Status::Connected { client };
                            println!("Started client");
                            //rpc.state.status = Status::Connected { client }
                        },
                        
                        Err(_error) => rpc.state.status = Status::Error { error: ErrorType::Unkown }
                    }
                    

                },
                _ => ()
            }

            
        };
    
        if stop_button.clicked() {
            
            match &mut rpc.state.status {
                Status::Connected {   client} => {
                    
                    match client.close(){
                        Ok(_) => {
                            rpc.state.status = Status::Disconnected;
                            
                            println!("Client closed");
                        },
                        Err(err) => panic!("{err}") 
                    }
                },
                _ => ()//todo!("stop button")
            }
            
        }
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
    
fn activity_type_widget(ui: &mut egui::Ui, rpc: &mut Rpc) {
    // let mut e = vec![false, false, false, false];
   
    // ui.horizontal(|ui| {
    //     ui.checkbox(&mut e[0], "Playing");
    //     ui.checkbox(&mut e[1], "Watching");
    //     ui.checkbox(&mut e[2], "Listening");
    //     ui.checkbox(&mut e[3], "Competing");
    // });

    // match e[..] {
    //     [true, _] => rpc.state.activity_type = 0,
    //     [false, true, false, false] => rpc.state.activity_type = 1,
    //     _ => (),
    // }
    let mut a = Activities::Playing;
    let mut b = Activities::Watching;
    let mut selected = &mut rpc.state.activity_type;
    egui::ComboBox::from_label("Select one!")
    .selected_text(format!("{:?}", selected))
    .show_ui(ui, |ui| {
        ui.selectable_value(&mut selected, &mut a, "Playing");
        ui.selectable_value(&mut selected,  &mut b, "Watching");
        
    }
);
   
}


fn start_client(rpc: &mut Rpc) -> Result<DiscordIpcClient, Box<dyn std::error::Error>> {

    if &rpc.state.token == "".to_string().trim(){
        rpc.state.status = Status::Error { error : ErrorType::MissingToken };
    };
    let client = DiscordIpcClient::new(&rpc.state.token);
    match client {
        Ok (client) => Ok(client),
        Err(error) => Err(error)
    }
        
    
}

#[derive(Debug)]
struct State {
    token: String,
    status: Status,
    activity_type: Activities,
}

impl Default for State{
    fn default() -> Self {
        Self {
        token: String::new(),
        status: Status::Disconnected,
        activity_type : Activities::Watching,
        }
    }
}


impl State {
    pub fn _new() -> Self {
        Self {
            token: String::from("token here"),
            status: Status::Disconnected,
            activity_type: Activities::Watching,
        }
    }
}
#[derive(Debug, PartialEq)]
enum Activities{
    Playing, 
    Watching
}

#[derive(Debug)]
enum Status{
    Connected { client: DiscordIpcClient },
    Disconnected,
    Connecting,
    Disconnecting,
    Error { error: ErrorType},
}
#[derive(Debug)]
enum ErrorType {
    MissingToken,
    Unkown,
    //None
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
