use crate::gravity_objects;

#[derive(serde::Deserialize, serde::Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum PlotView {
    XY,
    XZ,
    YZ,
}
