use world;
use world::specs::{World, Entity};
use nalgebra::{Point3, Vector3, Rotation3, Translation3};
use network;
use nalgebra;

#[derive(PartialEq)]
pub struct ent{
    pub ent: Entity
}

impl ent{
    pub fn set_pos(&self, point: Point3<f32>, world: &World){
        let mut positions = world.write::<world::Position>();
        positions.insert(self.ent, world::Position{point: point});
    }
    pub fn set_rot(&self, vec: Vector3<f32>, world: &World){
        let mut vels = world.write::<world::Rotation>();
        vels.insert(self.ent, world::Rotation{rot: Rotation3::new(vec)});
    }
    pub fn set_size(&self, size: f32, world: &World){
        let mut sizes = world.write::<world::Size>();
        sizes.insert(self.ent, world::Size{size: size});
    }
    pub fn set_model(&self, model: String, world: &World){
        let mut models = world.write::<world::Model>();
        models.insert(self.ent, world::Model{mesh_path: model});
    }
    pub fn set_texture(&self, tex: String, world: &World){
        let mut textures = world.write::<world::Texture>();
        textures.insert(self.ent, world::Texture{tex: tex});
    }
    pub fn set_shader(&self, shader: String, world: &World){
        let mut shaders = world.write::<world::Shader>();
        shaders.insert(self.ent, world::Shader{shader: shader});
    }
    pub fn set_visible(&self, vis: bool, world: &World){
        let mut viss = world.write::<world::Visible>();
        viss.insert(self.ent, world::Visible{visible: vis});
    }
    pub fn get_pos(&self, world: &World) -> Point3<f32>{
        world.read::<world::Position>().get(self.ent).unwrap().point
    }
    pub fn get_rot(&self, world: &World) -> nalgebra::Rotation3<f32>{
        world.read::<world::Rotation>().get(self.ent).unwrap().rot
    }
    pub fn get_size(&self, world: &World) -> f32{
        world.read::<world::Size>().get(self.ent).unwrap().size
    }
    pub fn get_visible(&self, world: &World) -> bool{
        world.read::<world::Visible>().get(self.ent).unwrap().visible
    }
    pub fn get_texture(&self, world: &World) -> String{
        world.read::<world::Texture>().get(self.ent).unwrap().tex.clone()
    }
    pub fn get_model(&self, world: &World) -> String{
        world.read::<world::Model>().get(self.ent).unwrap().mesh_path.clone()
    }
    pub fn get_shader(&self, world: &World) -> String{
        world.read::<world::Shader>().get(self.ent).unwrap().shader.clone()
    }
    pub fn to_network(&self, world: &World) -> network::NetworkEntity{
        let pos = self.get_pos(world);
        let vis = self.get_visible(world);
        let tex = self.get_texture(world);
        let mdl = self.get_model(world);
        network::NetworkEntity{
            id: self.ent.id(),
            pos: (pos.x, pos.y, pos.z),
            vis: match vis{
                true => 1,
                false => 0
            },
            tex: tex,
            mdl: mdl,
        }
    }
}
