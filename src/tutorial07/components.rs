
#[derive(Default)]
pub struct PlayerShip {
    pub target_x: f32,
    pub target_y: f32,
}

pub struct Bullet;

#[derive(Default)]
pub struct GunState {
    pub frames_to_fire: i32
}

//Enemies Management used as Resource
pub struct GlobalEnemyState {
    pub frames_to_next_enemy: i32,
    pub spawn_interval: i32,
}
impl Default for GlobalEnemyState {
    fn default() -> Self {
        GlobalEnemyState {
            frames_to_next_enemy: 20,
            spawn_interval: 20,
        }
    }
}

//One Enemy
pub struct Enemy {
    //when this==0, enemy can move and has collision.
    pub frames_to_appear: i32
}
impl Default for Enemy {
    fn default() -> Self {
        Enemy { frames_to_appear: 10 }
    }
}



