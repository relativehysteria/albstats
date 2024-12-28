#![feature(variant_count)]
use pnet::packet::Packet;
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::transport::{
    transport_channel, udp_packet_iter,
    TransportChannelType, TransportProtocol
};
use proxy::albion::{PORT_FILTER, Registry};
use photon_decode::Photon;

fn main() {
    // Build the photon decoder
    let mut photon = Photon::new();

    // Build a UDP channel
    let (_tx, mut rx) = transport_channel(4096,
        TransportChannelType::Layer4(TransportProtocol::Ipv4(
                IpNextHeaderProtocols::Udp))).unwrap();

    // Build the albion event registry
    let registry = Registry::new();

    // Listen for UDP packets
    while let Ok((packet, _addr)) = udp_packet_iter(&mut rx).next() {
        // Drop anything non-Albion
        if PORT_FILTER.iter().find(|&&f| f as u16 == packet.get_source())
            .is_none() { continue; }

        // Decode the packet and drop encrypted/unrecognized packets
        // XXX: maybe decrypt? ;)
        let decoded = photon.decode(packet.payload());
        if decoded.is_empty() { continue; }

        // Handle the packets
        decoded.iter().for_each(|msg| {
            match registry.handle_msg(msg) {
                Err(e) => println!("Error while handling message: {e:x?}"),
                Ok(_) => (),
            };
        });
    }
}
