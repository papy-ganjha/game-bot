mod screen;
mod system_control;

use opencv::imgcodecs;

fn test_opencv(path: &str) {
    println!("The path is: {}", path);
    let image = imgcodecs::imread(path, imgcodecs::IMREAD_COLOR);
    println!("Image read!")
}

#[cfg(test)]
#[test]
fn test_plot() {
    println!("TEST")
}

fn main() {
    test_opencv("screenshot.png");
    // system_control::system_inputs();
    // screen::screenshot();
}
