use dialoguer::Select;
use scrap::{Capturer, Display};
use std::io::prelude::*;
use std::io::ErrorKind::WouldBlock;
use std::net::IpAddr;
use std::thread;
use std::time::Duration;
use structopt::StructOpt;
use utils::network::socket::{ListenerError, SocketError};
use utils::{compress, image, network};

#[derive(StructOpt, Debug)]
#[structopt(name = "scvr-server")]
struct Cli {
    #[structopt(short, long, name = "IPv4/6 address")]
    ip: Option<IpAddr>,
    #[structopt(short, long, name = "Port")]
    port: Option<u16>,
    #[structopt(short = "l", long, name = "FPS limit", default_value = "60")]
    fps: u8,
    #[structopt(short = "f", long, name = "Image format", default_value = "jpeg")]
    image_format: image::ImageFormat,
    #[structopt(
        short = "q",
        long = "quality",
        name = "JPEG quality",
        default_value = "75"
    )]
    quality: u8,
    #[structopt(short, long, name = "Image width", default_value = "1920")]
    width: u16,
    #[structopt(short, long, name = "Image height", default_value = "720")]
    height: u16,
}

fn main() {
    let args = Cli::from_args();

    let ips = network::ips::get_all(true).expect("Couldn't get interface addresses.");
    let ip = match args.ip {
        Some(ip) => ip,
        None => {
            let ip_selections: Vec<std::string::String> =
                ips.iter().map(|ip| ip.to_string()).collect();
            let ip = Select::new()
                .default(0)
                .with_prompt("IP address")
                .items(&ip_selections)
                .interact()
                .expect("Couldn't get IP selection.");
            ips[ip]
        }
    };

    let listener = match network::socket::create_listener(ip, args.port) {
        Err(why) => panic!(
            "{}",
            match why {
                ListenerError::LocalAddrError => "Couldn't get local address.",
                ListenerError::BindError => "Couldn't bind to address.",
            }
        ),
        Ok(listener) => listener,
    };
    let (mut stream, _) = match network::socket::get_socket(listener) {
        Err(why) => panic!(
            "{}",
            match why {
                SocketError::AcceptError => "Couldn't connect.",
                SocketError::SetOptionError => "Couldn't set TCP options.",
            }
        ),
        Ok(stream) => stream,
    };

    let one_second = Duration::from_secs(1);
    let one_frame = one_second / args.fps.into();

    let display = Display::primary().expect("Couldn't find primary display.");
    let mut capturer = Capturer::new(display).expect("Couldn't begin capture.");
    let (w, h) = (capturer.width(), capturer.height());

    loop {
        let buffer = match capturer.frame() {
            Ok(buffer) => buffer,
            Err(error) => {
                if error.kind() == WouldBlock {
                    thread::sleep(one_frame);
                    continue;
                } else {
                    panic!("Error: {}", error);
                }
            }
        };

        let image = image::bgra_to_image(
            &buffer,
            image::Dimensions {
                width: w,
                height: h,
            },
        );

        let transformed_image =
            image::vr_transform(&image, None, None).expect("Couldn't transform image.");

        let encoded_image = image::encode_image(transformed_image, args.image_format, args.quality)
            .expect("Couldn't encode image.");

        let compressed_bytes = compress::compress(
            &encoded_image,
            None,
            Some(utils::compress::CompressionFormat::Deflate),
        )
        .expect("Couldn't compress image.");

        stream
            .write(&(compressed_bytes.len() as i32).to_le_bytes())
            .expect("Couldn't send length.");
        stream
            .write_all(&compressed_bytes)
            .expect("Couldn't send data.");
    }
}
