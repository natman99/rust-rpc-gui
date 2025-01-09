use eframe::{egui::CentralPanel, App, NativeOptions, egui::RichText, egui::Color32};
use discord_rich_presence::{activity::{self, ActivityType, Assets}, DiscordIpc, DiscordIpcClient};
use std::{sync::mpsc::{self, Receiver, SendError, Sender, TryRecvError}, thread};

//use egui::frame;

#[derive(Default)]
struct Rpc {
    state: State,
} 
#[derive(Debug)]
enum ThreadHandler{
    Inactive,
    Active {tx: Sender<DiscordIpcClient>, rx: Receiver<DiscordIpcClient> }
}

#[derive(Debug)]
struct State {
    token: String,
    status: Status,
    activity_type: Activities,
    state: String,
    details: String,
    large_img: String,
    large_img_text: String,
    small_img: String,
    small_img_text: String,
    thread_handler: ThreadHandler,
}

impl Default for State{
    fn default() -> Self {
        Self {
        token: String::new(),
        status: Status::Disconnected,
        activity_type : Activities::Playing,
        state: String::new(),
        details: String::new(),
        large_img: String::new(),
        large_img_text: String::new(),
        small_img: String::new(),
        small_img_text: String::new(),
        thread_handler: ThreadHandler::Inactive,
        }
    }
}

#[derive(Debug, PartialEq)]
enum Activities{
    Playing, 
    Listening,
    Watching,
    Competing

}

#[derive(Debug)]
enum Status{
    Connected { client: DiscordIpcClient },
    Disconnected,
    Connecting,
    //Disconnecting,
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
        Self::default()
    }
}


impl App for Rpc{
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame ) {
        CentralPanel::default().show(ctx, |ui|{
            let _response= client_buttons(ui, self);

            match self.state.thread_handler {
                ThreadHandler::Inactive => {
                    let (tx, rx) = mpsc::channel();
                    self.state.thread_handler = ThreadHandler::Active { tx, rx }
                },
                _ => (),
            }

            let maybe_client = match self.state.status{
                Status::Connecting => {
                    match &mut self.state.thread_handler {
                        ThreadHandler::Active { tx: _, rx } => rx.try_recv(),
                        _ => Err(TryRecvError::Empty),
                    }
                },
                _ => Err(TryRecvError::Empty)
            };
            
            match maybe_client {
                Ok(mut client) => self.state.status = {
                    println!("Recived content {:?}", &client);
                    set_client_status(&mut client, self);
                    Status::Connected { client }
                },
                Err(_err ) => (),
            }
            
            token_widget(ui, self);
            
            state_widget(ui, self);

            details_widget(ui, self);
            
            large_img_widget(ui, self);

            small_img_widget(ui, self);
            
            activity_type_widget(ui, self);

            //ui_counter(ui, );
        });
    }
}

fn client_buttons(ui: &mut egui::Ui, rpc: &mut Rpc) {

    ui.horizontal(|ui|{
        

        let start_button = match &rpc.state.status {
            Status::Connected { client: _ } =>  ui.add_enabled(false, egui::Button::new(RichText::new("Start").color(Color32::BLUE))),
            Status::Disconnected =>  ui.add_enabled(true, egui::Button::new(RichText::new("Start").color(Color32::BLUE))),
            Status::Error { error: ErrorType::MissingToken } => ui.add_enabled(true, egui::Button::new(RichText::new("Start").color(Color32::BLUE))),
            _  => ui.add_enabled(false, egui::Button::new(RichText::new("Start").color(Color32::BLUE)))
        };

        let stop_button = match &rpc.state.status {
            Status::Connected { client: _ } =>  ui.add_enabled(true, egui::Button::new(RichText::new("Stop").color(Color32::RED))),
            Status::Disconnected =>  ui.add_enabled(false, egui::Button::new(RichText::new("Stop").color(Color32::RED))),
            Status::Error { error: ErrorType::MissingToken } => ui.add_enabled(false, egui::Button::new(RichText::new("Stop").color(Color32::RED))),
            _  => ui.add_enabled(true, egui::Button::new(RichText::new("Stop").color(Color32::RED)))
        };

        let color: Color32 = match rpc.state.status {
            Status::Connected { client: _  } => Color32::GREEN,
            Status::Error { error: _ } => Color32::RED,
            _ => Color32::GOLD

        };

        let text = match rpc.state.status {
            Status::Connected { client: _  } => "Connected",
            Status::Connecting => "Connecting",
            Status::Disconnected => "Disconnected",
            Status::Error { error: ErrorType::MissingToken } => "Missing token",
            Status::Error { error: _ } => "Error",
            //_ => "Unkown"

        };

        ui.label(RichText::new( format!("{text}")).color(color) );


        if start_button.clicked() {
            // check token validity
            if rpc.state.token.trim() == "" {
                rpc.state.status = Status::Error { error: ErrorType::MissingToken };
                println!("missing token. setting status");
                return;
            } else{
                match &rpc.state.status {
                    Status::Error { error: _ } => {
                        
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
                    let token = rpc.state.token.clone();
                    match &rpc.state.thread_handler {
                        ThreadHandler::Active { tx, rx: _ } => {
                            let tx1 = tx.clone();
                            thread::spawn(move ||{
                                let e = start_client(token);
                                println!("Client created in thread!");
                                let _ = match e {
                                    Some(client ) => {
                                        let _ = tx1.send(client);
                                        ()
                                    },
                                    None => (),
                                };
                            });
                        },
                        _ => ()
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

fn start_client(token: String) -> Option<DiscordIpcClient> {

    
    let client = DiscordIpcClient::new(&token);

    let client = match client {
        Ok(mut client) => {
            client.connect();
            Ok(client)
        },
        Err(err ) =>  Err(err)
    };

    // match client.connect() {
    //     Ok() => (),
    //     Err(err: _) => return None
    // }
    match client{
        Ok(client) => Some(client),
        _ => None
    }
    
    
    
    
}


fn set_client_status(client: &mut DiscordIpcClient, rpc: &mut Rpc ){

    let chosen = match &rpc.state.activity_type {
        Activities::Playing => ActivityType::Playing,
        Activities::Listening => ActivityType::Listening,
        Activities::Watching => ActivityType::Watching,
        Activities::Competing => ActivityType::Competing
    };
    
    let activity = activity::Activity::new();

    let assets = Assets::new();
    let mut assets_filled = false;
    
    let activity = match &rpc.state.details.trim().is_empty() {
        false => activity.details(&rpc.state.details),
        true => activity,
    };

    let activity = match &rpc.state.state.trim().is_empty() {
        false => activity.state(&rpc.state.state),
        true => activity,
    };

    let assets = match &rpc.state.large_img.trim().is_empty() {
        false => {
            assets_filled = true;
            assets.large_image(&rpc.state.large_img)}
            ,
        true => assets,
    };

    let assets = match &rpc.state.large_img.trim().is_empty() {
        false => {
            assets_filled = true;
            assets.large_image(&rpc.state.large_img)}
            ,
        true => assets,
    };

    
    let activity = activity.activity_type(chosen);
    
    let activity = activity.assets(assets);

    client.set_activity(activity).expect("wow it broke");
    
    println!("Started client");
    //rpc.state.status = Status::Connected { client }
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
    egui::ComboBox::from_label("Activity Type")
    .selected_text(format!("{:?}", rpc.state.activity_type))
    .show_ui(ui, |ui| {
        ui.selectable_value(&mut rpc.state.activity_type, Activities::Playing, "Playing");
        ui.selectable_value(&mut rpc.state.activity_type,  Activities::Listening, "Listening");
        ui.selectable_value(&mut rpc.state.activity_type,  Activities::Watching, "Watching");
        ui.selectable_value(&mut rpc.state.activity_type,  Activities::Competing, "Competing");
        
    }
);
   
}

fn state_widget(ui: &mut egui::Ui, rpc: &mut Rpc) {

    ui.horizontal(|ui|{
        ui.label("State");
        ui.text_edit_singleline(&mut rpc.state.state)

    });
}

fn details_widget(ui: &mut egui::Ui, rpc: &mut Rpc) {

    ui.horizontal(|ui|{
        ui.label("Details");
        ui.text_edit_singleline(&mut rpc.state.details)

    });
}

fn large_img_widget(ui: &mut egui::Ui, rpc: &mut Rpc) {

    ui.horizontal(|ui|{
        ui.label("Large image");
        ui.text_edit_singleline(&mut rpc.state.large_img);
        ui.label("Large image key");
        ui.text_edit_singleline(&mut rpc.state.large_img_text);

    });
}

fn small_img_widget(ui: &mut egui::Ui, rpc: &mut Rpc) {

    ui.horizontal(|ui|{
        ui.label("Small image");
        ui.text_edit_singleline(&mut rpc.state.small_img);
        ui.label("Small image key");
        ui.text_edit_singleline(&mut rpc.state.small_img_text);

    });
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
