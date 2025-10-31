extern crate eframe;
use eframe::egui;
struct App {
    output: i32,
}

impl Default for App {
    fn default() -> Self {
        Self{
            output: 0,
        }
    }
}

impl eframe::App for App{
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame){
        let Self { output } = self;
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(self.output.to_string());
        });
        
    }
}


fn main() -> Result<(), eframe::Error>{
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Calculator",
        options,
        Box::new(|_cc| Ok(Box::new(App::default())))
    )
}
