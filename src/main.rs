use eframe::egui;

struct App {

}

impl Default for App {
    fn default() -> Self {
        Self{

        }
    }
}

impl eframe::App for App{
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame){

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
