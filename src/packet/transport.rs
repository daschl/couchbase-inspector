use byteorder::{ByteOrder, BigEndian};

#[derive(Debug)]
pub enum TransportPacket<'a> {
    TCP(TCPPacket<'a>)
}

#[derive(Debug)]
pub struct TCPPacket<'a> {
    src_port: u16,
    destination_port: u16,
    pub payload: &'a [u8],
}

impl<'a> TCPPacket<'a> {

    pub fn parse(data: &'a [u8]) -> Self {
        let data_offset = (&data[12] & 0xF0) >> 4;
        let offset = (data_offset * 4) as usize;

        TCPPacket {
            src_port: BigEndian::read_u16(&data[0..2]),
            destination_port: BigEndian::read_u16(&data[2..4]),
            payload: &data[offset..],
        }
    }

    pub fn src_port(&self) -> u16 {
        self.src_port
    }

    pub fn destination_port(&self) -> u16 {
        self.destination_port
    }

}

pub fn parse_transport_packet<'a>(data: &'a [u8], protocol: &u8) -> TransportPacket<'a> {
    match *protocol {
        0x00 => TransportPacket::TCP(TCPPacket::parse(data)), // TODO: ??? on loopback ???
        0x06 => TransportPacket::TCP(TCPPacket::parse(data)),
        p => panic!("Unsupported Protocol: {}", p),
    }
}
