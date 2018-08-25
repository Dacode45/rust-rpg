use map::Map;
use trigger::TriggerFn;
use entity::Entity;

fn teleport(tile_x: usize, tile_y: usize, layer: usize) -> Box<TriggerFn> {
    Box::new(move |trigger, map, entity, _, _ , _| {
        map.set_tile_pos(entity, tile_x, tile_y, layer)
    })
}

fn add_npc(map: &Map, npc: Entity) -> Box<TriggerFn> {
    Box::new(|trigger, map, entity, tx, ty, layer| {

    })
}