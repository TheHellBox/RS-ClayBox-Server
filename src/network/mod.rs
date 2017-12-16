
use std::collections::HashMap;
use bytevec::{ByteEncodable, ByteDecodable};

use cobalt::{
    BinaryRateLimiter, Config, NoopPacketModifier, MessageKind, UdpSocket,
    Server, ServerEvent
};

pub struct Network {
    server: Server<UdpSocket,BinaryRateLimiter,NoopPacketModifier>
}
impl Network {
    pub fn new() -> Network{
        let mut server = Server::<UdpSocket, BinaryRateLimiter, NoopPacketModifier>::new(Config::default());

        Network{
            server: server
        }
    }
    pub fn listen(&mut self, ip: &'static str, port: &'static str){
        self.server.listen(&format!("{}:{}", ip, port)).expect("Failed to bind to socket.");
    }
    pub fn send(&mut self, msg: Vec<u8>){
        for (_, conn) in self.server.connections() {
            conn.send(MessageKind::Instant, msg.clone());
        }
    }
    pub fn accept(&mut self){
        while let Ok(event) = self.server.accept_receive() {
            println!("{:?}", event);
        };
        self.server.send(true).is_ok();
    }
}
#[derive(PartialEq, Debug, Default)]
pub struct NetworkEntity {
    pub id: u32,
    pub pos: (f32,f32,f32),
    pub vis: u8,
    pub mdl: String,
    pub tex: String
}
bytevec_impls! {
    impl NetworkEntity {
        id: u32,
        pos: (f32,f32,f32),
        vis: u8,
        mdl: String,
        tex: String
    }
}
