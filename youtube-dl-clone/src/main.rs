
const PKG: &str = env!("CARGO_PKG_VERSION");
const PKG_NAME: &str = env!("CARGO_PKG_NAME");

fn main() {
    println!("{PKG_NAME} v{PKG}");

}
