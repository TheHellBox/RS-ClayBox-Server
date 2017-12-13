use cobalt::{
    BinaryRateLimiter, Config, NoopPacketModifier, MessageKind, UdpSocket,
    Server, ServerEvent
};


pub struct network {
    server: Server<UdpSocket,BinaryRateLimiter,NoopPacketModifier>
}

impl network {
    pub fn new() -> network{
        let mut server = Server::<UdpSocket, BinaryRateLimiter, NoopPacketModifier>::new(Config::default());

        network{
            server: server
        }
    }

    pub fn listen(&mut self, ip: &'static str, port: &'static str){
        self.server.listen(&format!("{}:{}", ip, port)).expect("Failed to bind to socket.");
    }

    pub fn accept(&mut self){
        while let Ok(event) = self.server.accept_receive() {
            println!("{:?}", event);
        };
        self.server.send(true).is_ok();
    }
}
