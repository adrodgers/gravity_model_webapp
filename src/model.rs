use crate::gravity_objects;
use crate::plot::PlotView;
use egui::plot::PlotUi;
use gravity_objects::{Cuboid, GravityModelObject, GravityObject, Sphere};
use std::{
    collections::{BTreeMap, BTreeSet},
    env::current_dir,
    error::Error,
    f64::consts::TAU,
    fs::{self, create_dir, File},
    io::BufReader,
    path::Path,
};

const MAX_OBJECTS: usize = 10;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Model {
    pub name: String,
    pub objects: BTreeMap<String, Option<GravityModelObject>>,
    pub groups: BTreeMap<String, Option<BTreeSet<String>>>,
    pub object_counter: u128,
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

    pub fn translate_selected(&mut self, plot_ui: &mut PlotUi, plot_view: &mut PlotView) {
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

    pub fn scale_selected(&mut self, plot_ui: &mut PlotUi, plot_view: &mut PlotView) {
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
