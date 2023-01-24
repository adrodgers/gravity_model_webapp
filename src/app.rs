use std::{collections::HashMap};

use ndarray::{Array1, Array2, Axis};
use crate::cuboid::{Cuboid, DataType, GravityObject};
use egui::{plot::{Line, Plot, PlotPoints, Polygon, LinkedAxisGroup, Points, Legend, LineStyle, PlotPoint}, Color32, Key};

const MAX_CUBOIDS: usize = 10;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    cuboids: HashMap<String,Cuboid>,
    current_cuboid_id: String,
    measurement_params: MeasurementParameters,
    #[serde(skip)]
    groups: Vec<LinkedAxisGroup>,
    add_cuboid_id: String,
    #[serde(skip)]
    colours: Box<dyn Iterator<Item = Color32>>,

}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct MeasurementParameters {
    measurement_type: DataType,
    x_start: f64,
    x_end: f64,
    n: usize,
    y: f64,
    z: f64,
    gradient: f64
}

impl Default for MeasurementParameters {
    fn default() -> Self {
        Self { measurement_type: DataType::Gz, x_start: -10., x_end: 10., n: 200, y: 0., z: 0.25, gradient: 0. }
    }
}

impl MeasurementParameters {
    pub fn points(&self) -> Array2<f64> {
        let x: Array1<f64> = ndarray::Array::linspace(self.x_start, self.x_end, self.n);
        let mut points: Array2<f64> = ndarray::Array::zeros((self.n, 3));
        for i in 0..x.len() {
            points[[i, 0]] = x[i] ;
            points[[i, 1]] = self.y;
            points[[i, 2]] = self.z + self.gradient*x[i];
        }
        points*1.0001
}
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            cuboids: HashMap::new(),
            measurement_params: MeasurementParameters::default(),
            current_cuboid_id: String::new(),
            groups: vec![LinkedAxisGroup::new(true, false),LinkedAxisGroup::new(false, true)],
            add_cuboid_id: String::new(),
            colours: Box::new(vec![Color32::RED,Color32::BLUE,Color32::YELLOW,Color32::GREEN,Color32::BROWN].into_iter().cycle())
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
        let Self { cuboids, groups, measurement_params, current_cuboid_id, add_cuboid_id, colours } = self;

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
                // if ui.button("reset all").clicked() {
                //     for (_,cuboid) in cuboids.iter_mut() {
                //         *cuboid = Cuboid::default();
                //     }
                // }
            });
            ui.text_edit_singleline(add_cuboid_id);
            if ui.button("Add cuboid").clicked() {
                if cuboids.len() < MAX_CUBOIDS {
                    cuboids.insert(add_cuboid_id.to_string(), Cuboid {id: add_cuboid_id.to_string(), colour: colours.next().unwrap(), ..Default::default()});
                    *current_cuboid_id = add_cuboid_id.to_string();
                }
            }
            if ui.button("Remove cuboid").clicked() {
                if cuboids.len() > 1 {
                    println!("Remove: {}",&current_cuboid_id);
                    cuboids.remove(&*current_cuboid_id);
                    *current_cuboid_id = cuboids.keys().next().unwrap().to_string();
                }
            }
            egui::ComboBox::from_label("Edit cuboid")
                .selected_text(current_cuboid_id.to_string())
                .show_ui(ui, |ui| {
                    for key in cuboids.keys() {
                        ui.selectable_value(current_cuboid_id, key.to_string(), key);
                    }
                }
            );
            if ui.button("reset").clicked() {
                let cuboid = cuboids.get_mut(current_cuboid_id).unwrap();
                *cuboid = Cuboid {id: cuboid.id.to_owned(), colour: cuboid.colour,..Default::default()};
            }

            if !cuboids.is_empty() {
                let current_cuboid = cuboids.get_mut(current_cuboid_id).unwrap();
                egui::CollapsingHeader::new("position")
                    .show(ui, |ui| {
                        ui.label("x centroid");
                        ui.add(egui::Slider::new(&mut current_cuboid.x_centroid, -50.0..=50.0).text("m"));

                        ui.label("y centroid");
                        ui.add(egui::Slider::new(&mut current_cuboid.y_centroid, -50.0..=50.0).text("m"));
                        
                        ui.label("z centroid");
                        ui.add(egui::Slider::new(&mut current_cuboid.z_centroid, -25.0..=25.0).text("m"));
                });
                egui::CollapsingHeader::new("volume")
                    .show(ui, |ui| {
                        ui.label("x length");
                        ui.add(egui::Slider::new(&mut current_cuboid.x_length, 0.1..=100.0).text("m"));
                        
                        ui.label("y length");
                        ui.add(egui::Slider::new(&mut current_cuboid.y_length, 0.1..=100.0).text("m"));

                        ui.label("z length");
                        ui.add(egui::Slider::new(&mut current_cuboid.z_length, 0.1..=25.0).text("m").drag_value_speed(0.1));
                        ui.separator();
                        ui.label(format!("Volume: {}",current_cuboid.volume()));
                });
                egui::CollapsingHeader::new("density")
                    .show(ui, |ui| {
                        ui.add(egui::Slider::new(&mut current_cuboid.density, -3000.0..=22590.).text("kg/m^3"));
                        if ui.button("soil void").clicked() {
                            current_cuboid.density = -1800.;
                        }
                        if ui.button("concrete").clicked() {
                            current_cuboid.density = 2000.;
                        }
                        if ui.button("lead").clicked() {
                            current_cuboid.density = 11340.;
                        }
                        if ui.button("tungsten").clicked() {
                            current_cuboid.density = 19300.;
                        }
                        ui.separator();
                        ui.label(format!("Mass: {}",current_cuboid.mass()));      
                });
            }
            
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
                    ui.label("gradient");
                    ui.add(egui::Slider::new(&mut measurement_params.gradient, -0.5..=0.5));
            });

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
            // ui.heading("Gravity Model Builder");
            let measurement_points = measurement_params.points();
            let x = measurement_points.index_axis(Axis(1),0);
            
            // Associate colour with a cuboid. Also give cuboid a name id.
            let data_plot = Plot::new("gravity")
            .view_aspect(2.0)
            .link_axis(groups[0].clone())
            .include_x(-10.)
            .include_x(10.)
            .include_y(0.)
            .width(720.)
            .legend(Legend::default());

            let edit_mode = ctx.input().key_down(Key::P) || ctx.input().key_down(Key::L);

            let model_plot_xz = Plot::new("underground_xz")
                .view_aspect(2.0)
                .data_aspect(1.0)
                .link_axis(groups[0].clone())
                .link_axis(groups[1].clone())
                .include_x(-10.)
                .include_x(10.)
                .include_y(2.)
                .include_y(-10.)
                .width(720.)
                .legend(Legend::default())
                .allow_boxed_zoom(if edit_mode {false} else {true})
                .allow_drag(if edit_mode {false} else {true});

            let model_plot_yz = Plot::new("underground_yz")
                .view_aspect(2.0)
                .data_aspect(1.0)
                .link_axis(groups[1].clone())
                .include_x(-10.)
                .include_x(10.)
                .include_y(2.)
                .include_y(-10.)
                .width(720.)
                .legend(Legend::default())
                .allow_boxed_zoom(if edit_mode {false} else {true})
                .allow_drag(if edit_mode {false} else {true});

            let model_plot_xy = Plot::new("underground_xy")
                .view_aspect(2.0)
                .data_aspect(1.0)
                // .link_axis(groups[1].clone())
                .include_x(-10.)
                .include_x(10.)
                .include_y(2.)
                .include_y(-10.)
                .width(720.)
                .legend(Legend::default())
                .allow_boxed_zoom(if edit_mode {false} else {true})
                .allow_drag(if edit_mode {false} else {true});
            
            let mut data_total: Array1<f64> = Array1::zeros(measurement_points.len_of(Axis(0)));
            ui.horizontal(|ui| {
                data_plot
                .show(ui, |plot_ui| {
                    for (key,cuboid) in cuboids.iter() {
                        let data = cuboid.calculate(&measurement_params.measurement_type,&measurement_points);
                        let data_2d: Vec<_> = x.into_iter().zip(data.iter()).map(|(x,val)| {
                        [*x,*val]}).collect();
                        let line = Line::new(data_2d);
                        plot_ui.line(line.name(key).color(cuboid.colour));
                        data_total = data_total + &data;
                    }
                    let data_2d: Vec<_> = x.into_iter().zip(data_total.into_iter()).map(|(x,val)| {
                    [*x,val]}).collect();
                    let data_total_line = Line::new(data_2d);
                    plot_ui.line(data_total_line.name("Combined").color(Color32::WHITE).style(LineStyle::dashed_loose()));
                });

                model_plot_xy
                .show(ui, |plot_ui| {
                    let plot_points: Vec<[f64; 2]> = measurement_points.index_axis(Axis(1), 0).into_iter()
                    .zip(measurement_points.index_axis(Axis(1), 1).into_iter())
                    .map(|(x,z)| [*x,*z]).collect();
                    plot_ui.points(Points::new(plot_points).name("data points").color(Color32::WHITE));

                    for (key,cuboid) in cuboids.iter() {
                        let polygon = Polygon::new(PlotPoints::from(cuboid.vertices_xy()));
                        plot_ui.polygon(polygon.name(key).color(cuboid.colour));
                    }

                    if ctx.input().key_down(Key::L) && plot_ui.plot_hovered() {
                        let drag_delta = plot_ui.pointer_coordinate_drag_delta();
                        if !cuboids.is_empty() {
                            let current_cuboid = cuboids.get_mut(current_cuboid_id).unwrap();
                            if current_cuboid.x_length + drag_delta.x as f64 > 0. {
                                current_cuboid.x_length += drag_delta.x as f64;
                            }
                            
                            if current_cuboid.y_length + drag_delta.y as f64 > 0. {
                                current_cuboid.y_length += drag_delta.y as f64;
                            }
                        }
                    }

                    if ctx.input().key_down(Key::P) && plot_ui.plot_hovered() {
                        let position = plot_ui.pointer_coordinate().unwrap_or(PlotPoint {x: 0., y: 0.}).to_vec2();
                        if !cuboids.is_empty() {
                            let current_cuboid = cuboids.get_mut(current_cuboid_id).unwrap();
                            current_cuboid.x_centroid = position.x as f64;
                            current_cuboid.y_centroid = position.y as f64;
                        }
                    }
                });
            });
            ui.separator();

            ui.horizontal(|ui|{
                model_plot_xz
                .show(ui, |plot_ui| {
                    let plot_points: Vec<[f64; 2]> = measurement_points.index_axis(Axis(1), 0).into_iter()
                    .zip(measurement_points.index_axis(Axis(1), 2).into_iter())
                    .map(|(x,z)| [*x,*z]).collect();
                    plot_ui.points(Points::new(plot_points).name("data points").color(Color32::WHITE));

                    for (key,cuboid) in cuboids.iter() {
                        let polygon = Polygon::new(PlotPoints::from(cuboid.vertices_xz()));
                        plot_ui.polygon(polygon.name(key).color(cuboid.colour));
                    }

                    if ctx.input().key_down(Key::L) && plot_ui.plot_hovered() {
                        let drag_delta = plot_ui.pointer_coordinate_drag_delta();
                        if !cuboids.is_empty() {
                            let current_cuboid = cuboids.get_mut(current_cuboid_id).unwrap();
                            if current_cuboid.x_length + drag_delta.x as f64 > 0. {
                                current_cuboid.x_length += drag_delta.x as f64;
                            }
                            
                            if current_cuboid.z_length + drag_delta.y as f64 > 0. {
                                current_cuboid.z_length += drag_delta.y as f64;
                            }
                        }
                    }

                    if ctx.input().key_down(Key::P) && plot_ui.plot_hovered() {
                        let position = plot_ui.pointer_coordinate().unwrap_or(PlotPoint {x: 0., y: 0.}).to_vec2();
                        if !cuboids.is_empty() {
                            let current_cuboid = cuboids.get_mut(current_cuboid_id).unwrap();
                            current_cuboid.x_centroid = position.x as f64;
                            current_cuboid.z_centroid = position.y as f64;
                        }
                    }
                });

                model_plot_yz
                .show(ui, |plot_ui| {
                    let plot_points: Vec<[f64; 2]> = measurement_points.index_axis(Axis(1), 1).into_iter()
                    .zip(measurement_points.index_axis(Axis(1), 2).into_iter())
                    .map(|(x,z)| [*x,*z]).collect();
                    plot_ui.points(Points::new(plot_points).name("data points").color(Color32::WHITE));

                    for (key,cuboid) in cuboids.iter() {
                        let polygon = Polygon::new(PlotPoints::from(cuboid.vertices_yz()));
                        plot_ui.polygon(polygon.name(key).color(cuboid.colour));
                    }

                    if ctx.input().key_down(Key::L) && plot_ui.plot_hovered() {
                        let drag_delta = plot_ui.pointer_coordinate_drag_delta();
                        if !cuboids.is_empty() {
                            let current_cuboid = cuboids.get_mut(current_cuboid_id).unwrap();
                            if current_cuboid.y_length + drag_delta.x as f64 > 0. {
                                current_cuboid.y_length += drag_delta.x as f64;
                            }
                            
                            if current_cuboid.z_length + drag_delta.y as f64 > 0. {
                                current_cuboid.z_length += drag_delta.y as f64;
                            }
                        }
                    }

                    if ctx.input().key_down(Key::P) && plot_ui.plot_hovered() {
                        let position = plot_ui.pointer_coordinate().unwrap_or(PlotPoint {x: 0., y: 0.}).to_vec2();
                        if !cuboids.is_empty() {
                            let current_cuboid = cuboids.get_mut(current_cuboid_id).unwrap();
                            current_cuboid.y_centroid = position.x as f64;
                            current_cuboid.z_centroid = position.y as f64;
                        }
                    }
                });

                

            });
            if ui.button("verts").clicked() {
                for (_,cuboid) in cuboids {
                    println!("{:?}",cuboid.vertices());
                    println!("{:?}",cuboid.vertices_xz());
                    println!("{:?}",cuboid.vertices_yz());
                    println!("{:?}",cuboid.vertices_xy());
                }
                
            }
        });
    }
}
