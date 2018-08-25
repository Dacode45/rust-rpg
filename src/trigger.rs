use map::Map;
use entity::Entity;

pub type TriggerFn = FnMut(&Trigger, &mut Map, &mut Entity, usize, usize, usize);

pub struct Trigger {
    on_enter: Box<TriggerFn>,
    on_exit: Box<TriggerFn>,
    on_use: Box<TriggerFn>,
}
