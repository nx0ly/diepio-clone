use hecs::{Entity, World};

#[derive(Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug)]
pub struct Velocity {
    pub vx: f32,
    pub vy: f32,
}

#[derive(Debug)]
pub struct Health {
    pub hp: i32,
    pub max_hp: i32,
}

#[derive(Debug)]
pub struct Bullet {
    pub damage: i32,
    pub owner: Entity,
}

// UPDATE THIS LATER!!!

fn movement_system(world: &mut World, dt: f32) {
    for (_, (pos, vel)) in world.query::<(&mut Position, &Velocity)>().iter() {
        pos.x += vel.vx * dt;
        pos.y += vel.vy * dt;
    }
}

fn bullet_system(world: &mut World) {
    let mut to_despawn = Vec::new();

    for (entity, bullet) in world.query::<&Bullet>().iter() {
        if bullet.damage <= 0 {
            to_despawn.push(entity);
        }
    }

    for e in to_despawn {
        world.despawn(e).unwrap();
    }
}
