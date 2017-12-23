extern crate cobalt;
extern crate nalgebra;
extern crate specs;
#[macro_use]
extern crate bytevec;

mod network;
mod world;
mod lua;

use specs::Join;
use bytevec::{ByteEncodable, ByteDecodable};

fn main() {
    println!("[Debug] Starting server...");
    let mut server = network::Network::new();
    let _ = server.listen("0.0.0.0", "8901");
    println!("[Debug] Creating world...");
    let mut world = world::W_World::new();
    let ent = world.create_entity();
    println!("[Debug] Starting LUA...");
    let lua = lua::lua::new();
    lua.init();
    lua.add_event("Update");
    lua.std_lib();
    let mut ents = vec![];
    for entity in world.world.entities().join() {
        ents.push(world::entity::ent{ent: entity});
    }
    lua.update(&world, &ents);
    lua.run_all();
    println!("[Debug] Server started!");
    loop{
        lua.update(&world, &ents);
        let mut ents = vec![];
        for entity in world.world.entities().join() {
            ents.push(world::entity::ent{ent: entity});
        }
        let _ = server.accept();
        let net_ent = ent.to_network(&world.world);
        let ent_msg = net_ent.encode::<u8>().unwrap();
        let _ = server.send(ent_msg);
        lua.call_event("Update", None);
        lua.get_all(&mut world, &mut ents);
    }
}
