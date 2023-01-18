use ndarray::prelude::*;
use std::fmt::{self};
// use assert_approx_eq::assert_approx_eq;
// use ndarray::arr2;
// use rayon::prelude::*;
// use std::time::Instant;

const G: f64 = 6.674e-11;

pub struct Cuboid {
    pub vertices: Array2<f64>,
    density: f64,
}

impl Default for Cuboid {
    fn default() -> Self {
        let vertices: Array2<f64> = array![
        [-1., -1., -1.],
        [-1., -1., 1.],
        [-1., 1., 1.],
        [-1., 1., -1.],
        [1., -1., -1.],
        [1., 1., -1.],
        [1., 1., 1.],
        [1., -1., 1.]
    ]; 
        Self { vertices: vertices, density: -2000. }
    }
}

impl Cuboid {
    /// Check vertices are in the correct order!
    pub fn new_from_vertices(vertices: Array2<f64>, density: f64) -> Cuboid {
        Cuboid {
            vertices: vertices,
            density: density,
        }
    }

    pub fn new_from_lengths(centroid: [f64;3], lengths: [f64;3], density: f64) -> Cuboid {
        let vertices: Array2<f64> = array![
            [centroid[0] - lengths[0]/2., centroid[1] - lengths[1]/2., centroid[2] - lengths[2]/2.],
            [centroid[0] - lengths[0]/2., centroid[1] - lengths[1]/2., centroid[2] + lengths[2]/2.],
            [centroid[0] - lengths[0]/2., centroid[1] + lengths[1]/2., centroid[2] + lengths[2]/2.],
            [centroid[0] - lengths[0]/2., centroid[1] + lengths[1]/2., centroid[2] - lengths[2]/2.],
            [centroid[0] + lengths[0]/2., centroid[1] - lengths[1]/2., centroid[2] - lengths[2]/2.],
            [centroid[0] + lengths[0]/2., centroid[1] + lengths[1]/2., centroid[2] - lengths[2]/2.],
            [centroid[0] + lengths[0]/2., centroid[1] + lengths[1]/2., centroid[2] + lengths[2]/2.],
            [centroid[0] + lengths[0]/2., centroid[1] - lengths[1]/2., centroid[2] + lengths[2]/2.],
            ];

        Cuboid {
            vertices: vertices,
            density: density,
        }
    }

    fn x_length(&self) -> f64 {
        let len: f64 = self.vertices[[0, 0]] - self.vertices[[7, 0]];
        len.abs()
    }

    fn y_length(&self) -> f64 {
        let len: f64 = self.vertices[[0, 1]] - self.vertices[[2, 1]];
        len.abs()
    }

    fn z_length(&self) -> f64 {
        let len: f64 = self.vertices[[0, 2]] - self.vertices[[2, 2]];
        len.abs()
    }

    pub fn volume(&self) -> f64 {
        self.x_length() * self.y_length() * self.z_length()
    }

    pub fn scale(&mut self, factor: (f64,f64,f64)) {
        self.scale_x(factor.0);
        self.scale_y(factor.1);
        self.scale_z(factor.2);
    }

    pub fn scale_x(&mut self, factor: f64) {
        let mut x = self.vertices.index_axis_mut(Axis(1), 0);
        x *= factor;
    }

    pub fn scale_y(&mut self, factor: f64) {
        let mut y = self.vertices.index_axis_mut(Axis(1), 1);
        y *= factor;
    }

    pub fn scale_z(&mut self, factor: f64) {
        let mut z = self.vertices.index_axis_mut(Axis(1), 2);
        z *= factor;
    }

    pub fn mass(&self) -> f64 {
        self.density * self.volume()
    }

    pub fn translate(&mut self, point: &Array1<f64>) {
        self.vertices += point;
    }

    fn rotate_x(&mut self, angle: f64) {
        let r: Array2<f64> = array![
            [1., 0., 0.],
            [0., angle.cos(), angle.sin()],
            [0., -angle.sin(), angle.cos()]
        ];
        let rotated_vertices = (&self.vertices-self.centre()).dot(&r) + self.centre();
        self.vertices = rotated_vertices;
    }

    fn rotate_y(&mut self, angle: f64) {
        let r: Array2<f64> = array![
            [angle.cos(), 0., -angle.sin()],
            [0., 1., 0.],
            [angle.sin(), 0., angle.cos()]
        ];
        let rotated_vertices = (&self.vertices-self.centre()).dot(&r) + self.centre();
        self.vertices = rotated_vertices;
    }

    fn rotate_z(&mut self, angle: f64) {
        let r: Array2<f64> = array![
            [angle.cos(), angle.sin(), 0.],
            [-angle.sin(), angle.cos(), 0.],
            [0., 0., 1.]
        ];
        let rotated_vertices = (&self.vertices-self.centre()).dot(&r) + self.centre();
        self.vertices = rotated_vertices;
    }

    /// Order of indices for gravity summation
    fn index_order() -> Array1<f64> {
        array![1., -1., 1., -1., -1., 1., -1., 1.]
    }

    pub fn centre(&self) -> Array1<f64> {
        self.vertices.sum_axis(Axis(0)) / 8.
    }

    /// Return verices ordered to plot a rectangle using egui Polygon
    pub fn vertices_xz(&self) -> Vec<[f64;2]> {
        let verts = &self.vertices;
        vec![[verts[[0, 0]],verts[[0, 2]]],[verts[[1, 0]],verts[[1, 2]]],[verts[[6, 0]],verts[[6, 2]]],[verts[[4, 0]],verts[[4, 2]]],]
    }

    // fn gravity(&self, position: &Array1<f64>) -> Array1<f64> {
    //     let mut gravity: Array1<f64> = ndarray::Array::zeros(3);
    //     let pos_neg: Array1<f64> = array![1., -1., 1., -1., -1., 1., -1., 1.];
    //     for i in 0..8 {
    //         let p_dash: Array1<f64> =
    //             position * (1. + 1e-7) - self.vertices.index_axis(Axis(0), i).to_owned();
    //         let r = p_dash.mapv(|p_dash| p_dash.powi(2)).sum().sqrt();
    //         gravity[0] += pos_neg[i]
    //             * ((p_dash[1] * (r + p_dash[2]).ln()) + (p_dash[2] * (r + p_dash[1]).ln())
    //                 - (p_dash[0] * ((p_dash[1] * p_dash[2]) / (r * p_dash[0])).atan()));
    //         gravity[1] += pos_neg[i]
    //             * ((p_dash[2] * (r + p_dash[0]).ln()) + (p_dash[0] * (r + p_dash[2]).ln())
    //                 - (p_dash[1] * ((p_dash[0] * p_dash[2]) / (r * p_dash[1])).atan()));
    //         gravity[2] += pos_neg[i]
    //             * ((p_dash[0] * (r + p_dash[1]).ln()) + (p_dash[1] * (r + p_dash[0]).ln())
    //                 - (p_dash[2] * ((p_dash[0] * p_dash[1]) / (r * p_dash[2])).atan()));
    //     }
    //     gravity * G * self.density
    // }

    // fn gravity_gradient(&self, position: &Array1<f64>) -> Array2<f64> {
    //     let mut gravity_gradient: Array2<f64> = ndarray::Array::zeros((3, 3));
    //     let pos_neg: Array1<f64> = array![1., -1., 1., -1., -1., 1., -1., 1.];
    //     for i in 0..8 {
    //         let p_dash: Array1<f64> =
    //             position * (1. + 1e-7) - self.vertices.index_axis(Axis(0), i).to_owned();
    //         let r = p_dash.mapv(|p_dash| p_dash.powi(2)).sum().sqrt();

    //         gravity_gradient[[0, 0]] +=
    //             pos_neg[i] * -((p_dash[1] * p_dash[2]) / (r * p_dash[0])).atan();
    //         gravity_gradient[[1, 1]] +=
    //             pos_neg[i] * -((p_dash[0] * p_dash[2]) / (r * p_dash[1])).atan();
    //         gravity_gradient[[2, 2]] +=
    //             pos_neg[i] * -((p_dash[1] * p_dash[0]) / (r * p_dash[2])).atan();
    //         gravity_gradient[[0, 1]] += pos_neg[i] * (r + p_dash[2]).ln();
    //         gravity_gradient[[0, 2]] += pos_neg[i] * (r + p_dash[1]).ln();
    //         gravity_gradient[[1, 2]] += pos_neg[i] * (r + p_dash[0]).ln();
    //     }
    //     gravity_gradient * G * self.density
    // }

    pub fn gz (&self , position: &Array1<f64>) -> f64 {
        let mut gz = 0.;
        for i in 0..8 {
            let p_dash: Array1<f64> =
                position * (1. + 1e-7) - self.vertices.index_axis(Axis(0), i).to_owned();
            // Only fetch relevant values once
            let r = p_dash.mapv(|p_dash| p_dash.powi(2)).sum().sqrt();
            let sign = Cuboid::index_order()[i];
            let x = p_dash[0];
            let y = p_dash[1];
            let z = p_dash[2];
            gz += sign
                * ((x * (r + y).ln()) + (y * (r + x).ln())
                    - (z * ((x * y) / (r * z)).atan()));
        };
        gz * G * self.density
    }

    pub fn gzz (&self , position: &Array1<f64>) -> f64 {
        let mut gzz = 0.;
        for i in 0..8 {
            let p_dash: Array1<f64> =
                position * (1. + 1e-7) - self.vertices.index_axis(Axis(0), i).to_owned();
            // Only fetch relevant values once
            let r = p_dash.mapv(|p_dash| p_dash.powi(2)).sum().sqrt();
            let sign = Cuboid::index_order()[i];
            let x = p_dash[0];
            let y = p_dash[1];
            let z = p_dash[2];
            gzz += sign * -((y * x) / (r * z)).atan()
        };
        gzz * G * self.density
    }

    fn gravity_complete(&self, position: &Array1<f64>) -> (Array1<f64>,Array1<f64>) {
        let mut gravity: Array1<f64> = ndarray::Array::zeros(3);
        let mut gravity_gradient: Array1<f64> = ndarray::Array::zeros(9);
        // let pos_neg: Array1<f64> = array![1., -1., 1., -1., -1., 1., -1., 1.];
        let density = self.density;
        for i in 0..8 {
            let p_dash: Array1<f64> =
                position * (1. + 1e-7) - self.vertices.index_axis(Axis(0), i).to_owned();
            // Only fetch relevant values once
            let r = p_dash.mapv(|p_dash| p_dash.powi(2)).sum().sqrt();
            let sign = Cuboid::index_order()[i];
            let x = p_dash[0];
            let y = p_dash[1];
            let z = p_dash[2];

            gravity[0] += sign
                * ((y * (r + z).ln()) + (z * (r + y).ln())
                    - (x * ((y * z) / (r * x)).atan()));
            gravity[1] += sign
                * ((z * (r + x).ln()) + (x * (r + z).ln())
                    - (y * ((x * z) / (r * y)).atan()));
            gravity[2] += sign
                * ((x * (r + y).ln()) + (y * (r + x).ln())
                    - (z * ((x * y) / (r * z)).atan()));
            // xx
            gravity_gradient[0] +=
                sign * -((y * z) / (r * x)).atan();
            // yy
            gravity_gradient[1] +=
                sign * -((x * z) / (r * y)).atan();
            // zz
            gravity_gradient[2] +=
                sign * -((y * x) / (r * z)).atan();
            // xy
            gravity_gradient[3] += sign * (r + z).ln();
            // xz
            gravity_gradient[4] += sign * (r + y).ln();
            // yz
            gravity_gradient[5] += sign * (r + x).ln();
        };
        (gravity * G * density, gravity_gradient * G * density)
    }

}

impl fmt::Display for Cuboid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "x_length: {}, y_length: {}, z_length: {}, volume: {}, mass: {}, centre: {}",
            self.x_length(),
            self.y_length(),
            self.z_length(),
            self.volume(),
            self.mass(),
            self.centre()
        )
    }
}

