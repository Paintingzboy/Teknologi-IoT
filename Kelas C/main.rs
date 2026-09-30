use eframe::egui::{self, Color32, RichText, ScrollArea, Vec2};

#[derive(Debug, Clone)]
struct CoffeeData {
    device_id: String,
    predicted_class: String,
    confidence: f32,
    temperature: f32,
    humidity: f32,
    timestamp: String,
}

struct CoffeeDashboard {
    history: Vec<CoffeeData>,
    last_update: f64,
    counter: usize,
}

impl CoffeeDashboard {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut visuals = egui::Visuals::dark();
        visuals.panel_fill = Color32::from_rgb(20, 20, 24);
        cc.egui_ctx.set_visuals(visuals);

        Self {
            history: Vec::new(),
            last_update: 0.0,
            counter: 0,
        }
    }
}

impl eframe::App for CoffeeDashboard {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Sistem waktu yang kompatibel dengan browser web
        let now = ctx.input(|i| i.time);
        
        // Buat data dummy setiap 2 detik
        if now - self.last_update > 2.0 {
            self.last_update = now;
            self.counter += 1;

            let classes = ["Arabica Roasted", "Robusta Green", "Arabica Green", "Robusta Roasted"];
            let data = CoffeeData {
                device_id: "azure_web_simulator".to_string(),
                predicted_class: classes[self.counter % 4].to_string(),
                confidence: 0.85 + (self.counter as f32 % 15.0) / 100.0,
                temperature: 26.0 + (self.counter as f32 % 5.0) * 0.5,
                humidity: 55.0 + (self.counter as f32 % 8.0) * 1.2,
                timestamp: format!("2026-09-29T16:{:02}:{:02}Z", (self.counter / 60) % 60, self.counter % 60),
            };

            self.history.insert(0, data);
            if self.history.len() > 50 { self.history.pop(); }
        }

        ctx.request_repaint(); // Memaksa animasi UI terus berjalan

        egui::TopBottomPanel::top("header_panel").show(ctx, |ui| {
            ui.add_space(10.0);
            ui.horizontal(|ui| {
                ui.heading(RichText::new("☕ Smart Coffee Web Dashboard").size(24.0).color(Color32::from_rgb(200, 160, 100)));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(RichText::new("Terhubung (Azure Cloud)").color(Color32::GREEN));
                });
            });
            ui.add_space(10.0);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(latest) = self.history.first() {
                ui.add_space(10.0);
                egui::Frame::group(ui.style()).fill(Color32::from_rgb(30, 30, 35)).show(ui, |ui| {
                    ui.set_width(ui.available_width());
                    ui.label(RichText::new("LATEST PREDICTION").size(14.0).color(Color32::GRAY));
                    ui.add_space(5.0);
                    
                    let class_color = match latest.predicted_class.as_str() {
                        "Arabica Roasted" | "Robusta Roasted" => Color32::from_rgb(255, 140, 0),
                        _ => Color32::from_rgb(144, 238, 144),
                    };

                    ui.horizontal(|ui| {
                        ui.label(RichText::new(&latest.predicted_class).size(32.0).strong().color(class_color));
                        ui.add_space(20.0);
                        ui.label(RichText::new(format!("Confidence: {:.1}%", latest.confidence * 100.0)).size(20.0));
                    });

                    ui.add_space(15.0);
                    ui.horizontal(|ui| {
                        ui.label(RichText::new(format!("🌡 Suhu: {:.1}°C", latest.temperature)).size(18.0).color(Color32::LIGHT_RED));
                        ui.add_space(20.0);
                        ui.label(RichText::new(format!("💧 Kelembaban: {:.1}%", latest.humidity)).size(18.0).color(Color32::LIGHT_BLUE));
                    });
                });
                ui.add_space(20.0);
            }

            ui.separator();
            ui.heading(RichText::new("Riwayat Deteksi").size(18.0));
            ui.add_space(10.0);

            ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
                egui::Grid::new("history_grid").striped(true).spacing(Vec2::new(30.0, 10.0)).show(ui, |ui| {
                    for data in &self.history {
                        ui.label(&data.timestamp);
                        ui.label(&data.predicted_class);
                        ui.label(format!("{:.1}%", data.confidence * 100.0));
                        ui.label(format!("{:.1}°C", data.temperature));
                        ui.end_row();
                    }
                });
            });
        });
    }
}

// Entry point khusus untuk WebAssembly (Browser)
#[cfg(target_arch = "wasm32")]
fn main() {
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();
    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "the_canvas_id",
                eframe::WebOptions::default(),
                Box::new(|cc| Box::new(CoffeeDashboard::new(cc))),
            )
            .await
            .expect("Gagal menjalankan eframe di browser");
    });
}