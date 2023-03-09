use std::{
    collections::{BTreeMap, BTreeSet},
    env::current_dir,
    error::Error,
    f64::consts::TAU,
    fs::{self, create_dir, File},
    io::BufReader,
    path::Path,
};

use crate::gravity_objects::{
    Cuboid, DataType, GravityCalc, GravityModelObject, GravityObject, InputUI, Sphere,
};
use egui::{
    plot::{Legend, Line, LineStyle, LinkedAxisGroup, Plot, PlotPoints, PlotUi, Points, Polygon},
    Align2, Color32, Context, Key, Pos2, Sense, Stroke, Style, Ui, Vec2, Visuals,
};
use itertools::izip;
use ndarray::{s, Array1, Array2, Axis};
use ndarray_stats::*;
// use serde::Serialize;

const MAX_OBJECTS: usize = 10;
const PLOT_WIDTH: f32 = 750.;

#[derive(serde::Deserialize, serde::Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum PlotView {
    XY,
    XZ,
    YZ,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Model {
    name: String,
    objects: BTreeMap<String, Option<GravityModelObject>>,
    groups: BTreeMap<String, Option<BTreeSet<String>>>,
    object_counter: u128,
}

impl Default for Model {
    fn default() -> Self {
        let mut objects = BTreeMap::new();
        objects.insert("None".to_string(), None);
        let mut groups: BTreeMap<String, Option<BTreeSet<String>>> = BTreeMap::new();
        groups.insert("None".to_string(), None);
        Self {
            name: "Default".to_string(),
            objects,
            groups,
            object_counter: 0,
        }
    }
}

impl Model {
    pub fn number_objects_selected(&self) -> u128 {
        let mut num_selected = 0;
        for (_, object) in self.objects.iter() {
            match object {
                Some(obj) => {
                    if obj.is_selected {
                        num_selected += 1;
                    }
                }
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
                    if obj.is_selected {
                        selected_object_ids.push(obj.id.to_string());
                    }
                }
                None => {}
            }
        }
        selected_object_ids
    }

    pub fn select_by_click(&mut self, plot_ui: &mut PlotUi, plot_view: &mut PlotView) {
        for (_, object) in self.objects.iter_mut() {
            let pointer_pos = plot_ui.pointer_coordinate().unwrap();
            match object {
                Some(obj) => match &obj.object {
                    GravityObject::Cuboid(cuboid) => {
                        let pos: [f64; 2] = match plot_view {
                            PlotView::XY => [cuboid.x_centroid, cuboid.y_centroid],
                            PlotView::XZ => [cuboid.x_centroid, cuboid.z_centroid],
                            PlotView::YZ => [cuboid.y_centroid, cuboid.z_centroid],
                        };
                        if ((pos[0] - pointer_pos.x as f64).powi(2)
                            + (pos[1] - pointer_pos.y as f64).powi(2))
                        .sqrt()
                            < 0.5
                        {
                            obj.is_selected = !obj.is_selected;
                        }
                    }
                    GravityObject::Sphere(sphere) => {
                        let pos: [f64; 2] = match plot_view {
                            PlotView::XY => [sphere.x_centroid, sphere.y_centroid],
                            PlotView::XZ => [sphere.x_centroid, sphere.z_centroid],
                            PlotView::YZ => [sphere.y_centroid, sphere.z_centroid],
                        };
                        if ((pos[0] - pointer_pos.x as f64).powi(2)
                            + (pos[1] - pointer_pos.y as f64).powi(2))
                        .sqrt()
                            < sphere.radius
                        {
                            obj.is_selected = !obj.is_selected;
                        }
                    }
                },
                None => {}
            }
        }
    }

    pub fn deselect_all(&mut self) {
        for (_, object) in self.objects.iter_mut() {
            match object {
                Some(obj) => obj.is_selected = false,
                None => {}
            }
        }
    }

    fn translate_selected(&mut self, plot_ui: &mut PlotUi, plot_view: &mut PlotView) {
        for (_, object) in self.objects.iter_mut() {
            let pointer_delta = plot_ui.pointer_coordinate_drag_delta();
            match object {
                Some(obj) => match &mut obj.object {
                    GravityObject::Cuboid(cuboid) => {
                        if obj.is_selected {
                            match plot_view {
                                PlotView::XY => {
                                    cuboid.x_centroid += pointer_delta.x as f64;
                                    cuboid.y_centroid += pointer_delta.y as f64;
                                }
                                PlotView::XZ => {
                                    cuboid.x_centroid += pointer_delta.x as f64;
                                    cuboid.z_centroid += pointer_delta.y as f64;
                                }
                                PlotView::YZ => {
                                    cuboid.y_centroid += pointer_delta.x as f64;
                                    cuboid.z_centroid += pointer_delta.y as f64;
                                }
                            };
                        }
                    }
                    GravityObject::Sphere(sphere) => {
                        if obj.is_selected {
                            match plot_view {
                                PlotView::XY => {
                                    sphere.x_centroid += pointer_delta.x as f64;
                                    sphere.y_centroid += pointer_delta.y as f64;
                                }
                                PlotView::XZ => {
                                    sphere.x_centroid += pointer_delta.x as f64;
                                    sphere.z_centroid += pointer_delta.y as f64;
                                }
                                PlotView::YZ => {
                                    sphere.y_centroid += pointer_delta.x as f64;
                                    sphere.z_centroid += pointer_delta.y as f64;
                                }
                            };
                        }
                    }
                },
                None => {}
            }
        }
    }

    fn scale_selected(&mut self, plot_ui: &mut PlotUi, plot_view: &mut PlotView) {
        for (_, object) in self.objects.iter_mut() {
            let pointer_delta = plot_ui.pointer_coordinate_drag_delta();
            match object {
                Some(obj) => match &mut obj.object {
                    GravityObject::Cuboid(cuboid) => {
                        if obj.is_selected {
                            match plot_view {
                                PlotView::XY => {
                                    if (cuboid.x_length + pointer_delta.x as f64) > 0. {
                                        cuboid.x_length += pointer_delta.x as f64;
                                    }
                                    if (cuboid.y_length + pointer_delta.y as f64) > 0. {
                                        cuboid.y_length += pointer_delta.y as f64;
                                    }
                                }
                                PlotView::XZ => {
                                    if (cuboid.x_length + pointer_delta.x as f64) > 0. {
                                        cuboid.x_length += pointer_delta.x as f64;
                                    }
                                    if (cuboid.z_length + pointer_delta.y as f64) > 0. {
                                        cuboid.z_length += pointer_delta.y as f64;
                                    }
                                }
                                PlotView::YZ => {
                                    if (cuboid.y_length + pointer_delta.x as f64) > 0. {
                                        cuboid.y_length += pointer_delta.x as f64;
                                    }
                                    if (cuboid.z_length + pointer_delta.y as f64) > 0. {
                                        cuboid.z_length += pointer_delta.y as f64;
                                    }
                                }
                            };
                        }
                    }
                    GravityObject::Sphere(sphere) => {
                        if obj.is_selected {
                            if (sphere.radius + pointer_delta.y as f64) > 0. {
                                sphere.radius += pointer_delta.y as f64;
                            }
                        }
                    }
                },
                None => {}
            }
        }
    }

    pub fn copy_selected(&mut self) {
        if (self.objects.len() + self.number_objects_selected() as usize) < MAX_OBJECTS {
            for id in self.selected_object_ids() {
                let mut object = self
                    .objects
                    .get_mut(&id.to_string())
                    .unwrap()
                    .as_mut()
                    .unwrap();
                object.is_selected = false;
                let mut new_object = object.clone();
                new_object.id = self.object_counter;
                new_object.is_selected = true;
                match &mut new_object.object {
                    GravityObject::Cuboid(cuboid) => cuboid.z_centroid += 1.,
                    GravityObject::Sphere(sphere) => sphere.z_centroid += 1.,
                }
                self.add_object(new_object);
            }
        }
    }

    pub fn add_object(&mut self, object: GravityModelObject) {
        if self.objects.len() < MAX_OBJECTS {
            self.objects.insert(object.id.to_string(), Some(object));
            self.object_counter += 1;
        }
    }

    pub fn delete_objects(&mut self) {
        let mut ids_to_delete: Vec<String> = vec![];
        for (id, object) in self.objects.iter_mut() {
            match object {
                Some(obj) => {
                    if obj.is_selected {
                        ids_to_delete.push(id.to_string());
                    }
                }
                None => {}
            }
        }
        for id in ids_to_delete {
            self.objects.remove(&id.to_string());
        }
    }

    pub fn save_json(&self) {
        let data = serde_json::to_string(self).unwrap();
        let path = current_dir().unwrap();
        let mut new_path = path.join("models");
        if !new_path.exists() {
            create_dir(&new_path).unwrap();
        }
        new_path.push(self.name.to_string());
        new_path.set_extension("json");
        fs::write(new_path, data).expect("Unable to write file");
    }

    // pub fn load_json<P: AsRef<Path>>(path: P) -> Result<Model, Box<dyn Error>> {
    //     // Open the file in read-only mode with buffer.
    //     let file = File::open(path)?;
    //     let reader = BufReader::new(file);

    //     // Read the JSON contents of the file as an instance of `User`.
    //     let u = serde_json::from_reader(reader)?;

    //     // Return the `User`.
    //     Ok(u)
    // }
}

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

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct GravityBuilderApp {
    model: Model,
    data_params: DataParameters,
    #[serde(skip)]
    plot_group: [LinkedAxisGroup; 2],
    plot_view: PlotView,
    plot_range: [f64; 2],
    add_object: AddObject,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct DataParameters {
    data_type: DataType,
    x_start: f64,
    x_end: f64,
    x_n: usize,
    x_y: f64,
    x_gradient: f64,
    y_start: f64,
    y_end: f64,
    y_n: usize,
    y_x: f64,
    y_gradient: f64,
    z: f64,
    grid_x_n: usize,
    grid_y_n: usize,
}

impl Default for DataParameters {
    fn default() -> Self {
        Self {
            data_type: DataType::Gz,
            x_start: -10.,
            x_end: 10.,
            x_n: 200,
            x_y: 0.,
            y_start: -10.,
            y_end: 10.,
            y_n: 200,
            y_x: 0.,
            z: 0.25,
            x_gradient: 0.,
            y_gradient: 0.,
            grid_x_n: 50,
            grid_y_n: 50,
        }
    }
}

impl DataParameters {
    pub fn points_xz(&self) -> Array2<f64> {
        let x: Array1<f64> = ndarray::Array::linspace(self.x_start, self.x_end, self.x_n);
        let mut points: Array2<f64> = ndarray::Array::zeros((self.x_n, 3));
        for i in 0..x.len() {
            points[[i, 0]] = x[i];
            points[[i, 1]] = self.x_y;
            points[[i, 2]] = self.z + self.x_gradient * x[i];
        }
        points * 1.0001
    }

    pub fn points_yz(&self) -> Array2<f64> {
        let y: Array1<f64> = ndarray::Array::linspace(self.y_start, self.y_end, self.y_n);
        let mut points: Array2<f64> = ndarray::Array::zeros((self.y_n, 3));
        for i in 0..y.len() {
            points[[i, 0]] = self.y_x;
            points[[i, 1]] = y[i];
            points[[i, 2]] = self.z + self.y_gradient * y[i];
        }
        points * 1.0001
    }

    pub fn points_xy(&self) -> Array2<f64> {
        let x: Array1<f64> = ndarray::Array::linspace(self.x_start, self.x_end, self.grid_x_n);
        let y: Array1<f64> = ndarray::Array::linspace(self.y_start, self.y_end, self.grid_y_n);
        let mut points: Array2<f64> = ndarray::Array::zeros((x.len() * y.len(), 3));
        let mut idx = 0;
        for i in 0..x.len() {
            for j in 0..y.len() {
                points[[idx, 0]] = x[i];
                points[[idx, 1]] = y[j];
                points[[idx, 2]] = self.z + self.x_gradient * x[i] + self.y_gradient * y[j];
                idx += 1;
            }
        }
        points * 1.0001
    }

    pub fn ui(&mut self, ui: &mut Ui) {
        egui::ComboBox::from_label("Component")
            .selected_text(format!("{:?}", self.data_type))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut self.data_type, DataType::Gx, "gx");
                ui.selectable_value(&mut self.data_type, DataType::Gy, "gy");
                ui.selectable_value(&mut self.data_type, DataType::Gz, "gz");
                ui.selectable_value(&mut self.data_type, DataType::Gxx, "gxx");
                ui.selectable_value(&mut self.data_type, DataType::Gxy, "gxy");
                ui.selectable_value(&mut self.data_type, DataType::Gxz, "gxz");
                ui.selectable_value(&mut self.data_type, DataType::Gyy, "gyy");
                ui.selectable_value(&mut self.data_type, DataType::Gyz, "gyz");
                ui.selectable_value(&mut self.data_type, DataType::Gzz, "gzz");
            });
        ui.collapsing("x", |ui| {
            ui.label("start");
            ui.add(egui::Slider::new(&mut self.x_start, -50.0..=0.).text("m"));
            ui.label("end");
            ui.add(egui::Slider::new(&mut self.x_end, 0.0..=50.).text("m"));
            ui.label("n measurements");
            ui.add(egui::Slider::new(&mut self.x_n, 1..=200));
            ui.label("y");
            ui.add(egui::Slider::new(&mut self.x_y, self.y_start..=self.y_end));
            ui.label("gradient");
        });
        ui.collapsing("y", |ui| {
            ui.add(egui::Slider::new(&mut self.x_gradient, -0.5..=0.5));
            ui.label("start");
            ui.add(egui::Slider::new(&mut self.y_start, -50.0..=0.).text("m"));
            ui.label("end");
            ui.add(egui::Slider::new(&mut self.y_end, 0.0..=50.).text("m"));
            ui.label("n measurements");
            ui.add(egui::Slider::new(&mut self.y_n, 1..=200));
            ui.label("x");
            ui.add(egui::Slider::new(&mut self.y_x, self.x_start..=self.x_end));
            ui.label("gradient");
            ui.add(egui::Slider::new(&mut self.y_gradient, -0.5..=0.5));
        });
        ui.collapsing("z", |ui| {
            ui.add(egui::Slider::new(&mut self.z, -25.0..=25.));
        });
        ui.collapsing("grid", |ui| {
            ui.label("x n measurements");
            ui.add(egui::Slider::new(&mut self.grid_x_n, 1..=50));
            ui.label("y n measurements");
            ui.add(egui::Slider::new(&mut self.grid_y_n, 1..=50));
        });
    }
}

impl Default for GravityBuilderApp {
    fn default() -> Self {
        Self {
            model: Model::default(),
            data_params: DataParameters::default(),
            plot_group: [
                LinkedAxisGroup::new(true, false),
                LinkedAxisGroup::new(true, false),
            ],
            plot_view: PlotView::XZ,
            plot_range: [-10., 10.],
            add_object: AddObject::default(),
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
            data_params,
            add_object,
            plot_view,
            plot_range,
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
                ui.menu_button("Edit", |ui| {
                    if ui.button("Save").clicked() {
                        model.save_json();
                    }
                    if ui.button("Load").clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_file() {
                            if let Ok(loaded_model) = read_model_from_file(path) {
                                *model = loaded_model;
                            }
                        }
                    }
                });
            });
        });

        // egui::SidePanel::left("side_panel").show(ctx, |ui| {
        // ui.horizontal(|ui| {
        //     ui.heading("Model Settings");
        // });
        // ui.text_edit_singleline(&mut model.name);

        // if ui.button("Remove objects").clicked() {
        //     model.delete_objects();
        // }
        // Show list of model objects
        // egui::ScrollArea::vertical().show(ui, |ui| {
        //     for (_, object) in model.objects.iter_mut() {
        //         match object {
        //             Some(obj) => {
        //                 ui.checkbox(
        //                     &mut obj.is_selected,
        //                     format!("{}: {}", obj.id, obj.name.to_string()),
        //                 );
        //             }
        //             None => {}
        //         }
        //     }
        // });

        // ui.separator();
        // ui.horizontal(|ui| {
        //     ui.heading("Data Settings");
        // });

        // ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
        //     ui.hyperlink("https://github.com/adrodgers/gravity_model_webapp");
        //     ui.add(egui::github_link_file!(
        //         "https://github.com/adrodgers/gravity_model_webapp/blob/master/",
        //         "Source code."
        //     ));
        //     egui::warn_if_debug_build(ui);
        //     ui.horizontal(|ui| {
        //         ui.label("powered by ");
        //         ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        //         ui.label(" and ");
        //         ui.hyperlink_to(
        //             "eframe",
        //             "https://github.com/emilk/egui/tree/master/crates/eframe",
        //         );
        //         ui.label(".");
        //     });
        // });
        // });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                let (mut min_x, mut max_x, mut min_y, mut max_y) = (0., 0., 0., 0.);
                egui::Window::new("XZ View").show(ctx, |ui| {
                    [min_x, max_x] = plot(
                        ctx,
                        ui,
                        model,
                        data_params,
                        plot_group,
                        &mut PlotView::XZ,
                        self.plot_range,
                    );
                });
                egui::Window::new("YZ View").show(ctx, |ui| {
                    [min_y, max_y] = plot(
                        ctx,
                        ui,
                        model,
                        data_params,
                        plot_group,
                        &mut PlotView::YZ,
                        self.plot_range,
                    );
                });

                self.plot_range[0] = min_x.min(min_y);
                self.plot_range[1] = max_x.max(max_y);
            });

            egui::Window::new("XY View").show(ctx, |ui| {
                plot_xy(ctx, ui, model, data_params);
                let gradient = colorous::VIRIDIS;
                ui.horizontal_wrapped(|ui| {
                    for i in 1..=10 {
                        let (rect, _response) =
                            ui.allocate_at_least(Vec2 { x: 32., y: 16. }, Sense::hover());
                        let colour = gradient.eval_continuous(1. / i as f64);

                        ui.painter().rect(
                            rect,
                            0.,
                            Color32::from_rgba_premultiplied(colour.r, colour.g, colour.b, 100),
                            Stroke::new(2., Color32::TRANSPARENT),
                        );
                    }
                });
            });

            egui::Window::new("Settings").show(ctx, |ui| {
                egui::CollapsingHeader::new("Data").show(ui, |ui| {
                    data_params.ui(ui);
                });

                egui::CollapsingHeader::new("Model").show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Name: ");
                        ui.text_edit_singleline(&mut model.name);
                    });

                    egui::CollapsingHeader::new("Create Object").show(ui, |ui| {
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
                        ui.horizontal(|ui| {
                            ui.label("Name: ");
                            ui.text_edit_singleline(&mut add_object.name);
                        });
                        ui.horizontal(|ui| {
                            ui.label("Colour: ");
                            ui.color_edit_button_srgba(&mut add_object.colour);
                        });
                        if ui.button("Create").clicked() {
                            let object = match add_object.object_type {
                                GravityObject::Cuboid(_) => GravityObject::Cuboid(Cuboid {
                                    ..Default::default()
                                }),
                                GravityObject::Sphere(_) => GravityObject::Sphere(Sphere {
                                    ..Default::default()
                                }),
                            };
                            model.add_object(GravityModelObject {
                                object,
                                name: add_object.name.to_string(),
                                id: model.object_counter,
                                colour: add_object.colour,
                                is_selected: true,
                            });
                        }
                    });
                });
            });

            if model.number_objects_selected() == 1 {
                for (_, object) in model.objects.iter_mut() {
                    match object {
                        Some(obj) => {
                            if obj.is_selected {
                                egui::Window::new("Selected Object").show(ctx, |ui| {
                                    obj.ui(ui);
                                });
                            }
                        }
                        None => {}
                    }
                }
            }
        });
    }
}

fn read_model_from_file<P: AsRef<Path>>(path: P) -> Result<Model, Box<dyn Error>> {
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    let u = serde_json::from_reader(reader)?;

    // Return the `User`.
    Ok(u)
}

fn plot_xy(ctx: &Context, ui: &mut Ui, model: &mut Model, data_params: &mut DataParameters) {
    let edit_mode = ctx.input().key_down(Key::M) || ctx.input().key_down(Key::L);
    let data_points = data_params.points_xy();
    let plot = Plot::new("xy")
        .view_aspect(1.0)
        // .include_x(-10.)
        // .include_x(10.)
        // .include_y(-10.)
        // .include_y(10.)
        .width(PLOT_WIDTH)
        .auto_bounds_x()
        .auto_bounds_y()
        .allow_boxed_zoom(if edit_mode { false } else { true })
        .allow_drag(if edit_mode { false } else { true });
    // .legend(Legend::default());

    let mut data_total: Array1<f64> = Array1::zeros(data_points.len_of(Axis(0)));

    plot.show(ui, |plot_ui| {
        for (_, object) in model.objects.iter() {
            match object {
                Some(obj) => {
                    let data = match &obj.object {
                        GravityObject::Cuboid(cuboid) => {
                            let edge_lines = cuboid.edge_lines_xy();
                            for edge in edge_lines {
                                plot_ui.line(
                                    edge.name(format!("{}: {}", obj.id, obj.name.to_string()))
                                        .color(obj.colour)
                                        .highlight(obj.is_selected),
                                );
                            }
                            let polygon = Polygon::new(PlotPoints::from_parametric_callback(
                                |t| {
                                    (
                                        cuboid.x_centroid + 0.5 * t.sin(),
                                        cuboid.y_centroid + 0.5 * t.cos(),
                                    )
                                },
                                0.0..TAU,
                                100,
                            ));
                            plot_ui.polygon(
                                polygon
                                    .name(format!("{}: {}", obj.id, obj.name.to_string()))
                                    .style(LineStyle::Dashed { length: 5. })
                                    .fill_alpha(0.)
                                    .color(obj.colour)
                                    .highlight(obj.is_selected),
                            );
                            cuboid.calculate(&data_params.data_type, &data_points)
                        }
                        GravityObject::Sphere(sphere) => {
                            let polygon = Polygon::new(PlotPoints::from_parametric_callback(
                                |t| {
                                    (
                                        sphere.x_centroid + sphere.radius * t.sin(),
                                        sphere.y_centroid + sphere.radius * t.cos(),
                                    )
                                },
                                0.0..TAU,
                                100,
                            ));
                            plot_ui.polygon(
                                polygon
                                    .name(format!("{}: {}", obj.id, obj.name.to_string()))
                                    .color(obj.colour)
                                    .highlight(obj.is_selected),
                            );
                            sphere.calculate(&data_params.data_type, &data_points)
                        }
                    };
                    data_total = data_total + &data;
                }
                None => {}
            };
        }
        let min = data_total.min().unwrap();
        let max = data_total.max().unwrap();

        let line = Line::new(vec![
            [data_params.x_start, data_params.x_y],
            [data_params.x_end, data_params.x_y],
        ]);
        plot_ui.line(line.name("x").color(Color32::WHITE).highlight(true));

        let line = Line::new(vec![
            [data_params.y_x, data_params.y_start],
            [data_params.y_x, data_params.y_end],
        ]);
        plot_ui.line(line.name("y").color(Color32::WHITE).highlight(true));

        let gradient = colorous::VIRIDIS;
        for (x, y, val) in izip!(
            data_points.index_axis(Axis(1), 0),
            data_points.index_axis(Axis(1), 1),
            data_total.clone(),
        ) {
            let val_norm = normalize_range(val, *min, *max);
            // println!(
            //     "{},{},{},{}, {}",
            //     val,
            //     min,
            //     max,
            //     val_norm,
            //     ((1. - val_norm) * 255.) as u8
            // );
            let colour = gradient.eval_continuous(val_norm);

            plot_ui.points(
                Points::new([*x, *y])
                    .radius(5.)
                    .name(format!("{val:.2}"))
                    // .color(Color32::from_rgb(colour.r, colour.g, colour.b)),
                    .color(Color32::from_rgba_premultiplied(
                        colour.r, colour.g, colour.b, 100,
                    )),
            );
        }
        let mut view = PlotView::XY;
        if plot_ui.plot_hovered() && plot_ui.plot_clicked() && !ctx.input().modifiers.shift {
            model.deselect_all();
            model.select_by_click(plot_ui, &mut view);
        } else if plot_ui.plot_hovered() && plot_ui.plot_clicked() && ctx.input().modifiers.shift {
            model.select_by_click(plot_ui, &mut view);
        }
        if plot_ui.plot_hovered() && ctx.input().key_pressed(Key::C) && ctx.input().modifiers.ctrl {
            model.copy_selected();
        }
        if plot_ui.plot_hovered() && ctx.input().key_pressed(Key::Delete) {
            model.delete_objects();
        }
        if plot_ui.plot_hovered() && ctx.input().key_down(Key::M) {
            model.translate_selected(plot_ui, &mut view);
        }
        if plot_ui.plot_hovered()
            && ctx.input().key_down(Key::L)
            && model.number_objects_selected() == 1
        {
            model.scale_selected(plot_ui, &mut view);
        }
    });
}

fn plot(
    ctx: &Context,
    ui: &mut Ui,
    model: &mut Model,
    data_params: &mut DataParameters,
    plot_group: &mut [LinkedAxisGroup; 2],
    plot_view: &mut PlotView,
    plot_range: [f64; 2],
) -> [f64; 2] {
    // The central panel the region left after adding TopPanel's and SidePanel's
    let data_points = match plot_view {
        PlotView::XY => todo!(),
        PlotView::XZ => data_params.points_xz(),
        PlotView::YZ => data_params.points_yz(),
    };
    let pos = match plot_view {
        PlotView::XY => todo!(),
        PlotView::XZ => data_points.index_axis(Axis(1), 0),
        PlotView::YZ => data_points.index_axis(Axis(1), 1),
    };

    let (data_plot_name, model_plot_name) = match plot_view {
        PlotView::XY => todo!(),
        PlotView::XZ => ("data_xz", "model_xz"),
        PlotView::YZ => ("data_yz", "model_yz"),
    };
    let group = match plot_view {
        PlotView::XY => todo!(),
        PlotView::XZ => &plot_group[0],
        PlotView::YZ => &plot_group[1],
    };
    // Associate colour with a cuboid. Also give cuboid a name id.
    let data_plot = Plot::new(data_plot_name)
        .view_aspect(2.0)
        .link_axis(group.clone())
        .include_x(-5.)
        .include_x(5.)
        .include_y(plot_range[0])
        .include_y(plot_range[1])
        .width(PLOT_WIDTH)
        .auto_bounds_x()
        .auto_bounds_y()
        .legend(Legend::default());

    let edit_mode = ctx.input().key_down(Key::M) || ctx.input().key_down(Key::L);

    let model_plot = Plot::new(model_plot_name)
        .view_aspect(2.0)
        .data_aspect(1.0)
        .link_axis(group.clone())
        .include_x(-5.)
        .include_x(5.)
        .include_y(2.)
        .include_y(-5.)
        .width(PLOT_WIDTH)
        .legend(Legend::default())
        .auto_bounds_x()
        .auto_bounds_y()
        .allow_boxed_zoom(if edit_mode { false } else { true })
        .allow_drag(if edit_mode { false } else { true });

    let mut data_total: Array1<f64> = Array1::zeros(data_points.len_of(Axis(0)));
    // let mut data_yz_total: Array1<f64> = Array1::zeros(data_points.len_of(Axis(0)));
    ui.vertical(|ui| {
        data_plot.show(ui, |plot_ui| {
            for (_, object) in model.objects.iter() {
                match object {
                    Some(obj) => {
                        let data = match &obj.object {
                            GravityObject::Cuboid(cuboid) => {
                                cuboid.calculate(&data_params.data_type, &data_points)
                            }
                            GravityObject::Sphere(sphere) => {
                                sphere.calculate(&data_params.data_type, &data_points)
                            }
                        };
                        let data_2d: Vec<_> = pos
                            .into_iter()
                            .zip(data.iter())
                            .map(|(p, val)| [*p, *val])
                            .collect();
                        let line = Line::new(data_2d);
                        plot_ui.line(
                            line.name(format!("{}: {}", obj.id, obj.name.to_string()))
                                .color(obj.colour)
                                .highlight(obj.is_selected),
                        );
                        data_total = &data_total + &data;
                    }
                    None => {}
                };
            }
            let data_2d: Vec<_> = pos
                .into_iter()
                .zip(data_total.iter())
                .map(|(p, val)| [*p, *val])
                .collect();
            let data_total_line = Line::new(data_2d);
            plot_ui.line(
                data_total_line
                    .name("Combined")
                    .color(if ctx.style().visuals == Visuals::light() {
                        Color32::BLACK
                    } else {
                        Color32::WHITE
                    })
                    .style(LineStyle::dashed_loose()),
            );
        });

        let plot_response = model_plot
            .show(ui, |plot_ui| {
                let idx = match plot_view {
                    PlotView::XY => todo!(),
                    PlotView::XZ => 0,
                    PlotView::YZ => 1,
                };
                let plot_points: Vec<[f64; 2]> = data_points
                    .index_axis(Axis(1), idx)
                    .into_iter()
                    .zip(data_points.index_axis(Axis(1), 2).into_iter())
                    .map(|(p, z)| [*p, *z])
                    .collect();
                plot_ui.points(Points::new(plot_points).name("Points").color(
                    if ctx.style().visuals == Visuals::light() {
                        Color32::BLACK
                    } else {
                        Color32::WHITE
                    },
                ));

                for (id, object) in model.objects.iter() {
                    match object {
                        Some(obj) => match obj.object.clone() {
                            GravityObject::Cuboid(cuboid) => {
                                match plot_view {
                                    PlotView::XY => todo!(),
                                    PlotView::XZ => {
                                        let edge_lines = cuboid.edge_lines_xz();
                                        for edge in edge_lines {
                                            plot_ui.line(
                                                edge.name(format!(
                                                    "{}: {}",
                                                    obj.id,
                                                    obj.name.to_string()
                                                ))
                                                .color(obj.colour)
                                                .highlight(obj.is_selected),
                                            );
                                        }
                                        let polygon =
                                            Polygon::new(PlotPoints::from_parametric_callback(
                                                |t| {
                                                    (
                                                        cuboid.x_centroid + 0.5 * t.sin(),
                                                        cuboid.z_centroid + 0.5 * t.cos(),
                                                    )
                                                },
                                                0.0..TAU,
                                                100,
                                            ));
                                        plot_ui.polygon(
                                            polygon
                                                .name(format!(
                                                    "{}: {}",
                                                    obj.id,
                                                    obj.name.to_string()
                                                ))
                                                .style(LineStyle::Dashed { length: 5. })
                                                .fill_alpha(0.)
                                                .color(obj.colour)
                                                .highlight(obj.is_selected),
                                        );
                                    }
                                    PlotView::YZ => {
                                        let edge_lines = cuboid.edge_lines_yz();
                                        for edge in edge_lines {
                                            plot_ui.line(
                                                edge.name(format!(
                                                    "{}: {}",
                                                    obj.id,
                                                    obj.name.to_string()
                                                ))
                                                .color(obj.colour)
                                                .highlight(obj.is_selected),
                                            );
                                        }
                                        let polygon =
                                            Polygon::new(PlotPoints::from_parametric_callback(
                                                |t| {
                                                    (
                                                        cuboid.y_centroid + 0.5 * t.sin(),
                                                        cuboid.z_centroid + 0.5 * t.cos(),
                                                    )
                                                },
                                                0.0..TAU,
                                                100,
                                            ));
                                        plot_ui.polygon(
                                            polygon
                                                .name(format!(
                                                    "{}: {}",
                                                    obj.id,
                                                    obj.name.to_string()
                                                ))
                                                .style(LineStyle::Dashed { length: 5. })
                                                .fill_alpha(0.)
                                                .color(obj.colour)
                                                .highlight(obj.is_selected),
                                        );
                                    }
                                };
                            }
                            GravityObject::Sphere(sphere) => {
                                match plot_view {
                                    PlotView::XY => todo!(),
                                    PlotView::XZ => {
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
                                                .name(format!(
                                                    "{}: {}",
                                                    obj.id,
                                                    obj.name.to_string()
                                                ))
                                                .color(obj.colour)
                                                .highlight(obj.is_selected),
                                        );
                                    }
                                    PlotView::YZ => {
                                        let polygon =
                                            Polygon::new(PlotPoints::from_parametric_callback(
                                                |t| {
                                                    (
                                                        sphere.y_centroid + sphere.radius * t.sin(),
                                                        sphere.z_centroid + sphere.radius * t.cos(),
                                                    )
                                                },
                                                0.0..TAU,
                                                100,
                                            ));
                                        plot_ui.polygon(
                                            polygon
                                                .name(format!(
                                                    "{}: {}",
                                                    obj.id,
                                                    obj.name.to_string()
                                                ))
                                                .color(obj.colour)
                                                .highlight(obj.is_selected),
                                        );
                                    }
                                };
                            }
                        },
                        None => {}
                    }
                }

                if plot_ui.plot_hovered() && plot_ui.plot_clicked() && !ctx.input().modifiers.shift
                {
                    model.deselect_all();
                    model.select_by_click(plot_ui, plot_view);
                } else if plot_ui.plot_hovered()
                    && plot_ui.plot_clicked()
                    && ctx.input().modifiers.shift
                {
                    model.select_by_click(plot_ui, plot_view);
                }

                if plot_ui.plot_hovered()
                    && ctx.input().key_pressed(Key::C)
                    && ctx.input().modifiers.ctrl
                {
                    model.copy_selected();
                }

                if plot_ui.plot_hovered() && ctx.input().key_pressed(Key::Delete) {
                    model.delete_objects();
                }

                if plot_ui.plot_hovered() && ctx.input().key_down(Key::M) {
                    model.translate_selected(plot_ui, plot_view);
                }

                if plot_ui.plot_hovered()
                    && ctx.input().key_down(Key::L)
                    && model.number_objects_selected() == 1
                {
                    model.scale_selected(plot_ui, plot_view);
                }
            })
            .response;
    });
    [*data_total.min().unwrap(), *data_total.max().unwrap()]
}

pub fn normalize_range(value: f64, min: f64, max: f64) -> f64 {
    (value - min) / (max - min)
}
