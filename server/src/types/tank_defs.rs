use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TankFlags {
    invisibility: bool,
    #[serde(rename = "zoomAbility")]
    zoom_ability: bool,
    #[serde(rename = "canShoot")]
    can_shoot: bool,
    #[serde(rename = "devOnly")]
    dev_only: bool,
}

#[derive(Debug, Deserialize)]
pub struct InvisibilityDef {
    #[serde(rename = "visibilityRateShooting")]
    visibility_rate_shooting: f32,
    #[serde(rename = "visibilityRateMoving")]
    visibility_rate_moving: f32,
    #[serde(rename = "invisibilityRate")]
    invisibility_rate: f32,
}

#[derive(Debug, Deserialize)]
pub struct BarrelDef {
    angle: f32,
    offset: f32,
    size: u32,
    width: f32,
    delay: f32,
    reload: u32,
    recoil: u32,
    flags: BarrelFlags,
    #[serde(rename = "trapezoidDirection")]
    trapezoid_dir: f32,
    addon: u32,
}

// TODO: fix types here later
#[derive(Debug, Deserialize)]
pub struct BarrelBulletDef {
    #[serde(rename = "type")]
    bullet_type: BulletTypes,
    health: f32,
    damage: f32,
    speed: f32,
    #[serde(rename = "scatterRate")]
    scatter_rate: f32,
    #[serde(rename = "lifeLength")]
    life: f32,
    #[serde(rename = "absorbtionFactor")]
    absorbtion_factor: f32,
    #[serde(rename = "sizeRatio")]
    size_ratio: f32,
}

#[derive(Debug, Deserialize)]
pub enum BulletTypes {
    Trap,
    Drone,
    Bullet,
}

#[derive(Debug, Deserialize)]
pub struct BarrelFlags {
    #[serde(rename = "isTrapezoid")]
    is_trapezoid: bool,
    #[serde(rename = "forceFire")]
    force_fire: bool,
}

#[derive(Debug, Deserialize)]
pub struct StatsDef {
    name: String,
    max: u32,
}

#[derive(Debug, Deserialize)]
pub struct TankDef {
    id: u32,
    name: String,
    #[serde(rename = "upgradeMessage")]
    upgrade_msg: Option<String>,
    #[serde(rename = "levelRequirement")]
    level_req: u32,
    upgrades: Vec<u32>,
    flags: TankFlags,
    invisibility: Option<InvisibilityDef>,
    #[serde(rename = "fieldFactor")]
    field_factor: f32,
    #[serde(rename = "absorbtionFactor")]
    absorbtion_factor: f32,
    #[serde(rename = "maxHealth")]
    max_health: u32,
    // figure out what these are
    preAddon: u32,
    postAddon: u32,
    sides: u32,
    barrels: Vec<BarrelDef>,
    stats: Vec<StatsDef>,
}
