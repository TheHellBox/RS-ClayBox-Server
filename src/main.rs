extern crate cobalt;
extern crate nalgebra;

#[macro_use]
extern crate bytevec;

mod network;
mod world;

use bytevec::{ByteEncodable, ByteDecodable};

fn main() {
    println!("[Debug] Starting server...");
    let mut server = network::Network::new();
    let _ = server.listen("0.0.0.0", "8901");
    println!("[Debug] Creating world...");
    let mut world = world::W_World::new();
    let ent = world.create_entity();
    ent.set_model("./assets/models/cube.obj".to_string(), &world.world);
    ent.set_texture("./assets/textures/err.png".to_string(), &world.world);
    ent.set_size(0.1, &world.world);
    ent.set_visible(true, &world.world);
    loop{
        let _ = server.accept();
        let net_ent = ent.to_network(&world.world);
        let ent_msg = net_ent.encode::<u8>().unwrap();
        let _ = server.send(ent_msg);
        let old_pos = ent.get_pos(&world.world);
        ent.set_pos(nalgebra::Point3::new(old_pos[0] + 0.01, old_pos[1], old_pos[2]), &world.world);
    }
}
