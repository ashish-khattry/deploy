//05_enum_privacy
mod network {
    pub enum ServerState {
        Online,
        Offline,
        Maintenance,
    }
}
fn main() {
    let my_state = network::ServerState::Online;
    match my_state {
        network::ServerState::Online => {
            println!("Server state is online");
        }
        network::ServerState::Offline => {
            println!("Servere state is offline");
        }
        network::ServerState::Maintenance => {
            println!("Servere state is under maintenance");
        }
    }
}
