use std::{
    collections::{BTreeMap, BTreeSet},
    f64::consts::TAU,
};

use crate::gravity_objects::{Cuboid, DataType, GravityCalc, GravityInfo, GravityObject, Sphere};
use egui::{
    plot::{Legend, Line, LineStyle, LinkedAxisGroup, Plot, PlotPoints, PlotUi, Points, Polygon},
    Color32, Key,
};
use ndarray::{Array1, Array2, Axis};

const MAX_OBJECTS: usize = 100;
#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Model {
    objects: BTreeMap<String, Option<GravityObject>>,
    groups: BTreeMap<String, Option<BTreeSet<String>>>,
    object_counter: u128,
}

impl Default for Model {
    fn default() -> Self {
        let mut objects = BTreeMap::new();
        objects.insert("None".to_string(), None);
        let mut groups: BTreeMap<String, Option<BTreeSet<String>>> = BTreeMap::new();
        groups.insert("None".to_string(), None);
        Self { objects, groups, object_counter: 0 }
    }
}

impl Model {
    // pub fn calculate(&self, data_type: &DataType, measurement_points: &Array2<f64>) -> Array1<f64> {
    //     let mut data = Array1::zeros(measurement_points.len_of(Axis(0)));
    //     for (id, object) in self.objects.iter() {
    //         match object {
    //             Some(obj) => {
    //                 match obj {
    //                     GravityObject::Cuboid(cuboid) => {data += &cuboid.calculate(data_type, measurement_points)},
    //                     GravityObject::Sphere(sphere) => {data += &sphere.calculate(data_type, measurement_points)},
    //                 }
    //             },
    //             None => {},
    //         };
    //     }
    //     data
    // }

    pub fn number_objects_selected(&self) -> u128 {
        let mut num_selected = 0;
        for (_, object) in self.objects.iter() {
            match object {
                Some(obj) => match obj {
                    GravityObject::Cuboid(cuboid) => {
                        if cuboid.is_selected {
                            num_selected += 1
                        }
                    }
                    GravityObject::Sphere(sphere) => {
                        if sphere.is_selected {
                            num_selected += 1
                        }
                    }
                },
                None => {}
            }
        }
        num_selected
    }

    pub fn selected_object_ids(&self) -> Vec<String> {
        let mut selected_object_ids = vec![];
        for (_, object) in self.objects.iter() {
            match object {
                Some(obj) => {
                    match obj {
                        GravityObject::Cuboid(cuboid) => {
                            if cuboid.is_selected {
                                selected_object_ids.push(cuboid.id.to_string());
                            } else {
                                // cuboid.is_selected = false;
                            }
                        }
                        GravityObject::Sphere(sphere) => {
                            if sphere.is_selected {
                                selected_object_ids.push(sphere.id.to_string());
                            } else {
                                // cuboid.is_selected = false;
                            }
                        }
                    }
                }
                None => {}
            }
        }
        selected_object_ids
    }

    // pub fn object_centroids(&self) -> Vec<Array1<f64>> {
    //     let mut centroids= vec![];
    //     for (id,object) in self.objects.iter() {
    //         match object {
    //             Some(obj) => {
    //                 match obj {
    //                     GravityObject::Cuboid(cuboid) => centroids.push(cuboid.centre()),
    //                     GravityObject::Sphere(sphere) => centroids.push(sphere.centre()),
    //                 }
    //             },
    //             None => {},
    //         }
    //     }
    //     centroids
    // }

    pub fn select_by_click_xz(&mut self, plot_ui: &mut PlotUi) {
        for (_, object) in self.objects.iter_mut() {
            let pointer_pos = plot_ui.pointer_coordinate().unwrap();
            match object {
                Some(obj) => {
                    match obj {
                        GravityObject::Cuboid(cuboid) => {
                            if (cuboid.x_centroid - cuboid.x_length / 2.) < pointer_pos.x as f64
                                && (cuboid.x_centroid + cuboid.x_length / 2.) > pointer_pos.x as f64
                                && (cuboid.z_centroid + cuboid.z_length / 2.) > pointer_pos.y as f64
                                && (cuboid.z_centroid - cuboid.z_length / 2.) < pointer_pos.y as f64
                            {
                                cuboid.is_selected = !cuboid.is_selected;
                            } else {
                                // cuboid.is_selected = false;
                            }
                        }
                        GravityObject::Sphere(sphere) => {
                            if ((sphere.x_centroid - pointer_pos.x as f64).powi(2)
                                + (sphere.z_centroid - pointer_pos.y as f64).powi(2))
                            .sqrt()
                                < sphere.radius
                            {
                                sphere.is_selected = !sphere.is_selected;
                            } else {
                                // sphere.is_selected = false;
                            }
                        }
                    }
                }
                None => {}
            }
        }
    }

    fn translate_selected(&mut self, plot_ui: &mut PlotUi) {
        for (_, object) in self.objects.iter_mut() {
            let pointer_delta = plot_ui.pointer_coordinate_drag_delta();
            match object {
                Some(obj) => {
                    match obj {
                        GravityObject::Cuboid(cuboid) => {
                            if cuboid.is_selected {
                                cuboid.x_centroid += pointer_delta.x as f64;
                                cuboid.z_centroid += pointer_delta.y as f64;
                            } else {
                                // cuboid.is_selected = false;
                            }
                        }
                        GravityObject::Sphere(sphere) => {
                            if sphere.is_selected {
                                sphere.x_centroid += pointer_delta.x as f64;
                                sphere.z_centroid += pointer_delta.y as f64;
                            } else {
                                // cuboid.is_selected = false;
                            }
                        }
                    }
                }
                None => {}
            }
        }
    }

    fn scale_selected(&mut self, plot_ui: &mut PlotUi) {
        for (_, object) in self.objects.iter_mut() {
            let pointer_delta = plot_ui.pointer_coordinate_drag_delta();
            match object {
                Some(obj) => {
                    match obj {
                        GravityObject::Cuboid(cuboid) => {
                            if cuboid.is_selected {
                                if (cuboid.x_length + pointer_delta.x as f64) > 0. {
                                    cuboid.x_length += pointer_delta.x as f64;
                                }
                                if (cuboid.z_length + pointer_delta.y as f64) > 0. {
                                    cuboid.z_length += pointer_delta.y as f64;
                                }
                            } else {
                                // cuboid.is_selected = false;
                            }
                        }
                        GravityObject::Sphere(sphere) => {
                            if sphere.is_selected {
                                if (sphere.radius + pointer_delta.y as f64) > 0. {
                                    sphere.radius += pointer_delta.y as f64;
                                }
                            } else {
                                // cuboid.is_selected = false;
                            }
                        }
                    }
                }
                None => {}
            }
        }
    }

    pub fn copy_selected(&mut self) {
        for id in self.selected_object_ids() {
            // println!("Object: {}", id);
            let object = self.objects.get(&id.to_string()).unwrap().clone().unwrap();
            // println!("Object: {:?}", object);
            let new_object = match object {
                GravityObject::Cuboid(cuboid) => GravityObject::Cuboid(Cuboid {
                    // name: format!("{}_copy", cuboid.name),
                    id: self.object_counter,
                    ..cuboid
                }),
                GravityObject::Sphere(sphere) => GravityObject::Sphere(Sphere {
                    // name: format!("{}_copy", sphere.name),
                    id: self.object_counter,
                    ..sphere
                }),
            };
            println!("New Object: {:?}", new_object);
            self.add_object(new_object);
        }
    }

    pub fn add_object(&mut self, object: GravityObject) {
        // let object = add_object.create();
        if self.objects.len() < MAX_OBJECTS {
            match object {
                GravityObject::Cuboid(cuboid) => self.objects.insert(cuboid.id.to_string(),
                    Some(GravityObject::Cuboid(Cuboid{id:self.object_counter, ..cuboid}))
                    ),
                GravityObject::Sphere(sphere) => self.objects.insert(sphere.id.to_string(),
                    Some(GravityObject::Sphere(Sphere{id:self.object_counter, ..sphere}))
                    ),
            };
            self.object_counter += 1;
        }
    }
}
// #[derive(serde::Deserialize, serde::Serialize, Debug)]
// pub struct ObjectCollection {
//     objects: BTreeMap<String,Option<GravityObject>>,
// }

// impl Default for ObjectCollection {
//     fn default() -> Self {
//         let mut objects = BTreeMap::new();
//         objects.insert("None".to_string(),None);
//         Self { objects: objects }
//     }
// }

// impl ObjectCollection {
//     fn insert(&mut self, object: GravityObject) {
//         self.objects.insert(object.get_id(), Some(object));
//     }

//     fn remove(&mut self, key: String) {

//     }

//     fn get_mut()
// }

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct AddObject {
    name: String,
    colour: Color32,
    object_type: GravityObject,
    object_number: u128,
}

impl Default for AddObject {
    fn default() -> Self {
        Self {
            name: "Default".to_string(),
            colour: Color32::TEMPORARY_COLOR,
            object_type: GravityObject::Cuboid(Cuboid::default()),
            object_number: 0,
        }
    }
}

impl AddObject {
    // pub fn create(&self) -> GravityObject {
    //     match self.object_type {
    //         GravityObject::Cuboid(_) => GravityObject::Cuboid(Cuboid { id: id, name: self.name.to_string(), colour: self.colour, is_selected: true, ..Default::default() }),
    //         GravityObject::Sphere(_) => GravityObject::Sphere(Sphere { id: id, name: self.name.to_string(), colour: self.colour, is_selected: true, ..Default::default() }),
    //     }
    // }
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct GravityBuilderApp {
    model: Model,
    // selected_object_ids: Vec<(bool,String)>,
    // current_group_ids: Option<BTreeSet<String>>,
    measurement_params: MeasurementParameters,
    #[serde(skip)]
    plot_group: LinkedAxisGroup,
    add_object: AddObject,
    // #[serde(skip)]
    // colours: Box<dyn Iterator<Item = Color32>>,
    // colour: Color32
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct MeasurementParameters {
    measurement_type: DataType,
    x_start: f64,
    x_end: f64,
    n: usize,
    y: f64,
    z: f64,
    gradient: f64,
}

impl Default for MeasurementParameters {
    fn default() -> Self {
        Self {
            measurement_type: DataType::Gz,
            x_start: -10.,
            x_end: 10.,
            n: 200,
            y: 0.,
            z: 0.25,
            gradient: 0.,
        }
    }
}

impl MeasurementParameters {
    pub fn points(&self) -> Array2<f64> {
        let x: Array1<f64> = ndarray::Array::linspace(self.x_start, self.x_end, self.n);
        let mut points: Array2<f64> = ndarray::Array::zeros((self.n, 3));
        for i in 0..x.len() {
            points[[i, 0]] = x[i];
            points[[i, 1]] = self.y;
            points[[i, 2]] = self.z + self.gradient * x[i];
        }
        points * 1.0001
    }
}

impl Default for GravityBuilderApp {
    fn default() -> Self {
        Self {
            model: Model::default(),
            measurement_params: MeasurementParameters::default(),
            // selected_object_ids: Vec::new(),
            // current_group_ids: None,
            plot_group: LinkedAxisGroup::new(true, false),
            add_object: AddObject::default(),
            // colours: Box::new(vec![Color32::RED,Color32::BLUE,Color32::YELLOW,Color32::GREEN,Color32::BROWN].into_iter().cycle())
            // colour: Color32::TEMPORARY_COLOR
        }
    }
}

impl GravityBuilderApp {
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

impl eframe::App for GravityBuilderApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self {
            model,
            measurement_params,
            add_object,
            plot_group,
        } = self;

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
            });
            ui.radio_value(
                &mut add_object.object_type,
                GravityObject::Cuboid(Cuboid::default()),
                "Cuboid".to_string(),
            );
            ui.radio_value(
                &mut add_object.object_type,
                GravityObject::Sphere(Sphere::default()),
                "Sphere".to_string(),
            );
            ui.text_edit_singleline(&mut add_object.name);
            ui.color_edit_button_srgba(&mut add_object.colour);
            if ui.button("Create object").clicked() {
                let object = match add_object.object_type {
                    GravityObject::Cuboid(_) => GravityObject::Cuboid(Cuboid { id: model.object_counter, name: add_object.name.to_string(), colour: add_object.colour, is_selected: true, ..Default::default() }),
                    GravityObject::Sphere(_) => GravityObject::Sphere(Sphere { id: model.object_counter, name: add_object.name.to_string(), colour: add_object.colour, is_selected: true, ..Default::default() }),
                };
                model.add_object(object);
            }
            if ui.button("Remove objects").clicked() {
                let mut ids_to_delete: Vec<String> = vec![];
                for (id, object) in model.objects.iter_mut() {
                    match object {
                        Some(obj) => {
                            if obj.is_selected() {
                                ids_to_delete.push(id.to_string());
                                // ids_to_delete.push(id)
                            }
                        }
                        None => {}
                    }
                }
                for id in ids_to_delete {
                    model.objects.remove(&id.to_string());
                }
            }
            egui::ScrollArea::vertical().show(ui, |ui| {
                for (_, object) in model.objects.iter_mut() {
                    match object {
                        Some(obj) => match obj {
                            GravityObject::Cuboid(cuboid) => {
                                ui.checkbox(&mut cuboid.is_selected, format!("{}: {}",cuboid.id, cuboid.name.to_string()));
                            }
                            GravityObject::Sphere(sphere) => {
                                ui.checkbox(&mut sphere.is_selected, format!("{}: {}",sphere.id, sphere.name.to_string()));
                            }
                        },
                        None => {}
                    }
                }
            });

            if ui.button("print model").clicked() {
                println!("{:?}", model);
            }

            if model.number_objects_selected() == 1 {
                for (_, object) in model.objects.iter_mut() {
                    match object {
                        Some(obj) => match obj {
                            GravityObject::Cuboid(cuboid) => {
                                if cuboid.is_selected {
                                    cuboid.egui_input(ui)
                                }
                            }
                            GravityObject::Sphere(sphere) => {
                                if sphere.is_selected {
                                    sphere.egui_input(ui)
                                }
                            }
                        },
                        None => {}
                    }
                }
            } else if model.number_objects_selected() > 1 {
                // egui::CollapsingHeader::new("position")
                //     .show(ui, |ui| {
                //     ui.label("x centroid");
                //     ui.add(egui::Slider::new(&mut self.x_centroid, -50.0..=50.0).text("m"));

                //     ui.label("y centroid");
                //     ui.add(egui::Slider::new(&mut self.y_centroid, -50.0..=50.0).text("m"));

                //     ui.label("z centroid");
                //     ui.add(egui::Slider::new(&mut self.z_centroid, -25.0..=25.0).text("m"));
                // });
            }

            ui.separator();
            ui.horizontal(|ui| {
                ui.heading("Measurement Settings");
                if ui.button("reset").clicked() {
                    *measurement_params = MeasurementParameters::default();
                }
            });
            egui::CollapsingHeader::new("measurement").show(ui, |ui| {
                egui::ComboBox::from_label("data type")
                    .selected_text(format!("{:?}", measurement_params.measurement_type))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut measurement_params.measurement_type,
                            DataType::Gx,
                            "gx",
                        );
                        ui.selectable_value(
                            &mut measurement_params.measurement_type,
                            DataType::Gy,
                            "gy",
                        );
                        ui.selectable_value(
                            &mut measurement_params.measurement_type,
                            DataType::Gz,
                            "gz",
                        );
                        ui.selectable_value(
                            &mut measurement_params.measurement_type,
                            DataType::Gxx,
                            "gxx",
                        );
                        ui.selectable_value(
                            &mut measurement_params.measurement_type,
                            DataType::Gxy,
                            "gxy",
                        );
                        ui.selectable_value(
                            &mut measurement_params.measurement_type,
                            DataType::Gxz,
                            "gxz",
                        );
                        ui.selectable_value(
                            &mut measurement_params.measurement_type,
                            DataType::Gyy,
                            "gyy",
                        );
                        ui.selectable_value(
                            &mut measurement_params.measurement_type,
                            DataType::Gyz,
                            "gyz",
                        );
                        ui.selectable_value(
                            &mut measurement_params.measurement_type,
                            DataType::Gzz,
                            "gzz",
                        );
                    });
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
                ui.add(egui::Slider::new(
                    &mut measurement_params.gradient,
                    -0.5..=0.5,
                ));
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
            let x = measurement_points.index_axis(Axis(1), 0);
            // Associate colour with a cuboid. Also give cuboid a name id.
            let data_plot = Plot::new("gravity")
                .view_aspect(2.0)
                .link_axis(plot_group.clone())
                .include_x(-10.)
                .include_x(10.)
                .include_y(0.)
                .width(720.)
                .auto_bounds_x()
                .auto_bounds_y()
                .legend(Legend::default());

            let edit_mode = ctx.input().key_down(Key::M) || ctx.input().key_down(Key::L);

            let model_plot_xz = Plot::new("underground_xz")
                .view_aspect(2.0)
                .data_aspect(1.0)
                .link_axis(plot_group.clone())
                .include_x(-10.)
                .include_x(10.)
                .include_y(2.)
                .include_y(-10.)
                .width(720.)
                .legend(Legend::default())
                .auto_bounds_x()
                .auto_bounds_y()
                .allow_boxed_zoom(if edit_mode { false } else { true })
                .allow_drag(if edit_mode { false } else { true });

            // let model_plot_yz = Plot::new("underground_yz")
            //     .view_aspect(2.0)
            //     .data_aspect(1.0)
            //     .link_axis(groups[1].clone())
            //     .include_x(-10.)
            //     .include_x(10.)
            //     .include_y(2.)
            //     .include_y(-10.)
            //     .width(720.)
            //     .legend(Legend::default())
            //     .allow_boxed_zoom(if edit_mode {false} else {true})
            //     .allow_drag(if edit_mode {false} else {true});

            // let model_plot_xy = Plot::new("underground_xy")
            //     .view_aspect(2.0)
            //     .data_aspect(1.0)
            //     // .link_axis(groups[1].clone())
            //     .include_x(-10.)
            //     .include_x(10.)
            //     .include_y(2.)
            //     .include_y(-10.)
            //     .width(720.)
            //     .legend(Legend::default())
            //     .allow_boxed_zoom(if edit_mode {false} else {true})
            //     .allow_drag(if edit_mode {false} else {true});

            let mut data_total: Array1<f64> = Array1::zeros(measurement_points.len_of(Axis(0)));
            ui.horizontal(|ui| {
                data_plot.show(ui, |plot_ui| {
                    for (key, object) in model.objects.iter() {
                        match object {
                            Some(obj) => match obj {
                                GravityObject::Cuboid(cuboid) => {
                                    let data = cuboid.calculate(
                                        &measurement_params.measurement_type,
                                        &measurement_points,
                                    );
                                    let data_2d: Vec<_> = x
                                        .into_iter()
                                        .zip(data.iter())
                                        .map(|(x, val)| [*x, *val])
                                        .collect();
                                    let line = Line::new(data_2d);
                                    plot_ui.line(
                                        line.name(key)
                                            .color(cuboid.colour)
                                            .highlight(cuboid.is_selected),
                                    );
                                    data_total = data_total + &data;
                                }
                                GravityObject::Sphere(sphere) => {
                                    let data = sphere.calculate(
                                        &measurement_params.measurement_type,
                                        &measurement_points,
                                    );
                                    let data_2d: Vec<_> = x
                                        .into_iter()
                                        .zip(data.iter())
                                        .map(|(x, val)| [*x, *val])
                                        .collect();
                                    let line = Line::new(data_2d);
                                    // let line = sphere.egui_data_plot_line(data);
                                    plot_ui.line(
                                        line.name(key)
                                            .color(sphere.colour)
                                            .highlight(sphere.is_selected),
                                    );
                                    data_total = data_total + &data;
                                }
                            },
                            None => {}
                        };
                    }
                    let data_2d: Vec<_> = x
                        .into_iter()
                        .zip(data_total.into_iter())
                        .map(|(x, val)| [*x, val])
                        .collect();
                    let data_total_line = Line::new(data_2d);
                    plot_ui.line(
                        data_total_line
                            .name("Combined")
                            .color(Color32::WHITE)
                            .style(LineStyle::dashed_loose()),
                    );
                });
            });
            ui.separator();

            ui.horizontal(|ui| {
                let plot_response = model_plot_xz
                    .show(ui, |plot_ui| {
                        let plot_points: Vec<[f64; 2]> = measurement_points
                            .index_axis(Axis(1), 0)
                            .into_iter()
                            .zip(measurement_points.index_axis(Axis(1), 2).into_iter())
                            .map(|(x, z)| [*x, *z])
                            .collect();
                        plot_ui.points(
                            Points::new(plot_points)
                                .name("Points")
                                .color(Color32::WHITE),
                        );

                        for (id, object) in model.objects.iter() {
                            match object {
                                Some(obj) => match obj {
                                    GravityObject::Cuboid(cuboid) => {
                                        let polygon =
                                            Polygon::new(PlotPoints::from(cuboid.vertices_xz()));
                                        plot_ui.polygon(
                                            polygon
                                                .name(id)
                                                .color(cuboid.colour)
                                                .highlight(cuboid.is_selected),
                                        );
                                    }
                                    GravityObject::Sphere(sphere) => {
                                        let polygon =
                                            Polygon::new(PlotPoints::from_parametric_callback(
                                                |t| {
                                                    (
                                                        sphere.x_centroid + sphere.radius * t.sin(),
                                                        sphere.z_centroid + sphere.radius * t.cos(),
                                                    )
                                                },
                                                0.0..TAU,
                                                100,
                                            ));
                                        plot_ui.polygon(
                                            polygon
                                                .name(id)
                                                .color(sphere.colour)
                                                .highlight(sphere.is_selected),
                                        );
                                    }
                                },
                                None => {}
                            }
                        }

                        // if ctx.input().key_down(Key::L) && plot_ui.plot_hovered() {
                        //     let drag_delta = plot_ui.pointer_coordinate_drag_delta();
                        //     if !cuboids.is_empty() {
                        //         let current_cuboid = cuboids.get_mut(current_cuboid_id).unwrap();
                        //         if current_cuboid.x_length + drag_delta.x as f64 > 0. {
                        //             current_cuboid.x_length += drag_delta.x as f64;
                        //         }

                        //         if current_cuboid.z_length + drag_delta.y as f64 > 0. {
                        //             current_cuboid.z_length += drag_delta.y as f64;
                        //         }
                        //     }
                        // }

                        // if ctx.input().key_down(Key::P) && plot_ui.plot_hovered() {
                        //     let position = plot_ui.pointer_coordinate().unwrap_or(PlotPoint {x: 0., y: 0.}).to_vec2();
                        //     if !cuboids.is_empty() {
                        //         let current_cuboid = cuboids.get_mut(current_cuboid_id).unwrap();
                        //         current_cuboid.x_centroid = position.x as f64;
                        //         current_cuboid.z_centroid = position.y as f64;
                        //     }
                        // }
                        if plot_ui.plot_hovered() && plot_ui.plot_clicked()
                        // && ctx.input().modifiers.shift
                        {
                            model.select_by_click_xz(plot_ui);
                        }

                        if plot_ui.plot_hovered()
                            && ctx.input().key_pressed(Key::C)
                            && ctx.input().modifiers.ctrl
                        {
                            model.copy_selected();
                        }

                        if plot_ui.plot_hovered() && ctx.input().key_down(Key::M) {
                            model.translate_selected(plot_ui);
                        }

                        if plot_ui.plot_hovered()
                            && ctx.input().key_down(Key::L)
                            && model.number_objects_selected() == 1
                        {
                            model.scale_selected(plot_ui);
                        }
                    })
                    .response;

                // plot_response.context_menu(|ui| {

                // });
            });
        });
    }
}
