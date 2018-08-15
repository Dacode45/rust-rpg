extern crate serde;
extern crate serde_json;
extern crate tiled;
extern crate ggez;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_string() {

        let map = tiled::parse(EXAMPLE_CONF.as_bytes()).unwrap();
        println!("{:?}", map);
        panic!()
    }
}

#[allow(dead_code)]
pub static EXAMPLE_CONF: &'static str = 
r#"
<?xml version="1.0" encoding="UTF-8"?>
<map version="1.0" tiledversion="1.1.6" orientation="orthogonal" renderorder="right-down" width="8" height="7" tilewidth="32" tileheight="32" infinite="0" nextobjectid="1">
 <tileset firstgid="1" name="atlas" tilewidth="32" tileheight="32" tilecount="256" columns="16">
  <image source="../../../../../Downloads/examples_explore/tilemap-12-solution/atlas.png" width="512" height="512"/>
 </tileset>
 <layer name="Tile Layer 1" width="8" height="7">
  <data encoding="csv">
1,1,1,5,7,1,1,1,
1,1,1,5,7,1,2,3,
1,1,1,5,7,1,5,6,
1,1,1,5,7,1,5,6,
1,1,2,11,7,1,5,6,
1,1,5,6,7,1,8,9,
1,1,5,6,7,1,1,1
</data>
 </layer>
</map>
"#;