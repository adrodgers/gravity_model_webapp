use ndarray::{Array1, Array2, Axis};
use crate::cuboid::{Cuboid, self};
use egui::{plot::{Line, Plot, PlotPoints, Polygon, LinkedAxisGroup, Points, Legend}, Color32};



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
    // cuboids: Vec<Cuboid>,
    cuboids_params: Vec<CuboidParameters>,
    current_cuboid_index: usize,
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
        points*1.00001
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

impl CuboidParameters {
    pub fn cuboid(&self) -> Cuboid {
        Cuboid::new_from_lengths([self.x_centroid,self.y_centroid,self.z_centroid],
            [self.x_length,self.y_length,self.z_length], self.density)
    }
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // cuboid_params: CuboidParameters::default(),
            // cuboids: vec![Cuboid::default()],
            cuboids_params: vec![CuboidParameters::default()],
            measurement_params: MeasurementParameters::default(),
            current_cuboid_index: 0,
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
        let Self { cuboids_params, group, measurement_params, current_cuboid_index } = self;
        let colour_vec = vec![Color32::RED,Color32::BLUE,Color32::YELLOW,Color32::GREEN,Color32::BROWN];
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
                ui.heading("Model Settings");
                if ui.button("reset all").clicked() {
                    for cuboid_params in cuboids_params.iter_mut() {
                        *cuboid_params = CuboidParameters::default();
                    }
                }
            });
            if ui.button("Add cuboid").clicked() {
                if cuboids_params.len() < 5 {
                    cuboids_params.push(CuboidParameters::default());
                }
                if *current_cuboid_index < 4 {
                    *current_cuboid_index += 1;
                }
            }
            if ui.button("Remove cuboid").clicked() {
                if cuboids_params.len() > 1 {
                    cuboids_params.remove(*current_cuboid_index);
                }
                if *current_cuboid_index > 0 {
                    *current_cuboid_index -= 1;
                }
            }
            egui::ComboBox::from_label("Edit cuboid")
                .selected_text(format!("{:?}", current_cuboid_index))
                .show_ui(ui, |ui| {
                    for i in 0..cuboids_params.len() {
                        ui.selectable_value(current_cuboid_index, i as usize, i.to_string());
                    }
                }
            );
            if *current_cuboid_index > cuboids_params.len() - 1 {
                *current_cuboid_index = 0;
            }
            egui::CollapsingHeader::new("position")
                .show(ui, |ui| {
                    ui.label("x centroid");
                    ui.add(egui::Slider::new(&mut cuboids_params[*current_cuboid_index].x_centroid, -50.0..=50.0).text("m"));

                    ui.label("y centroid");
                    ui.add(egui::Slider::new(&mut cuboids_params[*current_cuboid_index].y_centroid, -50.0..=50.0).text("m"));
                    
                    ui.label("z centroid");
                    ui.add(egui::Slider::new(&mut cuboids_params[*current_cuboid_index].z_centroid, -25.0..=25.0).text("m"));
            });
            egui::CollapsingHeader::new("volume")
                .show(ui, |ui| {
                    ui.label("x length");
                    ui.add(egui::Slider::new(&mut cuboids_params[*current_cuboid_index].x_length, 1.0..=100.0).text("m"));
                    
                    ui.label("y length");
                    ui.add(egui::Slider::new(&mut cuboids_params[*current_cuboid_index].y_length, 1.0..=100.0).text("m"));

                    ui.label("z length");
                    ui.add(egui::Slider::new(&mut cuboids_params[*current_cuboid_index].z_length, 1.0..=25.0).text("m"));
                    ui.separator();
                    ui.label(format!("Volume: {}",cuboids_params[*current_cuboid_index].cuboid().volume()));
            });
            egui::CollapsingHeader::new("density")
                .show(ui, |ui| {
                    ui.add(egui::Slider::new(&mut cuboids_params[*current_cuboid_index].density, -3000.0..=22590.).text("kg/m^3"));
                    if ui.button("soil void").clicked() {
                        cuboids_params[*current_cuboid_index].density = -1800.;
                    }
                    if ui.button("concrete").clicked() {
                        cuboids_params[*current_cuboid_index].density = 2000.;
                    }
                    if ui.button("lead").clicked() {
                        cuboids_params[*current_cuboid_index].density = 11340.;
                    }
                    if ui.button("tungsten").clicked() {
                        cuboids_params[*current_cuboid_index].density = 19300.;
                    }
                    ui.separator();
                    ui.label(format!("Mass: {}",cuboids_params[*current_cuboid_index].cuboid().mass()));
            });
            
            ui.separator();
            ui.horizontal(|ui| {
                ui.heading("Measurement Settings");
                if ui.button("reset").clicked() {
                    *measurement_params = MeasurementParameters::default();
                }
            });
            egui::CollapsingHeader::new("measurement")
                .show(ui, |ui| {
                    egui::ComboBox::from_label("data type")
                        .selected_text(format!("{:?}", measurement_params.measurement_type))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut measurement_params.measurement_type, DataType::Gx, "gx");
                            ui.selectable_value(&mut measurement_params.measurement_type, DataType::Gy, "gy");
                            ui.selectable_value(&mut measurement_params.measurement_type, DataType::Gz, "gz");
                            ui.selectable_value(&mut measurement_params.measurement_type, DataType::Gxx, "gxx");
                            ui.selectable_value(&mut measurement_params.measurement_type, DataType::Gxy, "gxy");
                            ui.selectable_value(&mut measurement_params.measurement_type, DataType::Gxz, "gxz");
                            ui.selectable_value(&mut measurement_params.measurement_type, DataType::Gyy, "gyy");
                            ui.selectable_value(&mut measurement_params.measurement_type, DataType::Gyz, "gyz");
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
            // *cuboids = vec![];
            // for params in cuboids_params {
            //     cuboids.push(Cuboid::new_from_lengths([params.x_centroid,params.y_centroid,params.z_centroid],
            //         [params.x_length,params.y_length,params.z_length], params.density))
            // }
            
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
            let mut data: Vec<Array1<f64>> = vec![Array1::zeros(measurement_points.len());cuboids_params.len()];
            let mut data_total: Array1<f64> = Array1::zeros(measurement_points.len());
            let x = measurement_points.index_axis(Axis(1),0);
            // if ui.button("x").clicked() {
            //     println!("{:?}",x);
            // }
            // let polygon = Polygon::new(PlotPoints::from(cuboid.vertices_xz()));
            for (j,cuboid_params) in cuboids_params.iter().enumerate() {
                let cuboid = cuboid_params.cuboid();
                match &measurement_params.measurement_type {
                    DataType::Gx => {for (i, point )in measurement_points.axis_iter(Axis(0)).enumerate() {
                        data[j][i] += cuboid.gx(&point.to_owned())
                    }},
                    DataType::Gy => {for (i, point )in measurement_points.axis_iter(Axis(0)).enumerate() {
                        data[j][i] += cuboid.gy(&point.to_owned())
                    }},
                    DataType::Gz => {for (i, point )in measurement_points.axis_iter(Axis(0)).enumerate() {
                        data[j][i] += cuboid.gz(&point.to_owned())
                    }},
                    DataType::Gxx => {for (i, point )in measurement_points.axis_iter(Axis(0)).enumerate() {
                        data[j][i] += cuboid.gxx(&point.to_owned())
                    }},
                    DataType::Gxy => {for (i, point )in measurement_points.axis_iter(Axis(0)).enumerate() {
                        data[j][i] += cuboid.gxy(&point.to_owned())
                    }},
                    DataType::Gxz => {for (i, point )in measurement_points.axis_iter(Axis(0)).enumerate() {
                        data[j][i] += cuboid.gxz(&point.to_owned())
                    }},
                    DataType::Gyy => {for (i, point )in measurement_points.axis_iter(Axis(0)).enumerate() {
                        data[j][i] += cuboid.gyy(&point.to_owned())
                    }},
                    DataType::Gyz => {for (i, point )in measurement_points.axis_iter(Axis(0)).enumerate() {
                        data[j][i] += cuboid.gyz(&point.to_owned())
                    }},
                    DataType::Gzz => {for (i, point )in measurement_points.axis_iter(Axis(0)).enumerate() {
                        data[j][i] += cuboid.gzz(&point.to_owned())
                    }},
                    _ => {}
                }
                data_total = data_total + &data[j];
            }
            
            // if ui.button("gz").clicked() {
            //     println!("{:?}",gz);
            // }
            let mut data_plot = Plot::new("gravity")
            .view_aspect(2.0)
            .link_axis(group.clone())
            .include_x(-50.)
            .include_x(50.)
            .include_y(0.)
            .legend(Legend::default());

            let mut model_plot = Plot::new("underground")
            .view_aspect(2.0)
            .link_axis(group.clone())
            .include_x(-50.)
            .include_x(50.)
            .include_y(2.)
            .include_y(-10.)
            .legend(Legend::default());

            let data_2d: Vec<_> = x.into_iter().zip(data_total.into_iter()).map(|(x,val)| {
            let scaling = match measurement_params.measurement_type {
                DataType::Gx | DataType::Gy | DataType::Gz => {1E8},
                _ => {1E9}
            };
            // data_total = data_total + datum * scaling;
            [*x,val*scaling]}).collect();
            let data_total_line = Line::new(data_2d);
            
            data_plot
            .show(ui, |plot_ui| {plot_ui.line(data_total_line.color(Color32::WHITE).name("data total")); for (i,datum) in data.iter().enumerate() {
                let data_2d: Vec<_> = x.into_iter().zip(datum.into_iter()).map(|(x,val)| {
                let scaling = match measurement_params.measurement_type {
                    DataType::Gx | DataType::Gy | DataType::Gz => {1E8},
                    _ => {1E9}
                };
                // data_total = data_total + datum * scaling;
                [*x,val*scaling]}).collect();
                let line = Line::new(data_2d);
                plot_ui.line(line.name(format!("cuboid {i}")).color(colour_vec[i]));
            }});

            ui.separator();
            model_plot
            .show(ui, |plot_ui| { let plot_points: Vec<[f64; 2]> = measurement_points.index_axis(Axis(1), 0).into_iter()
                .zip(measurement_points.index_axis(Axis(1), 2).into_iter())
                .map(|(x,z)| [*x,*z]).collect();
                plot_ui.points(Points::new(plot_points).name("Measurement points").color(Color32::WHITE));

                for (i,cuboid_params) in cuboids_params.iter().enumerate() {
                    let cuboid = cuboid_params.cuboid();
                    let polygon = Polygon::new(PlotPoints::from(cuboid.vertices_xz()));
                    plot_ui.polygon(polygon.name(format!("cuboid {i}")).color(colour_vec[i]));
                }
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
