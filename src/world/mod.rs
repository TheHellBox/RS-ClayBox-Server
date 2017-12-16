extern crate specs;
extern crate nphysics3d;

pub mod entity;

use nalgebra;
use self::specs::{World, VecStorage, Component};
use self::nalgebra::{Point3, Rotation3, Vector3};


#[derive(Debug)]
pub struct Position{
    pub point: Point3<f32>
}
pub struct Rotation{
    pub rot: Rotation3<f32>
}
pub struct Model{
    pub mesh_path: String
}
pub struct Shader{
    pub shader: String
}
pub struct Visible{
    pub visible: bool
}
pub struct Size{
    pub size: f32
}
pub struct Texture{
    pub tex: String
}

impl Component for Position {
    type Storage = VecStorage<Position>;
}
impl Component for Rotation {
    type Storage = VecStorage<Rotation>;
}
impl Component for Model {
    type Storage = VecStorage<Model>;
}
impl Component for Visible {
    type Storage = VecStorage<Visible>;
}
impl Component for Shader {
    type Storage = VecStorage<Shader>;
}
impl Component for Size {
    type Storage = VecStorage<Size>;
}
impl Component for Texture {
    type Storage = VecStorage<Texture>;
}

pub struct W_World {
    pub world: World,
    pub pworld: nphysics3d::world::World<f32>
}

impl W_World {
    pub fn new() -> W_World{
        let mut world = World::new();
        world.register::<Position>();
        world.register::<Rotation>();
        world.register::<Model>();
        world.register::<Visible>();
        world.register::<Shader>();
        world.register::<Size>();
        world.register::<Texture>();
        let mut pworld = nphysics3d::world::World::new();
        pworld.set_gravity(Vector3::new(0.0, -9.81, 0.0));

        W_World {
            world: world,
            pworld: pworld
        }
    }
    pub fn create_entity(&mut self) -> entity::ent{
        let pos = Position{point: Point3::new(0.0,0.0,0.0)};
        let rot = Rotation{rot: Rotation3::new(Vector3::new(0.0,0.0,0.0))};
        let model = Model{mesh_path: String::new()};
        let vis = Visible{visible: false};
        let shader = Shader{shader: "./assets/shaders/simple".to_string()};
        let texture = Texture{tex: "./assets/textures/err.png".to_string()};
        let size = Size{size: 1.0};
        let ent = self.world.create_entity()
            .with(pos)
            .with(rot)
            .with(model)
            .with(vis)
            .with(shader)
            .with(size)
            .with(texture)
            .build();

        entity::ent{
            ent: ent
        }
    }

    pub fn get_physic_state(&self){
        use std::rc::Rc;
        use self::nphysics3d::object::{RigidBody, WorldObject};

        for x in self.pworld.rigid_bodies() {
            let x = x.borrow();;
            println!("Test {}", x.position())
        }
    }
}
