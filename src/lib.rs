pub use dverse_identity::*;
pub use dverse_shield::*;
pub use dverse_dns::*;
pub use dverse_dht::*;
pub use dverse_net::*;
pub use dverse_runtime::*;
pub use dverse_capsule::*;
pub use dverse_relay::*;
pub use dverse_storage::*;


fn start_node(config: DverseConfig) -> Result<()> {
    let net = Net::init(...);
    let trust = Shield::load(...);
    let dht = Dht::join(...);
    let runtime = Runtime::new(...);
    runtime.run_capsules();
    Ok(())
}
