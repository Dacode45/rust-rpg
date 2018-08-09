extern crate serde;
extern crate serde_json;

#[derive(Deserialize)]
pub enum TileLayerType {
    Tilelayer,
    Objectgroup, 
    Imagelayer,
    Group,
}

pub fn deserialize_tile_layer_type<'de, D>(de: D) -> Result<TileLayerType, D::Error>
where D: serde::Deserializer<'de>
{
    let deser_result: serde_json::Value = try!(serde::Deserialize::deserialize(de));
    match deser_result {
        serde_json::Value::String(ref s) if &*s == "tilelayer" => Ok(TileLayerType::Tilelayer),
        serde_json::Value::String(ref s) if &*s == "objectgroup" => Ok(TileLayerType::Objectgroup),
        serde_json::Value::String(ref s) if &*s == "imagelayer" => Ok(TileLayerType::Imagelayer),
        serde_json::Value::String(ref s) if &*s == "group" => Ok(TileLayerType::Group),
        _ => Err(serde::de::Error::custom("Unexpected value")),
    }
}

#[derive(Deserialize)]
pub struct TileLayer {
    data: Vec<i32>,
    height: i32,
    name: String,
    opacity: f32,
    #[serde(rename = "type")]
    conf_type: TileLayerType,
    visible: bool,
    width: i32,
    x: i32,
    y: i32,
}

#[derive(Deserialize)]
pub struct TileSet {
    columns: i32,
    firstgid: i32,
    image: String,
    imageheight: i32,
    imagewidth: i32,
    margin: i32,
    name: String,
    spacing: i32,
    tilecount: i32,
    tileheight: i32,
    tilewidth: i32,
}

#[derive(Deserialize)]
pub enum Orientation {
    Orthogonal,
    Isometric,
    Staggered,
    Hexagonal,
}

pub fn deserialize_orientation<'de, D>(de: D) -> Result<Orientation, D::Error>
where D: serde::Deserializer<'de>
{
    let deser_result: serde_json::Value = try!(serde::Deserialize::deserialize(de));
    match deser_result {
        serde_json::Value::String(ref s) if &*s == "orthogonal" => Ok(Orientation::Orthogonal),
        serde_json::Value::String(ref s) if &*s == "orthogonal" => Ok(Orientation::Orthogonal),
        serde_json::Value::String(ref s) if &*s == "staggered" => Ok(Orientation::Staggered),
        serde_json::Value::String(ref s) if &*s == "hexagonal" => Ok(Orientation::Hexagonal),
        _ => Err(serde::de::Error::custom("Unexpected value")),
    }
}

#[derive(Deserialize)]
pub struct TileMap {
    height: i32,
    infinite: bool,
    layers: Vec<TileLayer>,
    nextobjectid: i32,
    #[serde(deserialize_with="deserialize_orientation")]
    orientation: Orientation,
    renderorder: String,
    tiledversion: String,
    tileheight: i32,
    tilesets: Vec<TileSet>,
    tilewidth: i32,
    #[serde(rename = "type")]
    conf_type: String,
    version: i32,
    width: i32,
}

impl TileMap {
    pub fn from_json(data: &str) -> Result<TileMap, serde_json::Error> {
        let conf: TileMap = serde_json::from_str(data)?;
        Ok(conf)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_string() {
        let parsed = TileMap::from_json(EXAMPLE_CONF);
        match parsed {
            Ok(_) => println!("success"),
            Err(e) => {
                println!("{}", e);
                assert!(false, "failed to parse");
            }
        };
    }
}

#[allow(dead_code)]
pub static EXAMPLE_CONF: &'static str = 
r#"{ "height":7,
 "infinite":false,
 "layers":[
        {
         "data":[1, 1, 1, 5, 7, 1, 1, 1, 1, 1, 1, 5, 7, 1, 2, 3, 1, 1, 1, 5, 7, 1, 5, 6, 1, 1, 1, 5, 7, 1, 5, 6, 1, 1, 2, 11, 7, 1, 5, 6, 1, 1, 5, 6, 7, 1, 8, 9, 1, 1, 5, 6, 7, 1, 1, 1],
         "height":7,
         "name":"Tile Layer 1",
         "opacity":1,
         "type":"tilelayer",
         "visible":true,
         "width":8,
         "x":0,
         "y":0
        }],
 "nextobjectid":1,
 "orientation":"orthogonal",
 "renderorder":"right-down",
 "tiledversion":"1.1.6",
 "tileheight":32,
 "tilesets":[
        {
         "columns":16,
         "firstgid":1,
         "image":"..\/..\/..\/..\/..\/Downloads\/examples_explore\/tilemap-12-solution\/atlas.png",
         "imageheight":512,
         "imagewidth":512,
         "margin":0,
         "name":"atlas",
         "spacing":0,
         "tilecount":256,
         "tileheight":32,
         "tilewidth":32
        }],
 "tilewidth":32,
 "type":"map",
 "version":1,
 "width":8
}"#;