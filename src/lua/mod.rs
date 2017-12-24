extern crate rlua;
use world;
use self::rlua::{Table, Lua, UserData, ToLua, Value, UserDataMethods, Function, Nil};
use std::fs::File;
use std::io::prelude::*;
use nalgebra::{Point3};
use specs::Join;

pub struct lua{
    lua: Lua
}

#[derive(Clone)]
pub struct Lua_Entity {
    pub id: f32,
    pub pos: (f32,f32,f32),
    pub tex: String,
    pub model: String,
    pub vis: bool
}

impl UserData for Lua_Entity {
    fn add_methods(methods: &mut UserDataMethods<Self>) {
        methods.add_method_mut("set_pos", |_,ent: &mut Lua_Entity, pos: (f32,f32,f32)| {
            ent.pos = pos;
            Ok(())
        });

        methods.add_method_mut("set_texture", |_,ent: &mut Lua_Entity, tex: String| {
            ent.tex = tex;
            Ok(())
        });
        methods.add_method_mut("set_model", |_,ent: &mut Lua_Entity, mdl: String| {
            ent.model = mdl;
            Ok(())
        });
        methods.add_method_mut("set_visible", |_,ent: &mut Lua_Entity, vis: bool| {
            ent.vis = vis;
            Ok(())
        });

        methods.add_method_mut("get_pos", |_,ent: &mut Lua_Entity, ()| {
            Ok(ent.pos)
        });
        methods.add_method_mut("get_id", |_,ent: &mut Lua_Entity, ()| {
            Ok(ent.id)
        });
        methods.add_method_mut("get_tex", |_,ent: &mut Lua_Entity, ()| {
            Ok(ent.tex.clone())
        });

        methods.add_method_mut("get_model", |_,ent: &mut Lua_Entity, ()| {
            Ok(ent.model.clone())
        });
        methods.add_method_mut("get_visible", |_,ent: &mut Lua_Entity, ()| {
            Ok(ent.vis)
        });
    }
}

impl lua{
    pub fn new() -> lua{
        let lua = Lua::new();
        lua{
            lua: lua
        }
    }

    pub fn init(&self){
        let globals = self.globals();
        let world = self.lua.create_table();
        let ents = self.lua.create_table();
        let events = self.lua.create_table();
        let mouse = self.lua.create_table();
        mouse.set("x", 0);
        mouse.set("y", 0);
        world.set("ents", ents);
        globals.set("world", world);
        globals.set("mouse", mouse);
        globals.set("__events", events);
    }

    pub fn std_lib(&self){
        self.lua.eval::<()>(r#"
        function AddEventCallback(name, fn)
            table.insert(__events[name], fn)
        end
        "#,
        None
        );
    }

    pub fn globals(&self) -> Table{
        self.lua.globals()
    }

    pub fn call(&self, fnname: &'static str){
        let globals: Table = self.globals();
        let lfn: Function = globals.get(fnname).unwrap();
        lfn.call::<_, ()>(());
    }

    pub fn add_event(&self, evname: &'static str){
        let globals = self.globals();
        let luaevent = globals.get::<_,Table>("__events").unwrap();
        let triggers = self.lua.create_table();
        luaevent.set(evname, triggers);
    }

    pub fn call_event(&self, evname: &'static str, args: Option<Table>){
        let globals = self.globals();
        let luaevent = globals.get::<_,Table>("__events").unwrap();
        let fns = luaevent.get::<_,Table>(evname).unwrap();
        for pair in fns.pairs::<Value, Function>() {
            let (val, func) = pair.unwrap();
            match &args {
                &None => func.call::<_, ()>(()).unwrap(),
                &Some(ref Table) => func.call::<_, ()>(()).unwrap(),
                _ => (),
            };
        }
    }

    pub fn run(&self, code: String){
        self.lua.eval::<()>(
            &code,
            None,
        );
    }

    pub fn run_file(&self, path: String){
        let mut code = String::new();
        let mut file = File::open(&path).expect("Error: file not found");;
        file.read_to_string(&mut code)
            .expect("Error: read failed");
        self.run(code);
    }

    pub fn run_addon(&self, name: &'static str){
        let filename = format!("./assets/addons/{}/server/init.lua", name);
        self.run_file(filename);
    }

    pub fn run_all(&self){
        use std::fs;
        let path = "./assets/addons/";
        let paths = fs::read_dir(path).unwrap();
        for path in paths {
            let path = path.unwrap().path().display().to_string();
            let filename = format!("{}/server/init.lua", path);
            self.run_file(filename);
        }
    }

    pub fn get_all(&self, world: &mut world::W_World, ents: &mut Vec<world::entity::ent>){
        let globals = self.globals();
        let luaworld = globals.get::<_,Table>("world").unwrap();
        let luaents = luaworld.get::<_,Table>("ents").unwrap();
        let len = (luaents.raw_len() + 1) - ents.len() as i64;
        if len > -1{
            for _ in 0..len{
                let ent = world.create_entity();
                let id = ent.ent.id();
                let l_ent = Lua_Entity{
                    id: id as f32,
                    pos: (0.0,0.0,0.0),
                    tex: "".to_string(),
                    model: "".to_string(),
                    vis: false
                };
                luaents.set(id, l_ent);
                ents.push(ent);
                let code = format!(r#"
                for k,v in pairs(__events["OnEntityCreate"]) do
                    v(world.ents[{}])
                end
                "#, id);
                self.run(code);
            }
        }
        else{
            for _ in 0..len * -1{
                let mut lua_ids = vec![];
                let ids = ents.iter().map(|&ref x| x.ent.id()).collect::<Vec<_>>();
                let luaents = luaworld.get::<_,Table>("ents").unwrap();
                for pair in luaents.pairs::<Value, Lua_Entity>() {
                    let (_, value) = pair.unwrap();
                    lua_ids.push(value.id as u32);
                }
                let luaents = luaworld.get::<_,Table>("ents").unwrap();
                for id in &ids{
                    if !lua_ids.contains(&id) {
                        for x in 0..ents.len(){
                            if ents[x].ent.id() == *id {
                                world.world.delete_entity(ents[x].ent);
                                ents.remove(x);
                                let code = format!(r#"
                                for k,v in pairs(__events["OnEntityRemove"]) do
                                    v({})
                                end
                                "#, id);
                                self.run(code);
                                break
                            }
                        }
                    }
                }
            }
        }
        let ids = ents.iter().map(|&ref x| x.ent.id()).collect::<Vec<_>>();
        for pair in luaents.pairs::<Value, Lua_Entity>() {
            let (_, value) = pair.unwrap();
            let (posx, posy, posz) = value.pos;
            ents[value.id as usize].set_pos(Point3::new(posx, posy, posz), &world.world);
            ents[value.id as usize].set_texture(value.tex, &world.world);
            ents[value.id as usize].set_model(value.model, &world.world);
            ents[value.id as usize].set_visible(value.vis, &world.world);
        }
    }
    pub fn update(&self, world: &world::W_World, ents: &Vec<world::entity::ent>){
        let globals = self.globals();
        let luaworld = globals.get::<_,Table>("world").unwrap();
        let luaents = luaworld.get::<_,Table>("ents").unwrap();
        for ent in ents{
            let id = ent.ent.id();
            let pos = ent.get_pos(&world.world);
            let model = ent.get_model(&world.world);
            let texture = ent.get_texture(&world.world);
            let vis = ent.get_visible(&world.world);
            let ent = Lua_Entity{
                id: id as f32,
                pos: (pos.x,pos.y,pos.z),
                tex: texture,
                model: model,
                vis: vis
            };
            luaents.set(id, ent);
        }
    }
}
