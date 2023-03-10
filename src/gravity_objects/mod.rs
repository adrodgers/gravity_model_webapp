use egui::plot::Line;
use egui::{Color32, Ui, Vec2};
use ndarray::prelude::*;
use std::f64::consts::PI;
use std::fmt;

const G: f64 = 6.674e-11;

/// Required methods to define a new gravity object, to be used within a gravity model.
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq)]
pub enum GravityObject {
    Cuboid(Cuboid),
    Sphere(Sphere),
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct GravityModelObject {
    pub object: GravityObject,
    pub name: String,
    pub id: u128,
    pub colour: Color32,
    pub is_selected: bool,
}

pub trait InputUI {
    fn ui(&mut self, ui: &mut Ui);
}

impl InputUI for GravityModelObject {
    fn ui(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("Name: ");
            ui.text_edit_singleline(&mut self.name);
        });
        ui.horizontal(|ui| {
            ui.label("Colour: ");
            ui.color_edit_button_srgba(&mut self.colour);
        });

        match &mut self.object {
            GravityObject::Cuboid(cuboid) => {
                egui::CollapsingHeader::new("Centroid").show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label("x");
                        ui.add(egui::Slider::new(&mut cuboid.x_centroid, -50.0..=50.0).text("m"));
                    });
                    ui.horizontal(|ui| {
                        ui.label("y");
                        ui.add(egui::Slider::new(&mut cuboid.y_centroid, -50.0..=50.0).text("m"));
                    });
                    ui.horizontal(|ui| {
                        ui.label("z");
                        ui.add(egui::Slider::new(&mut cuboid.z_centroid, -25.0..=25.0).text("m"));
                    });
                });

                egui::CollapsingHeader::new("Rotation").show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label("x");
                        ui.add(
                            egui::Slider::new(&mut cuboid.x_rotation, -PI / 2.0..=PI / 2.)
                                .text("rad"),
                        );
                    });
                    ui.horizontal(|ui| {
                        ui.label("y");
                        ui.add(
                            egui::Slider::new(&mut cuboid.y_rotation, -PI / 2.0..=PI / 2.)
                                .text("rad"),
                        );
                    });
                    ui.horizontal(|ui| {
                        ui.label("z");
                        ui.add(
                            egui::Slider::new(&mut cuboid.z_rotation, -PI / 2.0..=PI / 2.)
                                .text("rad"),
                        );
                    });
                });

                egui::CollapsingHeader::new("Size").show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label("x");
                        ui.add(egui::Slider::new(&mut cuboid.x_length, 0.1..=100.0).text("m"));
                    });
                    ui.horizontal(|ui| {
                        ui.label("y");
                        ui.add(egui::Slider::new(&mut cuboid.y_length, 0.1..=100.0).text("m"));
                    });
                    ui.horizontal(|ui| {
                        ui.label("z");
                        ui.add(egui::Slider::new(&mut cuboid.z_length, 0.1..=25.0).text("m"));
                    });
                });
                egui::CollapsingHeader::new("Density").show(ui, |ui| {
                    ui.add(egui::Slider::new(&mut cuboid.density, -3000.0..=22590.).text("kg/m^3"));
                    ui.radio_value(&mut cuboid.density, -1800., "Soil Void");
                    ui.radio_value(&mut cuboid.density, 2000., "Concrete");
                    ui.radio_value(&mut cuboid.density, 11340., "Lead");
                    ui.radio_value(&mut cuboid.density, 19300., "Tungsten");
                });
            }
            GravityObject::Sphere(sphere) => {
                egui::CollapsingHeader::new("Centroid").show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label("x");
                        ui.add(egui::Slider::new(&mut sphere.x_centroid, -50.0..=50.0).text("m"));
                    });
                    ui.horizontal(|ui| {
                        ui.label("y");
                        ui.add(egui::Slider::new(&mut sphere.y_centroid, -50.0..=50.0).text("m"));
                    });
                    ui.horizontal(|ui| {
                        ui.label("z");
                        ui.add(egui::Slider::new(&mut sphere.z_centroid, -25.0..=25.0).text("m"));
                    });
                });
                egui::CollapsingHeader::new("Radius").show(ui, |ui| {
                    ui.add(egui::Slider::new(&mut sphere.radius, 0.1..=100.0).text("m"));
                });
                egui::CollapsingHeader::new("Density").show(ui, |ui| {
                    ui.add(egui::Slider::new(&mut sphere.density, -3000.0..=22590.).text("kg/m^3"));
                    ui.radio_value(&mut sphere.density, -1800., "Soil Void");
                    ui.radio_value(&mut sphere.density, 2000., "Concrete");
                    ui.radio_value(&mut sphere.density, 11340., "Lead");
                    ui.radio_value(&mut sphere.density, 19300., "Tungsten");
                });
            }
        }
    }
}

pub trait GravityCalc {
    fn calculate(&self, data_type: &DataType, points: &Array2<f64>) -> Array1<f64>;

    fn g(&self, position: &Array1<f64>) -> Array1<f64>;

    fn gg(&self, position: &Array1<f64>) -> Array2<f64>;

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
}

impl Default for Sphere {
    fn default() -> Self {
        Self {
            x_centroid: 0.,
            y_centroid: 0.,
            z_centroid: -1.,
            radius: 1.,
            density: -2000.,
        }
    }
}

impl GravityCalc for Sphere {
    fn calculate(&self, data_type: &DataType, points: &Array2<f64>) -> Array1<f64> {
        let mut data: Array1<f64> = Array1::zeros(points.len_of(Axis(0)));
        let scaling = match data_type {
            DataType::Gx | DataType::Gy | DataType::Gz => -1E8,
            _ => 1E9,
        };
        match data_type {
            DataType::Gx => {
                for (i, point) in points.axis_iter(Axis(0)).enumerate() {
                    data[i] += self.gx(&point.to_owned())
                }
            }
            DataType::Gy => {
                for (i, point) in points.axis_iter(Axis(0)).enumerate() {
                    data[i] += self.gy(&point.to_owned())
                }
            }
            DataType::Gz => {
                for (i, point) in points.axis_iter(Axis(0)).enumerate() {
                    data[i] += self.gz(&point.to_owned())
                }
            }
            DataType::Gxx => {
                for (i, point) in points.axis_iter(Axis(0)).enumerate() {
                    data[i] += self.gxx(&point.to_owned())
                }
            }
            DataType::Gxy => {
                for (i, point) in points.axis_iter(Axis(0)).enumerate() {
                    data[i] += self.gxy(&point.to_owned())
                }
            }
            DataType::Gxz => {
                for (i, point) in points.axis_iter(Axis(0)).enumerate() {
                    data[i] += self.gxz(&point.to_owned())
                }
            }
            DataType::Gyy => {
                for (i, point) in points.axis_iter(Axis(0)).enumerate() {
                    data[i] += self.gyy(&point.to_owned())
                }
            }
            DataType::Gyz => {
                for (i, point) in points.axis_iter(Axis(0)).enumerate() {
                    data[i] += self.gyz(&point.to_owned())
                }
            }
            DataType::Gzz => {
                for (i, point) in points.axis_iter(Axis(0)).enumerate() {
                    data[i] += self.gzz(&point.to_owned())
                }
            }
        }
        data * scaling
    }

    fn g(&self, position: &Array1<f64>) -> Array1<f64> {
        todo!()
    }

    fn gg(&self, position: &Array1<f64>) -> Array2<f64> {
        todo!()
    }

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
        // let x = p_dash[0];
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
    pub x_rotation: f64,
    pub y_rotation: f64,
    pub z_rotation: f64,
    pub density: f64,
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
            x_rotation: 0.,
            y_rotation: 0.,
            z_rotation: 0.,
            density: -2000.,
        }
    }
}

impl GravityCalc for Cuboid {
    fn calculate(&self, data_type: &DataType, points: &Array2<f64>) -> Array1<f64> {
        let rotated_points = (points - self.centre())
            .dot(&rotation_matrix_z(-self.z_rotation))
            .dot(&rotation_matrix_y(-self.y_rotation))
            .dot(&rotation_matrix_x(-self.x_rotation))
            + self.centre();
        let rotation_matrix = rotation_matrix_x(self.x_rotation)
            .dot(&rotation_matrix_y(self.y_rotation).dot(&rotation_matrix_z(self.z_rotation)));
        let mut data: Array1<f64> = Array1::zeros(points.len_of(Axis(0)));
        let scaling = match data_type {
            DataType::Gx | DataType::Gy | DataType::Gz => -1E8,
            _ => 1E9,
        };
        match data_type {
            DataType::Gx => {
                for (i, point) in rotated_points.axis_iter(Axis(0)).enumerate() {
                    data[i] += self.g(&point.to_owned()).dot(&rotation_matrix)[0]
                }
            }
            DataType::Gy => {
                for (i, point) in rotated_points.axis_iter(Axis(0)).enumerate() {
                    data[i] += self.g(&point.to_owned()).dot(&rotation_matrix)[1]
                }
            }
            DataType::Gz => {
                for (i, point) in rotated_points.axis_iter(Axis(0)).enumerate() {
                    data[i] += self.g(&point.to_owned()).dot(&rotation_matrix)[2]
                }
            }
            DataType::Gxx => {
                for (i, point) in rotated_points.axis_iter(Axis(0)).enumerate() {
                    data[i] += (rotation_matrix.t().dot(&self.gg(&point.to_owned())))
                        .dot(&rotation_matrix)[[0, 0]]
                }
            }
            DataType::Gxy => {
                for (i, point) in rotated_points.axis_iter(Axis(0)).enumerate() {
                    data[i] += (rotation_matrix.t().dot(&self.gg(&point.to_owned())))
                        .dot(&rotation_matrix)[[0, 1]]
                }
            }
            DataType::Gxz => {
                for (i, point) in rotated_points.axis_iter(Axis(0)).enumerate() {
                    data[i] += (rotation_matrix.t().dot(&self.gg(&point.to_owned())))
                        .dot(&rotation_matrix)[[0, 2]]
                }
            }
            DataType::Gyy => {
                for (i, point) in rotated_points.axis_iter(Axis(0)).enumerate() {
                    data[i] += (rotation_matrix.t().dot(&self.gg(&point.to_owned())))
                        .dot(&rotation_matrix)[[1, 1]]
                }
            }
            DataType::Gyz => {
                for (i, point) in rotated_points.axis_iter(Axis(0)).enumerate() {
                    data[i] += (rotation_matrix.t().dot(&self.gg(&point.to_owned())))
                        .dot(&rotation_matrix)[[1, 2]]
                }
            }
            DataType::Gzz => {
                for (i, point) in rotated_points.axis_iter(Axis(0)).enumerate() {
                    data[i] += (rotation_matrix.t().dot(&self.gg(&point.to_owned())))
                        .dot(&rotation_matrix)[[2, 2]]
                }
            }
        }
        data * scaling
    }

    fn gx(&self, position: &Array1<f64>) -> f64 {
        let mut gx = 0.;
        let verts = self.vertices_axis_aligned();
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
        let verts = self.vertices_axis_aligned();
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
        let verts = self.vertices_axis_aligned();
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

    fn g(&self, position: &Array1<f64>) -> Array1<f64> {
        let mut g: Array1<f64> = Array1::zeros(3);
        let verts = self.vertices_axis_aligned();
        for i in 0..8 {
            let p_dash: Array1<f64> =
                position * (1. + 1e-7) - verts.index_axis(Axis(0), i).to_owned();
            // Only fetch relevant values once
            let r = p_dash.mapv(|p_dash| p_dash.powi(2)).sum().sqrt();
            let sign = Cuboid::index_order()[i];
            let x = p_dash[0];
            let y = p_dash[1];
            let z = p_dash[2];
            g[0] +=
                sign * ((y * (r + z).ln()) + (z * (r + y).ln()) - (x * ((y * z) / (r * x)).atan()));
            g[1] +=
                sign * ((z * (r + x).ln()) + (x * (r + z).ln()) - (y * ((x * z) / (r * y)).atan()));
            g[2] +=
                sign * ((x * (r + y).ln()) + (y * (r + x).ln()) - (z * ((x * y) / (r * z)).atan()));
        }
        g * G * self.density
    }

    fn gg(&self, position: &Array1<f64>) -> Array2<f64> {
        let mut gg: Array2<f64> = Array2::zeros((3, 3));
        let verts = self.vertices_axis_aligned();
        for i in 0..8 {
            let p_dash: Array1<f64> =
                position * (1. + 1e-7) - verts.index_axis(Axis(0), i).to_owned();
            // Only fetch relevant values once
            let r = p_dash.mapv(|p_dash| p_dash.powi(2)).sum().sqrt();
            let sign = Cuboid::index_order()[i];
            let x = p_dash[0];
            let y = p_dash[1];
            let z = p_dash[2];
            gg[[0, 0]] += sign * -((y * z) / (r * x)).atan();
            gg[[1, 1]] += sign * -((x * z) / (r * y)).atan();
            gg[[2, 2]] += sign * -((y * x) / (r * z)).atan();

            gg[[0, 1]] += sign * (r + z).ln();
            gg[[0, 2]] += sign * (r + y).ln();
            gg[[1, 2]] += sign * (r + x).ln();
        }

        gg[[1, 0]] += gg[[0, 1]];
        gg[[2, 0]] += gg[[0, 2]];
        gg[[2, 1]] += gg[[1, 2]];

        gg * G * self.density
    }

    fn gxx(&self, position: &Array1<f64>) -> f64 {
        let mut gxx = 0.;
        let verts = self.vertices_axis_aligned();
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
        let verts = self.vertices_axis_aligned();
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
        let verts = self.vertices_axis_aligned();
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
        let verts = self.vertices_axis_aligned();
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
        let verts = self.vertices_axis_aligned();
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
        let verts = self.vertices_axis_aligned();
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
}

impl Cuboid {
    pub fn new_from_lengths(
        x_length: f64,
        y_length: f64,
        z_length: f64,
        x_centroid: f64,
        y_centroid: f64,
        z_centroid: f64,
        x_rotation: f64,
        y_rotation: f64,
        z_rotation: f64,
        density: f64,
    ) -> Cuboid {
        Cuboid {
            x_length,
            y_length,
            z_length,
            x_centroid,
            y_centroid,
            z_centroid,
            x_rotation,
            y_rotation,
            z_rotation,
            density: density,
        }
    }

    /// Order of indices for gravity summation
    fn index_order() -> Array1<f64> {
        array![1., -1., 1., -1., -1., 1., -1., 1.]
    }
    /// Return cuboid vertices, order aligned with self.index_order()
    pub fn vertices_axis_aligned(&self) -> Array2<f64> {
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

    /// Return verices ordered to plot a rectangle in x-z plane using egui Polygon.
    /// Assumes no rotation
    pub fn vertices_xz(&self) -> Vec<[f64; 2]> {
        let verts = (self.vertices_axis_aligned() - self.centre())
            .dot(&rotation_matrix_x(self.x_rotation))
            .dot(&rotation_matrix_y(self.y_rotation))
            .dot(&rotation_matrix_z(self.z_rotation))
            + self.centre();
        verts
            .slice(s![.., 0])
            .iter()
            .zip(verts.slice(s![.., 2]).iter())
            .map(|(x, z)| [*x, *z])
            .collect::<Vec<[f64; 2]>>()
    }

    pub fn edge_lines_xz(&self) -> Vec<Line> {
        let mut edges: Vec<Line> = vec![];
        let verts = self.vertices_xz();
        let edge_idx: [[usize; 2]; 12] = [
            [0, 1],
            [1, 2],
            [2, 3],
            [3, 0],
            [4, 5],
            [5, 6],
            [6, 7],
            [7, 4],
            [3, 5],
            [4, 0],
            [6, 2],
            [7, 1],
        ];
        edge_idx
            .iter()
            .for_each(|[i, j]| edges.push(Line::new(vec![verts[*i], verts[*j]])));
        edges
    }

    pub fn vertices_xy(&self) -> Vec<[f64; 2]> {
        let verts = (self.vertices_axis_aligned() - self.centre())
            .dot(&rotation_matrix_x(self.x_rotation))
            .dot(&rotation_matrix_y(self.y_rotation))
            .dot(&rotation_matrix_z(self.z_rotation))
            + self.centre();
        verts
            .slice(s![.., 0])
            .iter()
            .zip(verts.slice(s![.., 1]).iter())
            .map(|(x, y)| [*x, *y])
            .collect::<Vec<[f64; 2]>>()
    }

    pub fn edge_lines_xy(&self) -> Vec<Line> {
        let mut edges: Vec<Line> = vec![];
        let verts = self.vertices_xy();
        let edge_idx: [[usize; 2]; 12] = [
            [0, 1],
            [1, 2],
            [2, 3],
            [3, 0],
            [4, 5],
            [5, 6],
            [6, 7],
            [7, 4],
            [3, 5],
            [4, 0],
            [6, 2],
            [7, 1],
        ];
        edge_idx
            .iter()
            .for_each(|[i, j]| edges.push(Line::new(vec![verts[*i], verts[*j]])));
        edges
    }

    pub fn vertices_yz(&self) -> Vec<[f64; 2]> {
        let verts = (self.vertices_axis_aligned() - self.centre())
            .dot(&rotation_matrix_x(self.x_rotation))
            .dot(&rotation_matrix_y(self.y_rotation))
            .dot(&rotation_matrix_z(self.z_rotation))
            + self.centre();
        verts
            .slice(s![.., 1])
            .iter()
            .zip(verts.slice(s![.., 2]).iter())
            .map(|(y, z)| [*y, *z])
            .collect::<Vec<[f64; 2]>>()
    }

    pub fn edge_lines_yz(&self) -> Vec<Line> {
        let mut edges: Vec<Line> = vec![];
        let verts = self.vertices_yz();
        let edge_idx: [[usize; 2]; 12] = [
            [0, 1],
            [1, 2],
            [2, 3],
            [3, 0],
            [4, 5],
            [5, 6],
            [6, 7],
            [7, 4],
            [3, 5],
            [4, 0],
            [6, 2],
            [7, 1],
        ];
        edge_idx
            .iter()
            .for_each(|[i, j]| edges.push(Line::new(vec![verts[*i], verts[*j]])));
        edges
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

pub fn rotation_matrix_x(angle: f64) -> Array2<f64> {
    array![
        [1., 0., 0.],
        [0., angle.cos(), angle.sin()],
        [0., -angle.sin(), angle.cos()]
    ]
}

pub fn rotation_matrix_y(angle: f64) -> Array2<f64> {
    array![
        [angle.cos(), 0., -angle.sin()],
        [0., 1., 0.],
        [angle.sin(), 0., angle.cos()]
    ]
}

pub fn rotation_matrix_z(angle: f64) -> Array2<f64> {
    array![
        [angle.cos(), angle.sin(), 0.],
        [-angle.sin(), angle.cos(), 0.],
        [0., 0., 1.]
    ]
}
