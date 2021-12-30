#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Vec2 {
    x: f64,
    y: f64,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}


#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DirectionUv {
    /// Specifies the uv origin for the face. For this face, it is the upper-left corner, when looking at the face with y being up.
    uv: f64,
    /// The face maps this many texels from the uv origin. If not specified, the box dimensions are used instead.
    uv_size: f64,
    ///Specifies the UV's for the face that stretches
    material_instance: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Data {
    pub format_version: String,
    #[serde(rename = "minecraft:geometry")]
    pub minecraft_geometry: Vec<MinecraftGeometry>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MinecraftGeometry {
    pub bones: Vec<Bone>,
    pub description: Description,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Bone {
    #[serde(default)]
    pub cubes: Vec<Cube>,
    #[serde(default)]
    pub mirror: bool,
    pub name: String,
    pub parent: Option<String>,
    #[serde(default)]
    pub pivot: Vec<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Cube {
    pub origin: Vec<f64>,
    pub size: Vec<f64>,
    pub uv: Vec<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Description {
    pub identifier: String,
    pub texture_height: f64,
    pub texture_width: f64,
    pub visible_bounds_offset: Vec<f64>,
    pub visible_bounds_width: f64,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
