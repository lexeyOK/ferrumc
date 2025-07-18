use ferrumc_macros::{packet, NetDecode};

#[derive(NetDecode, Debug)]
#[packet(packet_id = "status_request", state = "status")]
pub struct StatusRequestPacket {}
