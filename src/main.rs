mod _pick_device;

fn main() {
    // println!("Hello, world!");
    evtest()
}

fn evtest() {
    let mut d = _pick_device::pick_device();
    println!("{}", d);
    println!("Events:");
    loop {
        for ev in d.fetch_events().unwrap() {
            println!("{:?}", ev);
        }
    }
}
