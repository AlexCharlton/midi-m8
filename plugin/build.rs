extern crate winres;

fn main() {
    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();
        res.set_icon("./include/256.ico");
        res.compile().unwrap();
        println!("Compiled resource into exe");
    }
}
