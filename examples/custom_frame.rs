use egui_custom_frame::CustomFrame;
use egui::{Pos2, Rect};

struct TestApp {
  tip: String,
  frame: CustomFrame,
}

impl TestApp {
  fn new(_cc: &eframe::CreationContext<'_>) -> Self {
    Self {
      tip: "This window is for testing a custom frame window.".to_string(),
      frame: CustomFrame::default().caption(
        Rect::from_min_max(
          Pos2::new(0.0, 0.0),
          Pos2::new(f32::MAX, f32::MAX) // Make the whole window draggable
        )
      ),
    }
  }
}

impl eframe::App for TestApp {  
  fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
    egui::Rgba::TRANSPARENT.to_array() // Make sure we don't paint anything behind the rounded corners and shadow
  }

  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    // Display contents in the custom frame
    self.frame.show(ctx, |ui| {
      ui.heading(&self.tip);
      if ui.button("Close").clicked() {
          ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
      }
    });
  }
}

fn main() -> Result<(), eframe::Error> {
  // Create test window
  let options = eframe::NativeOptions {
      viewport: egui::ViewportBuilder::default()
                  .with_inner_size([320.0, 120.0])
                  .with_decorations(false) // Custom frame
                  .with_transparent(true), // For rounded corners and shadow effects
      ..Default::default()
  };

  // Run test app
  eframe::run_native(
      "Custom Frame Test", // window title
      options, // viewport options
      Box::new(|cc| {
          // Create test app instance
          Ok(Box::new(TestApp::new(cc)))
      }),
  )
}