use crate::cuboid::Cuboid;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    // label: String,

    // this how you opt-out of serialization of a member
    // #[serde(skip)]
    // value: f32,
    // #[serde(skip)]
    // value_2: f32
    cuboid_params: CuboidParameters
}
#[derive(serde::Deserialize, serde::Serialize)]
pub struct CuboidParameters {
    pub x_length: f64,
    pub y_length: f64,
    pub z_length: f64,
    pub x_centroid: f64,
    pub y_centroid: f64,
    pub z_centroid: f64, 
    pub x_rotation: f64,
    pub y_rotation: f64,
    pub z_rotation: f64,
    pub density: f64
}

impl Default for CuboidParameters {
    fn default() -> Self {
        Self { x_length: 1.,
            y_length: 1.,
            z_length: 1.,
            x_centroid: 0.,
            y_centroid: 0.,
            z_centroid: -0.5,
            x_rotation: 0.,
            y_rotation: 0.,
            z_rotation: 0.,
            density: -2000.}
    }
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            cuboid_params: CuboidParameters::default(),
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { cuboid_params } = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Cuboid Parameters");

            // ui.horizontal(|ui| {
            //     ui.label("Write something: ");
            //     ui.text_edit_singleline(label);
            // });

            // ui.add(egui::DragValue::new(&mut cuboid_params.x_length)
            // .clamp_range(0..=100));
            // if ui.button(">").clicked() {
            //     cuboid_params.x_length += 1.0;
            // }
            ui.label("x length");
            ui.add(egui::Slider::new(&mut cuboid_params.x_length, 1.0..=100.0).text("m"));
            
            ui.label("y length");
            ui.add(egui::Slider::new(&mut cuboid_params.y_length, 1.0..=100.0).text("m"));

            ui.label("z length");
            ui.add(egui::Slider::new(&mut cuboid_params.z_length, 1.0..=50.0).text("m"));

            ui.label("x centroid");
            ui.add(egui::Slider::new(&mut cuboid_params.x_centroid, -50.0..=50.0).text("m"));

            ui.label("y centroid");
            ui.add(egui::Slider::new(&mut cuboid_params.y_centroid, -50.0..=50.0).text("m"));
            
            ui.label("z centroid");
            ui.add(egui::Slider::new(&mut cuboid_params.z_centroid, -25.0..=-cuboid_params.z_length/2.).text("m"));

            ui.label("density");
            ui.add(egui::Slider::new(&mut cuboid_params.density, -3000.0..=22590.).text("kg/m^3"));

            if ui.button("soil void").clicked() {
                cuboid_params.density = -1800.;
            }
            if ui.button("concrete").clicked() {
                cuboid_params.density = 2000.;
            }
            if ui.button("lead").clicked() {
                cuboid_params.density = 11340.;
            }
            if ui.button("tungsten").clicked() {
                cuboid_params.density = 19300.;
            }
            let cuboid = Cuboid::new_from_lengths([cuboid_params.x_centroid,cuboid_params.y_centroid,cuboid_params.z_centroid],
                [cuboid_params.x_length,cuboid_params.y_length,cuboid_params.z_length], cuboid_params.density);
            ui.label(format!("Volume: {}",cuboid.volume()));
            ui.label(format!("Mass: {}",cuboid.mass()));

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
            ui.hyperlink("https://github.com/adrodgers/gravity_model_webapp");
            ui.add(egui::github_link_file!(
                "https://github.com/adrodgers/gravity_model_webapp/blob/master/",
                "Source code."
            ));
            egui::warn_if_debug_build(ui);
                ui.horizontal(|ui| {
                    // ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("powered by ");
                    ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                    ui.label(" and ");
                    ui.hyperlink_to(
                        "eframe",
                        "https://github.com/emilk/egui/tree/master/crates/eframe",
                    );
                    ui.label(".");
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("Gravity Model");
            use egui::plot::{Line, Plot, PlotPoints};
            let sin: PlotPoints = (0..1000).map(|i| {
                let x = i as f64 * 0.01;
                [x, x.sin()]
            }).collect();
            let line = Line::new(sin);
            Plot::new("my_plot").view_aspect(2.0).show(ui, |plot_ui| plot_ui.line(line));
            
        });

        // if true {
        //     egui::Window::new("Window").show(ctx, |ui| {
        //         ui.label("Windows can be moved by dragging them.");
        //         ui.label("They are automatically sized based on contents.");
        //         ui.label("You can turn on resizing and scrolling if you like.");
        //         ui.label("You would normally choose either panels OR windows.");
        //     });
        // }
    }
}
