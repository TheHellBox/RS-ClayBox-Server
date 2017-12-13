extern crate cobalt;
mod network;

fn main() {
    println!("[Debug] Starting server...");
    let mut server = network::network::new();
    let _ = server.listen("0.0.0.0", "8901");
    loop{
        let _ = server.accept();
    }
}
