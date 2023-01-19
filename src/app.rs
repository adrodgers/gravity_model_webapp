use ndarray::{arr1, Array1, Array2, Axis, Array};
use crate::cuboid::Cuboid;
use egui::plot::{Line, Plot, PlotPoints, Polygon, LinkedAxisGroup, Points};

/// Generate a line of measurement points along the x axis

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state

pub struct TemplateApp {
    // Example stuff:
    // label: String,

    // this how you opt-out of serialization of a member
    // #[serde(skip)]
    // value: f32,
    #[serde(skip)]
    // value_2: f32
    // cuboid_params: CuboidParameters
    cuboid: Cuboid,
    cuboid_params: CuboidParameters,
    #[serde(skip)]
    measurement_params: MeasurementParameters,
    #[serde(skip)]
    group: LinkedAxisGroup,
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
#[derive(serde::Deserialize, serde::Serialize)]
pub struct MeasurementParameters {
    measurement_type: DataType,
    x_start: f64,
    x_end: f64,
    n: usize,
    y: f64,
    z: f64
}

impl Default for MeasurementParameters {
    fn default() -> Self {
        Self { measurement_type: DataType::Gz, x_start: -50., x_end: 50., n: 200, y: 0., z: 0.25 }
    }
}

impl MeasurementParameters {
    pub fn points(&self) -> Array2<f64> {
        let x: Array1<f64> = ndarray::Array::linspace(self.x_start, self.x_end, self.n);
        let mut points: Array2<f64> = ndarray::Array::zeros((self.n, 3));
        for i in 0..x.len() {
            points[[i, 0]] = x[i];
            points[[i, 1]] = self.y;
            points[[i, 2]] = self.z;
        }
        points
}
}
#[derive(serde::Deserialize, serde::Serialize, Debug, PartialEq)]
enum DataType {
    Gx,
    Gy,
    Gz,
    Gxx,
    Gxy,
    Gxz,
    Gyy,
    Gyz,
    Gzz
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
            // cuboid_params: CuboidParameters::default(),
            cuboid: Cuboid::default(),
            cuboid_params: CuboidParameters::default(),
            measurement_params: MeasurementParameters::default(),
            group: LinkedAxisGroup::new(true, false),
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
        // let Self { cuboid_params } = self;
        let Self { cuboid, cuboid_params, group, measurement_params } = self;

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
            ui.horizontal(|ui| {
                ui.heading("Cuboid Parameters");
                if ui.button("reset").clicked() {
                    *cuboid_params = CuboidParameters::default();
                }
            });
            egui::CollapsingHeader::new("position")
                .show(ui, |ui| {
                    ui.label("x centroid");
                    ui.add(egui::Slider::new(&mut cuboid_params.x_centroid, -50.0..=50.0).text("m"));

                    ui.label("y centroid");
                    ui.add(egui::Slider::new(&mut cuboid_params.y_centroid, -50.0..=50.0).text("m"));
                    
                    ui.label("z centroid");
                    ui.add(egui::Slider::new(&mut cuboid_params.z_centroid, -25.0..=25.0).text("m"));
            });
            egui::CollapsingHeader::new("volume")
                .show(ui, |ui| {
                    ui.label("x length");
                    ui.add(egui::Slider::new(&mut cuboid_params.x_length, 1.0..=100.0).text("m"));
                    
                    ui.label("y length");
                    ui.add(egui::Slider::new(&mut cuboid_params.y_length, 1.0..=100.0).text("m"));

                    ui.label("z length");
                    ui.add(egui::Slider::new(&mut cuboid_params.z_length, 1.0..=25.0).text("m"));
                    ui.separator();
                    ui.label(format!("Volume: {}",cuboid.volume()));
            });
            egui::CollapsingHeader::new("density")
                .show(ui, |ui| {
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
                    ui.separator();
                    ui.label(format!("Mass: {}",cuboid.mass()));
            });
            
            ui.separator();
            ui.heading("Data Parameters");
            egui::CollapsingHeader::new("measurement")
                .show(ui, |ui| {
                    egui::ComboBox::from_label("data type")
                        .selected_text(format!("{:?}", measurement_params.measurement_type))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut measurement_params.measurement_type, DataType::Gz, "gz");
                            ui.selectable_value(&mut measurement_params.measurement_type, DataType::Gzz, "gzz");
                        }
                    );
                    ui.label("x start");
                    ui.add(egui::Slider::new(&mut measurement_params.x_start, -50.0..=0.).text("m"));
                    ui.label("x end");
                    ui.add(egui::Slider::new(&mut measurement_params.x_end, 0.0..=50.).text("m"));
                    ui.label("n measurements");
                    ui.add(egui::Slider::new(&mut measurement_params.n, 1..=1000));
                    ui.label("y");
                    ui.add(egui::Slider::new(&mut measurement_params.y, -50.0..=50.));
                    ui.label("z");
                    ui.add(egui::Slider::new(&mut measurement_params.z, -25.0..=25.));
            });
            

            *cuboid = Cuboid::new_from_lengths([cuboid_params.x_centroid,cuboid_params.y_centroid,cuboid_params.z_centroid],
                [cuboid_params.x_length,cuboid_params.y_length,cuboid_params.z_length], cuboid_params.density);
            
            // if ui.button("vertices").clicked() {
            //     println!("{:?}",cuboid.vertices);
            //     println!("{:?}",cuboid.vertices_xz());
            // }

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
            let measurement_points = measurement_params.points();
            let mut data: Array1<f64> = Array1::zeros(measurement_points.len());
            let x = measurement_points.index_axis(Axis(1),0);
            // if ui.button("x").clicked() {
            //     println!("{:?}",x);
            // }
            // let polygon = Polygon::new(PlotPoints::from(cuboid.vertices_xz()));
            match &measurement_params.measurement_type {
                DataType::Gz => {for (i, point )in measurement_points.axis_iter(Axis(0)).enumerate() {
                    data[i] += cuboid.gz(&point.to_owned())
                }},
                DataType::Gzz => {for (i, point )in measurement_points.axis_iter(Axis(0)).enumerate() {
                    data[i] += cuboid.gzz(&point.to_owned())
                }},
                _ => {}
            }
            
            // if ui.button("gz").clicked() {
            //     println!("{:?}",gz);
            // }
            let data: Vec<_> = x.into_iter().zip(data.into_iter()).map(|(x,data)| {
                let scaling = match measurement_params.measurement_type {
                    DataType::Gx | DataType::Gy | DataType::Gz => {1E8},
                    _ => {1E9}
                };
                [*x,data*scaling]}).collect();
            let line = Line::new(data);
            Plot::new("gravity")
            .view_aspect(2.0)
            .link_axis(group.clone())
            .include_x(-50.)
            .include_x(50.)
            .include_y(0.)
            .show(ui, |plot_ui| plot_ui.line(line));

            ui.separator();
            let polygon = Polygon::new(PlotPoints::from(cuboid.vertices_xz()));
            let plot_points: Vec<[f64; 2]> = measurement_points.index_axis(Axis(1), 0).into_iter()
            .zip(measurement_points.index_axis(Axis(1), 2).into_iter())
            .map(|(x,z)| [*x,*z]).collect();
            Plot::new("underground")
            .view_aspect(2.0)
            .link_axis(group.clone())
            .include_x(-50.)
            .include_x(50.)
            .include_y(2.)
            .include_y(-10.)
            .show(ui, |plot_ui| {
                plot_ui.polygon(polygon.name("Cuboid"));
                plot_ui.points(Points::new(plot_points));
            });
            
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
