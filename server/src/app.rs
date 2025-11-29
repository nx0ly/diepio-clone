// src/app.rs
use flume::{Receiver, Sender, unbounded};
use hecs::World;
use once_cell::sync::Lazy;
use parking_lot::RwLock;
use std::{
    any::{Any, TypeId},
    collections::HashMap,
    sync::Arc,
};
use tokio::time::{Duration, interval};

pub type AppJob = Box<dyn FnOnce(&mut App) + Send + 'static>;

pub static APP_JOB_SENDER: Lazy<Sender<AppJob>> = Lazy::new(|| {
    let (tx, rx) = unbounded();
    start_game_loop(rx);
    tx
});

use std::cell::RefCell;

pub struct App {
    pub world: RefCell<World>,
    resources: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
    systems: Vec<SystemFn>,
}

type SystemFn = Box<dyn Fn(&mut World, &mut App) + Send + Sync>;

impl App {
    pub fn new() -> Self {
        Self {
            world: RefCell::new(World::new()),
            resources: HashMap::new(),
            systems: Vec::new(),
        }
    }

    pub fn add_system<F>(&mut self, f: F)
    where
        F: Fn(&mut World, &mut App) + Send + Sync + 'static,
    {
        self.systems.push(Box::new(f));
    }

    pub fn insert_resource<T: Any + Send + Sync + 'static>(&mut self, resource: T) {
        self.resources.insert(TypeId::of::<T>(), Box::new(resource));
    }

    pub fn get_resource<T: Any + Send + Sync + 'static>(&self) -> Option<&T> {
        self.resources
            .get(&TypeId::of::<T>())
            .and_then(|b| b.downcast_ref::<T>())
    }

    pub fn get_resource_mut<T: Any + Send + Sync + 'static>(&mut self) -> Option<&mut T> {
        self.resources
            .get_mut(&TypeId::of::<T>())
            .and_then(|b| b.downcast_mut::<T>())
    }
}

fn start_game_loop(rx: Receiver<AppJob>) {
    tokio::spawn(async move {
        let mut app = App::new();

        crate::players::init_player_resources(&mut app);

        let mut ticker = interval(Duration::from_millis(111)); // probably consider reducing for smoother game
        crate::net::init_network_resources(&mut app);
        crate::players::init_player_resources(&mut app);
        crate::systems::register_systems(&mut app);

        loop {
            while let Ok(job) = rx.try_recv() {
                job(&mut app);
            }

            //app.update();

            // wait until next tick
            ticker.tick().await;
        }
    });
}
