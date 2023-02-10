use egui::{Color32, Ui};
use ndarray::prelude::*;
use std::f64::consts::PI;
use std::fmt;

// use crate::app::MeasurementParameters;
// use assert_approx_eq::assert_approx_eq;
// use ndarray::arr2;
// use rayon::prelude::*;
// use std::time::Instant;
// #![feature(const_trait_impl)]

const G: f64 = 6.674e-11;

// pub struct Group {
//     id: String,
//     members: HashSet<String>
// }

// impl Default for Group {
//     fn default() -> Self {
//         Self { id: String::new(), members: HashSet::new()}
//     }
// }

/// Required methods to define a new gravity object, to be used within a gravity model.
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq)]
pub enum GravityObject {
    Cuboid(Cuboid),
    Sphere(Sphere),
}

impl GravityInfo for GravityObject {
    fn get_density(&self) -> f64 {
        match self {
            GravityObject::Cuboid(cuboid) => cuboid.density,
            GravityObject::Sphere(sphere) => sphere.density,
        }
    }

    fn get_name(&self) -> String {
        match self {
            GravityObject::Cuboid(cuboid) => cuboid.name.to_string(),
            GravityObject::Sphere(sphere) => sphere.name.to_string(),
        }
    }

    fn is_selected(&self) -> bool {
        match self {
            GravityObject::Cuboid(cuboid) => cuboid.is_selected,
            GravityObject::Sphere(sphere) => sphere.is_selected,
        }
    }
}

// pub enum GravityObject {
//     Cuboid,
//     Sphere
// }

pub trait GravityInfo {
    // fn volume(&self) -> f64;

    // fn mass(&self) -> f64;

    fn get_density(&self) -> f64;

    fn get_name(&self) -> String;

    fn is_selected(&self) -> bool;
}

pub trait GravityCalc {
    fn calculate(&self, data_type: &DataType, measurement_points: &Array2<f64>) -> Array1<f64> {
        let mut data: Array1<f64> = Array1::zeros(measurement_points.len_of(Axis(0)));
        // let mut data = measurement_data.to_owned();
        let scaling = match data_type {
            DataType::Gx | DataType::Gy | DataType::Gz => -1E8,
            _ => 1E9,
        };
        match data_type {
            DataType::Gx => {
                for (i, point) in measurement_points.axis_iter(Axis(0)).enumerate() {
                    data[i] += self.gx(&point.to_owned())
                }
            }
            DataType::Gy => {
                for (i, point) in measurement_points.axis_iter(Axis(0)).enumerate() {
                    data[i] += self.gy(&point.to_owned())
                }
            }
            DataType::Gz => {
                for (i, point) in measurement_points.axis_iter(Axis(0)).enumerate() {
                    data[i] += self.gz(&point.to_owned())
                }
            }
            DataType::Gxx => {
                for (i, point) in measurement_points.axis_iter(Axis(0)).enumerate() {
                    data[i] += self.gxx(&point.to_owned())
                }
            }
            DataType::Gxy => {
                for (i, point) in measurement_points.axis_iter(Axis(0)).enumerate() {
                    data[i] += self.gxy(&point.to_owned())
                }
            }
            DataType::Gxz => {
                for (i, point) in measurement_points.axis_iter(Axis(0)).enumerate() {
                    data[i] += self.gxz(&point.to_owned())
                }
            }
            DataType::Gyy => {
                for (i, point) in measurement_points.axis_iter(Axis(0)).enumerate() {
                    data[i] += self.gyy(&point.to_owned())
                }
            }
            DataType::Gyz => {
                for (i, point) in measurement_points.axis_iter(Axis(0)).enumerate() {
                    data[i] += self.gyz(&point.to_owned())
                }
            }
            DataType::Gzz => {
                for (i, point) in measurement_points.axis_iter(Axis(0)).enumerate() {
                    data[i] += self.gzz(&point.to_owned())
                }
            }
        }
        data * scaling
    }

    fn gx(&self, position: &Array1<f64>) -> f64;

    fn gy(&self, position: &Array1<f64>) -> f64;

    fn gz(&self, position: &Array1<f64>) -> f64;

    fn gxx(&self, position: &Array1<f64>) -> f64;

    fn gxy(&self, position: &Array1<f64>) -> f64;

    fn gxz(&self, position: &Array1<f64>) -> f64;

    fn gyy(&self, position: &Array1<f64>) -> f64;

    fn gyz(&self, position: &Array1<f64>) -> f64;

    fn gzz(&self, position: &Array1<f64>) -> f64;

    fn centre(&self) -> Array1<f64>;

    fn volume(&self) -> f64;

    fn mass(&self) -> f64;

    // fn get_density(&self) -> f64;

    // fn get_id(&self) -> String;
}

#[derive(serde::Deserialize, serde::Serialize, Debug, PartialEq, Eq, Clone, Copy)]
pub enum DataType {
    Gx,
    Gy,
    Gz,
    Gxx,
    Gxy,
    Gxz,
    Gyy,
    Gyz,
    Gzz,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq)]
pub struct Sphere {
    pub x_centroid: f64,
    pub y_centroid: f64,
    pub z_centroid: f64,
    pub radius: f64,
    pub density: f64,
    pub name: String,
    pub id: u128,
    pub colour: Color32,
    pub is_selected: bool,
}

impl Default for Sphere {
    fn default() -> Self {
        Self {
            x_centroid: 0.,
            y_centroid: 0.,
            z_centroid: -1.,
            radius: 1.,
            density: -2000.,
            colour: Color32::RED,
            name: "Default".to_string(),
            id: 0,
            is_selected: false,
        }
    }
}

impl GravityCalc for Sphere {
    fn gx(&self, position: &Array1<f64>) -> f64 {
        let p_dash: Array1<f64> = position * (1. + 1e-7) - self.centre();
        // Only fetch relevant values once
        let r = p_dash.mapv(|p_dash| p_dash.powi(2)).sum();
        let x = p_dash[0];
        let constant = -(4. / 3.) * PI * G * self.radius.powi(3) * self.density;
        constant * x / r.powf(3. / 2.)
    }

    fn gy(&self, position: &Array1<f64>) -> f64 {
        let p_dash: Array1<f64> = position * (1. + 1e-7) - self.centre();
        // Only fetch relevant values once
        let r = p_dash.mapv(|p_dash| p_dash.powi(2)).sum();
        let y = p_dash[1];
        let constant = -(4. / 3.) * PI * G * self.radius.powi(3) * self.density;
        constant * y / r.powf(3. / 2.)
    }

    fn gz(&self, position: &Array1<f64>) -> f64 {
        let p_dash: Array1<f64> = position * (1. + 1e-7) - self.centre();
        // Only fetch relevant values once
        let r = p_dash.mapv(|p_dash| p_dash.powi(2)).sum();
        let z = p_dash[2];
        let constant = -(4. / 3.) * PI * G * self.radius.powi(3) * self.density;
        constant * z / r.powf(3. / 2.)
    }

    fn gxx(&self, position: &Array1<f64>) -> f64 {
        let p_dash: Array1<f64> = position * (1. + 1e-7) - self.centre();
        // Only fetch relevant values once
        let r = p_dash.mapv(|p_dash| p_dash.powi(2)).sum();
        let x = p_dash[0];
        // let y = p_dash[1];
        // let z = p_dash[2];
        let constant = -(4. / 3.) * PI * G * self.radius.powi(3) * self.density;
        (constant / r.powf(3. / 2.)) * (1. - ((3. * x.powi(2)) / r))
    }

    fn gxy(&self, position: &Array1<f64>) -> f64 {
        let p_dash: Array1<f64> = position * (1. + 1e-7) - self.centre();
        // Only fetch relevant values once
        let r = p_dash.mapv(|p_dash| p_dash.powi(2)).sum();
        // let x = p_dash[0];
        let y = p_dash[1];
        let z = p_dash[2];
        let constant = -(4. / 3.) * PI * G * self.radius.powi(3) * self.density;
        (-3. * constant * y * z) / (r.powf(5. / 2.))
    }

    fn gxz(&self, position: &Array1<f64>) -> f64 {
        let p_dash: Array1<f64> = position * (1. + 1e-7) - self.centre();
        // Only fetch relevant values once
        let r = p_dash.mapv(|p_dash| p_dash.powi(2)).sum();
        let x = p_dash[0];
        // let y = p_dash[1];
        let z = p_dash[2];
        let constant = -(4. / 3.) * PI * G * self.radius.powi(3) * self.density;
        (-3. * constant * x * z) / (r.powf(5. / 2.))
    }

    fn gyy(&self, position: &Array1<f64>) -> f64 {
        let p_dash: Array1<f64> = position * (1. + 1e-7) - self.centre();
        // Only fetch relevant values once
        let r = p_dash.mapv(|p_dash| p_dash.powi(2)).sum();
        // let x = p_dash[0];
        let y = p_dash[1];
        // let z = p_dash[2];
        let constant = -(4. / 3.) * PI * G * self.radius.powi(3) * self.density;
        (constant / r.powf(3. / 2.)) * (1. - ((3. * y.powi(2)) / r))
    }

    fn gyz(&self, position: &Array1<f64>) -> f64 {
        let p_dash: Array1<f64> = position * (1. + 1e-7) - self.centre();
        // Only fetch relevant values once
        let r = p_dash.mapv(|p_dash| p_dash.powi(2)).sum();
        let x = p_dash[0];
        let y = p_dash[1];
        let z = p_dash[2];
        let constant = -(4. / 3.) * PI * G * self.radius.powi(3) * self.density;
        (-3. * constant * y * z) / (r.powf(5. / 2.))
    }

    fn gzz(&self, position: &Array1<f64>) -> f64 {
        let p_dash: Array1<f64> = position * (1. + 1e-7) - self.centre();
        // Only fetch relevant values once
        let r = p_dash.mapv(|p_dash| p_dash.powi(2)).sum();
        // let x = p_dash[0];
        // let y = p_dash[1];
        let z = p_dash[2];
        let constant = -(4. / 3.) * PI * G * self.radius.powi(3) * self.density;
        (constant / r.powf(3. / 2.)) * (1. - ((3. * z.powi(2)) / r))
    }

    fn volume(&self) -> f64 {
        (4. / 3.) * PI * self.radius.powi(3)
    }

    fn mass(&self) -> f64 {
        self.density * self.volume()
    }

    fn centre(&self) -> Array1<f64> {
        Array1::from(vec![self.x_centroid, self.y_centroid, self.z_centroid])
    }

    // fn get_density(&self) -> f64 {
    //     self.density
    // }

    // fn get_id(&self) -> String {
    //     self.id
    // }
}

impl Sphere {
    pub fn egui_input(&mut self, ui: &mut Ui) {
        egui::CollapsingHeader::new("position").show(ui, |ui| {
            ui.label("x centroid");
            ui.add(egui::Slider::new(&mut self.x_centroid, -50.0..=50.0).text("m"));

            ui.label("y centroid");
            ui.add(egui::Slider::new(&mut self.y_centroid, -50.0..=50.0).text("m"));

            ui.label("z centroid");
            ui.add(egui::Slider::new(&mut self.z_centroid, -25.0..=25.0).text("m"));
        });
        egui::CollapsingHeader::new("volume").show(ui, |ui| {
            ui.label("radius");
            ui.add(egui::Slider::new(&mut self.radius, 0.1..=100.0).text("m"));

            // ui.separator();
            // ui.label(format!("Volume: {}",self.volume()));
        });
        egui::CollapsingHeader::new("density").show(ui, |ui| {
            ui.add(egui::Slider::new(&mut self.density, -3000.0..=22590.).text("kg/m^3"));
            if ui.button("soil void").clicked() {
                self.density = -1800.;
            }
            if ui.button("concrete").clicked() {
                self.density = 2000.;
            }
            if ui.button("lead").clicked() {
                self.density = 11340.;
            }
            if ui.button("tungsten").clicked() {
                self.density = 19300.;
            }
            // ui.separator();
            // ui.label(format!("Mass: {}",self.mass()));
        });
    }
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq)]
pub struct Cuboid {
    // pub vertices: Array2<f64>,
    pub x_length: f64,
    pub y_length: f64,
    pub z_length: f64,
    pub x_centroid: f64,
    pub y_centroid: f64,
    pub z_centroid: f64,
    pub density: f64,
    pub name: String,
    pub id: u128,
    pub colour: Color32,
    pub is_selected: bool,
}

impl Default for Cuboid {
    fn default() -> Self {
        //     let vertices: Array2<f64> = array![
        //     [-1., -1., -1.],
        //     [-1., -1., 1.],
        //     [-1., 1., 1.],
        //     [-1., 1., -1.],
        //     [1., -1., -1.],
        //     [1., 1., -1.],
        //     [1., 1., 1.],
        //     [1., -1., 1.]
        // ];
        Self {
            x_length: 1.,
            y_length: 1.,
            z_length: 1.,
            x_centroid: 0.,
            y_centroid: 0.,
            z_centroid: -1.,
            density: -2000.,
            colour: Color32::RED,
            name: "Default".to_string(),
            id: 0,
            is_selected: false,
        }
    }
}

impl GravityCalc for Cuboid {
    fn gx(&self, position: &Array1<f64>) -> f64 {
        let mut gx = 0.;
        let verts = self.vertices();
        for i in 0..8 {
            let p_dash: Array1<f64> =
                position * (1. + 1e-7) - verts.index_axis(Axis(0), i).to_owned();
            // Only fetch relevant values once
            let r = p_dash.mapv(|p_dash| p_dash.powi(2)).sum().sqrt();
            let sign = Cuboid::index_order()[i];
            let x = p_dash[0];
            let y = p_dash[1];
            let z = p_dash[2];
            gx +=
                sign * ((y * (r + z).ln()) + (z * (r + y).ln()) - (x * ((y * z) / (r * x)).atan()));
        }
        gx * G * self.density
    }

    fn gy(&self, position: &Array1<f64>) -> f64 {
        let mut gy = 0.;
        let verts = self.vertices();
        for i in 0..8 {
            let p_dash: Array1<f64> =
                position * (1. + 1e-7) - verts.index_axis(Axis(0), i).to_owned();
            // Only fetch relevant values once
            let r = p_dash.mapv(|p_dash| p_dash.powi(2)).sum().sqrt();
            let sign = Cuboid::index_order()[i];
            let x = p_dash[0];
            let y = p_dash[1];
            let z = p_dash[2];
            gy +=
                sign * ((z * (r + x).ln()) + (x * (r + z).ln()) - (y * ((x * z) / (r * y)).atan()));
        }
        gy * G * self.density
    }

    fn gz(&self, position: &Array1<f64>) -> f64 {
        let mut gz = 0.;
        let verts = self.vertices();
        for i in 0..8 {
            let p_dash: Array1<f64> =
                position * (1. + 1e-7) - verts.index_axis(Axis(0), i).to_owned();
            // Only fetch relevant values once
            let r = p_dash.mapv(|p_dash| p_dash.powi(2)).sum().sqrt();
            let sign = Cuboid::index_order()[i];
            let x = p_dash[0];
            let y = p_dash[1];
            let z = p_dash[2];
            gz +=
                sign * ((x * (r + y).ln()) + (y * (r + x).ln()) - (z * ((x * y) / (r * z)).atan()));
        }
        gz * G * self.density
    }

    fn gxx(&self, position: &Array1<f64>) -> f64 {
        let mut gxx = 0.;
        let verts = self.vertices();
        for i in 0..8 {
            let p_dash: Array1<f64> =
                position * (1. + 1e-7) - verts.index_axis(Axis(0), i).to_owned();
            // Only fetch relevant values once
            let r = p_dash.mapv(|p_dash| p_dash.powi(2)).sum().sqrt();
            let sign = Cuboid::index_order()[i];
            let x = p_dash[0];
            let y = p_dash[1];
            let z = p_dash[2];
            gxx += sign * -((y * z) / (r * x)).atan()
        }
        gxx * G * self.density
    }

    fn gxy(&self, position: &Array1<f64>) -> f64 {
        let mut gxy = 0.;
        let verts = self.vertices();
        for i in 0..8 {
            let p_dash: Array1<f64> =
                position * (1. + 1e-7) - verts.index_axis(Axis(0), i).to_owned();
            // Only fetch relevant values once
            let r = p_dash.mapv(|p_dash| p_dash.powi(2)).sum().sqrt();
            let sign = Cuboid::index_order()[i];
            // let x = p_dash[0];
            // let y = p_dash[1];
            let z = p_dash[2];
            gxy += sign * (r + z).ln()
        }
        gxy * G * self.density
    }

    fn gxz(&self, position: &Array1<f64>) -> f64 {
        let mut gxz = 0.;
        let verts = self.vertices();
        for i in 0..8 {
            let p_dash: Array1<f64> =
                position * (1. + 1e-7) - verts.index_axis(Axis(0), i).to_owned();
            // Only fetch relevant values once
            let r = p_dash.mapv(|p_dash| p_dash.powi(2)).sum().sqrt();
            let sign = Cuboid::index_order()[i];
            // let x = p_dash[0];
            let y = p_dash[1];
            // let z = p_dash[2];
            gxz += sign * (r + y).ln()
        }
        gxz * G * self.density
    }

    fn gyy(&self, position: &Array1<f64>) -> f64 {
        let mut gyy = 0.;
        let verts = self.vertices();
        for i in 0..8 {
            let p_dash: Array1<f64> =
                position * (1. + 1e-7) - verts.index_axis(Axis(0), i).to_owned();
            // Only fetch relevant values once
            let r = p_dash.mapv(|p_dash| p_dash.powi(2)).sum().sqrt();
            let sign = Cuboid::index_order()[i];
            let x = p_dash[0];
            let y = p_dash[1];
            let z = p_dash[2];
            gyy += sign * -((x * z) / (r * y)).atan()
        }
        gyy * G * self.density
    }

    fn gyz(&self, position: &Array1<f64>) -> f64 {
        let mut gyz = 0.;
        let verts = self.vertices();
        for i in 0..8 {
            let p_dash: Array1<f64> =
                position * (1. + 1e-7) - verts.index_axis(Axis(0), i).to_owned();
            // Only fetch relevant values once
            let r = p_dash.mapv(|p_dash| p_dash.powi(2)).sum().sqrt();
            let sign = Cuboid::index_order()[i];
            let x = p_dash[0];
            // let y = p_dash[1];
            // let z = p_dash[2];
            gyz += sign * (r + x).ln()
        }
        gyz * G * self.density
    }

    fn gzz(&self, position: &Array1<f64>) -> f64 {
        let mut gzz = 0.;
        let verts = self.vertices();
        for i in 0..8 {
            let p_dash: Array1<f64> =
                position * (1. + 1e-7) - verts.index_axis(Axis(0), i).to_owned();
            // Only fetch relevant values once
            let r = p_dash.mapv(|p_dash| p_dash.powi(2)).sum().sqrt();
            let sign = Cuboid::index_order()[i];
            let x = p_dash[0];
            let y = p_dash[1];
            let z = p_dash[2];
            gzz += sign * -((y * x) / (r * z)).atan()
        }
        gzz * G * self.density
    }

    fn volume(&self) -> f64 {
        self.x_length * self.y_length * self.z_length
    }

    fn mass(&self) -> f64 {
        self.density * self.volume()
    }

    fn centre(&self) -> Array1<f64> {
        Array1::from(vec![self.x_centroid, self.y_centroid, self.z_centroid])
    }

    // fn get_density(&self) -> f64 {
    //     self.density
    // }

    // fn get_id(&self) -> String {
    //     self.id
    // }
}

impl Cuboid {
    pub fn new_from_lengths(
        x_length: f64,
        y_length: f64,
        z_length: f64,
        x_centroid: f64,
        y_centroid: f64,
        z_centroid: f64,
        density: f64,
        id: u128,
        name: String,
        colour: Color32,
        is_selected: bool,
    ) -> Cuboid {
        Cuboid {
            x_length,
            y_length,
            z_length,
            x_centroid,
            y_centroid,
            z_centroid,
            density: density,
            id: id,
            name: name,
            colour: colour,
            is_selected: is_selected,
        }
    }

    /// Order of indices for gravity summation
    fn index_order() -> Array1<f64> {
        array![1., -1., 1., -1., -1., 1., -1., 1.]
    }
    /// Return cuboid vertices, order aligned with self.index_order()
    pub fn vertices(&self) -> Array2<f64> {
        array![
            [
                self.x_centroid - self.x_length / 2.,
                self.y_centroid - self.y_length / 2.,
                self.z_centroid - self.z_length / 2.
            ],
            [
                self.x_centroid - self.x_length / 2.,
                self.y_centroid - self.y_length / 2.,
                self.z_centroid + self.z_length / 2.
            ],
            [
                self.x_centroid - self.x_length / 2.,
                self.y_centroid + self.y_length / 2.,
                self.z_centroid + self.z_length / 2.
            ],
            [
                self.x_centroid - self.x_length / 2.,
                self.y_centroid + self.y_length / 2.,
                self.z_centroid - self.z_length / 2.
            ],
            [
                self.x_centroid + self.x_length / 2.,
                self.y_centroid - self.y_length / 2.,
                self.z_centroid - self.z_length / 2.
            ],
            [
                self.x_centroid + self.x_length / 2.,
                self.y_centroid + self.y_length / 2.,
                self.z_centroid - self.z_length / 2.
            ],
            [
                self.x_centroid + self.x_length / 2.,
                self.y_centroid + self.y_length / 2.,
                self.z_centroid + self.z_length / 2.
            ],
            [
                self.x_centroid + self.x_length / 2.,
                self.y_centroid - self.y_length / 2.,
                self.z_centroid + self.z_length / 2.
            ],
        ]
    }

    pub fn translate_x(&mut self, val: f64) {
        self.x_centroid += val;
    }

    pub fn translate_y(&mut self, val: f64) {
        self.y_centroid += val;
    }

    pub fn translate_z(&mut self, val: f64) {
        self.z_centroid += val;
    }

    // fn rotate_x(&mut self, angle: f64) {
    //     let r: Array2<f64> = array![
    //         [1., 0., 0.],
    //         [0., angle.cos(), angle.sin()],
    //         [0., -angle.sin(), angle.cos()]
    //     ];
    //     let rotated_vertices = (&self.vertices-self.centre()).dot(&r) + self.centre();
    //     self.vertices = rotated_vertices;
    // }

    // fn rotate_y(&mut self, angle: f64) {
    //     let r: Array2<f64> = array![
    //         [angle.cos(), 0., -angle.sin()],
    //         [0., 1., 0.],
    //         [angle.sin(), 0., angle.cos()]
    //     ];
    //     let rotated_vertices = (&self.vertices-self.centre()).dot(&r) + self.centre();
    //     self.vertices = rotated_vertices;
    // }

    // fn rotate_z(&mut self, angle: f64) {
    //     let r: Array2<f64> = array![
    //         [angle.cos(), angle.sin(), 0.],
    //         [-angle.sin(), angle.cos(), 0.],
    //         [0., 0., 1.]
    //     ];
    //     let rotated_vertices = (&self.vertices-self.centre()).dot(&r) + self.centre();
    //     self.vertices = rotated_vertices;
    // }

    /// Return verices ordered to plot a rectangle in x-z plane using egui Polygon.
    /// Assumes no rotation
    pub fn vertices_xz(&self) -> Vec<[f64; 2]> {
        let verts = self.vertices();
        vec![
            [verts[[0, 0]], verts[[0, 2]]],
            [verts[[1, 0]], verts[[1, 2]]],
            [verts[[6, 0]], verts[[6, 2]]],
            [verts[[4, 0]], verts[[4, 2]]],
        ]
    }
    /// Return verices ordered to plot a rectangle in y-z plane using egui Polygon.
    /// Assumes no rotation
    pub fn vertices_yz(&self) -> Vec<[f64; 2]> {
        let verts = self.vertices();
        vec![
            [verts[[0, 1]], verts[[0, 2]]],
            [verts[[1, 1]], verts[[1, 2]]],
            [verts[[2, 1]], verts[[2, 2]]],
            [verts[[3, 1]], verts[[3, 2]]],
        ]
    }

    pub fn vertices_xy(&self) -> Vec<[f64; 2]> {
        let verts = self.vertices();
        vec![
            [verts[[0, 0]], verts[[0, 1]]],
            [verts[[2, 0]], verts[[2, 1]]],
            [verts[[5, 0]], verts[[5, 1]]],
            [verts[[7, 0]], verts[[7, 1]]],
        ]
    }

    pub fn egui_input(&mut self, ui: &mut Ui) {
        egui::CollapsingHeader::new("position").show(ui, |ui| {
            ui.label("x centroid");
            ui.add(egui::Slider::new(&mut self.x_centroid, -50.0..=50.0).text("m"));

            ui.label("y centroid");
            ui.add(egui::Slider::new(&mut self.y_centroid, -50.0..=50.0).text("m"));

            ui.label("z centroid");
            ui.add(egui::Slider::new(&mut self.z_centroid, -25.0..=25.0).text("m"));
        });
        egui::CollapsingHeader::new("volume").show(ui, |ui| {
            ui.label("x length");
            ui.add(egui::Slider::new(&mut self.x_length, 0.1..=100.0).text("m"));

            ui.label("y length");
            ui.add(egui::Slider::new(&mut self.y_length, 0.1..=100.0).text("m"));

            ui.label("z length");
            ui.add(
                egui::Slider::new(&mut self.z_length, 0.1..=25.0)
                    .text("m")
                    .drag_value_speed(0.1),
            );
            // ui.separator();
            // ui.label(format!("Volume: {}",self.volume()));
        });
        egui::CollapsingHeader::new("density").show(ui, |ui| {
            ui.add(egui::Slider::new(&mut self.density, -3000.0..=22590.).text("kg/m^3"));
            if ui.button("soil void").clicked() {
                self.density = -1800.;
            }
            if ui.button("concrete").clicked() {
                self.density = 2000.;
            }
            if ui.button("lead").clicked() {
                self.density = 11340.;
            }
            if ui.button("tungsten").clicked() {
                self.density = 19300.;
            }
            // ui.separator();
            // ui.label(format!("Mass: {}",self.mass()));
        });
    }
}

impl fmt::Display for Cuboid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "x_length: {}, y_length: {}, z_length: {}, volume: {}, mass: {}, centre: {}",
            // "x_length: {}, y_length: {}, z_length: {}, centre: {}",
            self.x_length,
            self.y_length,
            self.z_length,
            self.volume(),
            self.mass(),
            self.centre()
        )
    }
}
