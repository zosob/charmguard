use eframe::{egui, egui::Visuals};
use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;

//-----RASA REST Client-----
use charmguard::rasa_client;
use charmguard::rasa_client::send_message_to_rasa;

use charmguard::timer;
use charmguard::session::load_total_charms;



//-----GUI State-----
#[derive(Default)]
struct GuiState{
    user: String,
    duration: u32,
    running: bool,
    log: String,
    //-----chat-----
    user_input: String,
    chat_history: Vec<String>,
}

pub fn run_gui() -> eframe::Result<()>{
    let mut native_options = eframe::NativeOptions::default();
    
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(egui::vec2(520.0, 480.0)),
        ..Default::default()
    };
    eframe::run_native(
        "CharmGuard GUI",
        native_options,
        Box::new(|_| Box::new(CharmGuardApp::default()))
    )
}

struct CharmGuardApp{
    state: Arc<Mutex<GuiState>>,
    rt: Runtime,
}

impl Default for CharmGuardApp{
    fn default() -> Self {
        Self{
            state: Arc::new(Mutex::new(GuiState{
                user: "Anonymous".into(),
                duration: 25,
                running: false,
                log: String::new(),
                user_input: String::new(),
                chat_history: Vec::new(),
            })),
            rt: Runtime::new().unwrap(),
        }
    }
}

impl eframe::App for CharmGuardApp{
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame){
        ctx.set_visuals(Visuals::light());
        let mut state = self.state.lock().unwrap();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("CHARMGUARD");
            ui.separator();

            //-----Session Controls-----
            ui.collapsing("Focus Session", |ui| {
                ui.horizontal(|ui| {
                    ui.label("User:");
                    ui.text_edit_singleline(&mut state.user);
                });
                ui.horizontal(|ui| {
                    ui.label("Duration (min):");
                    ui.add(egui::DragValue::new(&mut state.duration).clamp_range(5..=180));
                });
                if !state.running {
                    if ui.button("Start Session").clicked(){
                        let minutes = state.duration;
                        let state_clone = self.state.clone();
                        state.running = true;
                        state.log.push_str(&format!("Started {minutes}-minute session\n"));
                        std::thread::spawn(move || {
                            timer::start(minutes);
                            let mut st = state_clone.lock().unwrap();
                            st.running = false;
                            st.log.push_str("Session complete!\n");
                        });
                    }
                } else {
                    ui.label("Session Running!!");
                }
                if ui.button("Show Total Charms").clicked(){
                    let user = state.user.clone();
                    let total = load_total_charms(&state.user);
                    state.log.push_str(&format!("Total charms for {}: {}\n", user, total));
                }
            });

            ui.add_space(10.0);
            ui.separator();

            //-----Chatbot Panel-----
            ui.collapsing("Rasa Chat", |ui| {
                egui::ScrollArea::vertical().max_height(150.0).show(ui, |ui|{
                    for line in &state.chat_history{
                        ui.label(line);
                    }
                });

                ui.horizontal(|ui| {
                    let send_clicked = ui.button("Send").clicked();
                    let enter_pressed = ui.text_edit_singleline(&mut state.user_input).lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));

                    if (send_clicked || enter_pressed) && !state.user_input.trim().is_empty(){
                        let msg = state.user_input.trim().to_string();
                        let sender = state.user.clone();
                        
                        state.chat_history.push(format!("😃: {}", msg));
                        state.user_input.clear();

                        //-----async call to Rasa via helper-----
                        let history = &mut state.chat_history;
                        let fut = async move {
                            match send_message_to_rasa(&sender, &msg).await {
                                Ok(replies) => {
                                    for rep in replies {
                                        history.push(format!("🤖: {}", rep.text));
                                    }
                                }
                                Err(e) => history.push(format!("Rasa error: {}", e)),
                            }
                        };
                        self.rt.block_on(fut);
                    }
                });
            });
            ui.add_space(10.0);
            //-----Log Panel-----
            ui.group(|ui| {
                ui.label("Logs:");
                egui::ScrollArea::vertical().max_height(120.0).show(ui, |ui| {
                    ui.code(&state.log);
                });
            });
        });
    }
}