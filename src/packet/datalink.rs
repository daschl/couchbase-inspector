use packet::network::{NetworkPacket, parse_network_packet};
use pcap::Linktype;

#[derive(Debug)]
pub enum DataLinkPacket<'a> {
    Loopback(LoopbackPacket<'a>),
    Ethernet(EthernetPacket<'a>),
}

#[derive(Debug)]
pub enum EtherType {
    IPv4
}

#[derive(Debug)]
pub struct LoopbackPacket<'a> {
    pub payload: NetworkPacket<'a>,
}

impl<'a> LoopbackPacket<'a> {

    pub fn parse(data: &'a [u8]) -> Self {
        LoopbackPacket {
            payload: parse_network_packet(&data[4..], EtherType::IPv4)
        }
    }

}

#[derive(Debug)]
pub struct EthernetPacket<'a> {
    source_mac: &'a [u8],
    destination_mac: &'a [u8],
    ether_type: EtherType,
    pub payload: NetworkPacket<'a>,
}

impl<'a> EthernetPacket<'a> {

    pub fn parse(data: &'a [u8]) -> Self {
        let ether_type = || {
            match &data[12..14] {
                t if t == &[0x08, 0x00] => EtherType::IPv4,
                _ => panic!("Unsupported EtherType"),
            }
        };

        EthernetPacket {
            source_mac: &data[0..5],
            destination_mac: &data[4..9],
            ether_type: ether_type(),
            payload: parse_network_packet(&data[14..], ether_type()),
        }
    }

}

pub fn parse_packet<'a>(data: &'a [u8], linktype: &Linktype) -> DataLinkPacket<'a> {
    match *linktype {
        Linktype(0) => DataLinkPacket::Loopback(LoopbackPacket::parse(data)),
        Linktype(1) => DataLinkPacket::Ethernet(EthernetPacket::parse(data)),
        Linktype(t) => panic!("Unsupported Linktype {}", t),
    }
}