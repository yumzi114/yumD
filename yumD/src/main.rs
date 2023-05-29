mod MyApp;
mod MyInfo;
mod window_frame;
use eframe::{egui};
use MyApp::NewsCardData;
use std::{thread,time::Duration,};
use db_manager::*;

// impl eframe::Storage for MyApp::MyApp{
//     fn get_string(&self, key: &str) -> Option<String>{
//         match key {
//             "temp"=>return Some("asdas".to_string()),
//             _=>return Some("dd".to_string())
//         }
//     }
//     fn set_string(&mut self, key: &str, value: String){

//     }
//     fn flush(&mut self){}
// }
impl  eframe::App for MyApp::MyApp{
    // fn save(&mut self, _storage: &mut dyn eframe::Storage) {
        
    // }
    // fn save(&mut self, _storage: &mut dyn eframe::Storage) {
    //     _storage.set_string("test", "Test".to_string());
    //     let temp= _storage.get_string("test");
    //     println!("{:?}",temp);
    // }
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array()
    }
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // self.save(eframe::Storage);
        
        ctx.request_repaint();
        window_frame::custom_window_frame(ctx, frame, "yum:D",|ui|{
            self.render_sys(ui,ctx);
            ui.separator();
            ui.heading("Show Today News Headlines");
            egui::ScrollArea::vertical().show(ui, |ui| {
                self.news_menu(ui);
                ui.separator();
                ui.heading("Postgresql DB View And Migration");
                self.db_menu(ui);
                ui.separator();
                ui.heading("Stream Video View");
                self.stream_menu(ctx,ui);
                ui.separator();
                ui.heading("URL Video Down and View");
                self.video_menu(ui);
                ui.separator();
                });
        })
    }}

fn main()->Result<(),eframe::Error>{
    env_logger::init();
    let option = eframe::NativeOptions{
        decorated:false,
        transparent:true,
        resizable:true,
        min_window_size: Some(egui::vec2(400.1, 100.0)),
        initial_window_size: Some(egui::vec2(400.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native("yumD",option,Box::new(|_cc|
        {
            let mut app = MyApp::MyApp::new(_cc);
            let field = app.field.clone();
            let ttoken_time = app.ttoken_time.clone();
            // use std::net::ToSocketAddrs;
            app.fech_news();
            app.twitch_login();
            thread::spawn(move || {
                loop {
                    thread::sleep(Duration::from_secs(1));
                    *field.lock().unwrap() += 1;

                    
                    if *ttoken_time.lock().unwrap()!=0{
                        *ttoken_time.lock().unwrap()-=1;
                    }
                    //     println!("dd");
                    //     
                    // }else {
                    //     continue;
                    // }
                    // 
                    // if *ttoken_time.lock().unwrap()!=0{
                    //     println!("what??");
                        
                    // }
                }
            });
            Box::new(app)
        }
    ))
}
