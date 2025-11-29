use hecs::{Entity, World};
use std::collections::HashMap;
use uuid::Uuid;

use crate::components::{Position, Velocity};

pub struct PlayerMap {
    pub map: HashMap<Uuid, Entity>,
}

pub fn init_player_resources(app: &mut crate::app::App) {
    app.insert_resource(PlayerMap {
        map: HashMap::new(),
    });
}

pub fn spawn_player(app: &mut crate::app::App, client_id: Uuid) {
    let entity = app
        .world
        .get_mut()
        .spawn((Position { x: 0.0, y: 0.0 }, Velocity { vx: 0.0, vy: 0.0 }));

    let players = app
        .get_resource_mut::<PlayerMap>()
        .expect("player map must be init'ed");
    players.map.insert(client_id, entity);

    tracing::info!("spawned player {} -> {:?}", client_id, entity);
}
