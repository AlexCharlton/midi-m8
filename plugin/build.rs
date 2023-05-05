#[cfg(target_os = "windows")]
fn main() {
    let mut res = winres::WindowsResource::new();
    res.set_icon("./include/256.ico");
    res.compile().unwrap();
    println!("Compiled resource into exe");
}

#[cfg(not(target_os = "windows"))]
fn main() {}
