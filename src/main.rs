extern crate clap;
extern crate pcap;
extern crate byteorder;
extern crate httparse;

mod packet;

use std::collections::HashMap;

use pcap::{Capture, Device, PacketHeader};
use clap::App;
use httparse::{EMPTY_HEADER, Request, Response};

use packet::datalink::{DataLinkPacket, parse_packet};
use packet::network::NetworkPacket;
use packet::transport::TransportPacket;

fn main() {
    // Setup the CLI Flags
    let args = App::new("Couchbase Inspector")
                    .version("0.1")
                    .author("Michael Nitschinger <michael.nitschinger@couchbase.com>")
                    .about("This tool allows you to inspect real-time traffic easily.")
                    .args_from_usage("-i, --interface=<IFNAME> 'The network interface to use'")
                    .get_matches();


    // Try to locate the interface to use for capturing
    let wanted_if = args.value_of("interface").expect("No network interface set to use!");
    let mut devices = Device::list().expect("Could not list network interfaces!");
    let found_position = devices.iter().position(|d| d.name == wanted_if).expect("No matching network interface found!");
    let found_device = devices.remove(found_position);

    // Start the interface capture
    let mut capture = Capture::from_device(found_device).expect("Could not initialize capture")
        .promisc(true)
        .timeout(1000)
        .open().expect("Could not open the network capture");

    // Applying filters in BPF Syntax
    capture.filter("tcp port 8093").expect("Could not apply BPF filter!");


    let linktype = capture.get_datalink();

    let mut outstanding = HashMap::new();

    // Iterate through the packets as they arrive on the interface
    loop {
        while let Ok(packet) = capture.next() {
            match parse_packet(packet.data, &linktype) {
                DataLinkPacket::Ethernet(e) => handle_network_packets(e.payload, &packet.header, &mut outstanding),
                DataLinkPacket::Loopback(e) => handle_network_packets(e.payload, &packet.header, &mut outstanding),
            }
        }
    }
}

fn handle_network_packets(np: NetworkPacket, pcap_header: &PacketHeader, outstanding: &mut HashMap<String, MetricHolder>) {
    match np {
        NetworkPacket::IPv4(n) => match n.payload {
            TransportPacket::TCP(t) => {
                if t.destination_port() == 8093 && t.payload.len() > 0 {
                    let ident = t.src_port().to_string() + t.destination_port().to_string().as_ref();
                    handle_http_request(t.payload, ident, pcap_header, outstanding);
                }

                if t.src_port() == 8093 && t.payload.len() > 0 {
                    let ident = t.destination_port().to_string() + t.src_port().to_string().as_ref();
                    handle_http_response(t.payload, ident, pcap_header, outstanding);
                }
            }
        }
    }
}

fn handle_http_request(payload: &[u8], ident: String, pcap_header: &PacketHeader, outstanding: &mut HashMap<String, MetricHolder>) {
    let mut headers = [EMPTY_HEADER; 16];
    let mut req = Request::new(&mut headers);
    let res = req.parse(payload).unwrap().unwrap();

    if req.method.unwrap() == "POST" && req.path.unwrap() == "/query" {
        if payload.len() > res {
            outstanding.insert(ident, MetricHolder {
                query: std::str::from_utf8(&payload[res..]).unwrap().to_string(),
                req_time: pcap_header.ts.tv_usec as u64,
            });
        }
    }

}

fn handle_http_response(payload: &[u8], ident: String, pcap_header: &PacketHeader, outstanding: &mut HashMap<String, MetricHolder>) {
//    let mut headers = [EMPTY_HEADER; 16];
//    let mut req = Response::new(&mut headers);
    //let res = req.parse(payload).unwrap().unwrap();

    match outstanding.remove(&ident) {
        Some(v) => {
            println!("{} took {}ms", v.query, (pcap_header.ts.tv_usec as u64 - v.req_time)/1000);
        },
        None => ()
    }
}

#[derive(Debug)]
struct MetricHolder {
    pub query: String,
    pub req_time: u64
}