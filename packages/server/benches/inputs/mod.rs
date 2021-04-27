#[allow(dead_code)]
pub static SCREENSHOT: &str = "KDE_Plasma_5.15.jpg";
#[allow(dead_code)]
pub static SCREENSHOT_VR: &str = "KDE_Plasma_5.15-vr.jpg";

pub fn get_input(name: &str) -> String {
    format!("benches/inputs/{}", name)
}

pub fn get_image(name: &str) -> image::DynamicImage {
    image::open(get_input(name)).unwrap()
}
