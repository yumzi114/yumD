mod MyApp;
mod MyInfo;
mod window_frame;
use eframe::{egui};
use MyApp::NewsCardData;
use std::{thread,time::Duration,};



impl  eframe::App for MyApp::MyApp{
    
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array()
    }
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        ctx.request_repaint();
        window_frame::custom_window_frame(ctx, frame, "yum:D",|ui|{
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
    eframe::run_native("yumD",option,Box::new(|_cc|
        {
            let mut app = MyApp::MyApp::new(_cc);
            let field = app.field.clone();
            app.fech_news();
            // let config:MyApp::NewsConfig = confy::load("yumd", "yumdconfig").unwrap_or_default();
            // if let Ok(response) = api::NewsApi::new("kr", config.page_line, config.current_page).get_api(config.api_key){
            //     let articles = response.articles();
            //     for a in articles.iter(){
            //         let (first,last) = a.publishedAt.split_at(10);
            //         let news = NewsCardData{
            //             title: a.title.to_string(),
            //             url:a.url.to_string(),
            //             publishedAt:first.to_string()
            //         };
            //         app.articles.push(news);
            //     }
            // }
            // let field1 = app.articles.to_vec();
            thread::spawn(move || {
                loop {
                    thread::sleep(Duration::from_secs(1));
                    *field.lock().unwrap() += 1;
                }
            });
            Box::new(app)
        }
    ))
}
