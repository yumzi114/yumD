mod MyApp;
mod MyInfo;
mod window_frame;
use eframe::{egui};
use MyApp::NewsCardData;



impl  eframe::App for MyApp::MyApp{
    
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array()
    }
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        window_frame::custom_window_frame(ctx, frame, "yumD",|ui|{
            self.render_sys(ui,ctx);
            ui.separator();
            ui.heading("Show Today News Headlines");
            self.news_menu(ui);
            ui.separator();
            ui.heading("Postgresql DB View And Migration");
            self.db_menu(ui);
            ui.separator();
            ui.heading("Stream Video View");
            self.stream_menu(ui);
            ui.separator();
            ui.heading("URL Video Down and View");
            self.video_menu(ui);
            ui.separator();
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
    eframe::run_native("yumD",option,Box::new(|_cc|Box::new(MyApp::MyApp::new(_cc))))
    
}
