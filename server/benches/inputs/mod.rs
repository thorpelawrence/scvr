pub static SCREENSHOT: &'static str = "KDE_Plasma_5.15.jpg";

pub fn get_input(name: &str) -> String {
    format!("benches/inputs/{}", name)
}

pub fn get_image(name: &str) -> image::DynamicImage {
    image::open(get_input(name)).unwrap()
}
