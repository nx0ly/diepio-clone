use crate::components::*;
use hecs::World;

pub fn register_systems(app: &mut crate::app::App) {
    app.add_system(movement_system);
}

fn movement_system(world: &mut World, _app: &mut crate::app::App) {
    let dt = 1.0 / 111.;
    for (_, (pos, vel)) in world.query::<(&mut Position, &Velocity)>().iter() {
        pos.x += vel.vx * dt;
        pos.y += vel.vy * dt;
    }
}
