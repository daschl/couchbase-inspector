use packet::transport::{TransportPacket, parse_transport_packet};
use packet::datalink::EtherType;

#[derive(Debug)]
pub enum NetworkPacket<'a> {
    IPv4(IPv4Packet<'a>)
}


#[derive(Debug)]
pub struct IPv4Packet<'a> {
    source_addr: &'a [u8],
    destination_addr: &'a [u8],
    protocol: &'a u8,
    pub payload: TransportPacket<'a>,
}

impl<'a> IPv4Packet<'a> {

    pub fn parse(data: &'a [u8]) -> Self {
        let ihl = &data[0] & 0xF;
        let payload_start = (ihl * 4) as usize;

        IPv4Packet {
            source_addr: &data[12..16],
            destination_addr: &data[16..20],
            protocol: &data[9],
            payload: parse_transport_packet(&data[payload_start..], &data[9]),
        }
    }
}

pub fn parse_network_packet<'a>(data: &'a [u8], ether_type: EtherType) -> NetworkPacket<'a> {
    match ether_type {
        EtherType::IPv4 => NetworkPacket::IPv4(IPv4Packet::parse(data)),
    }
}
