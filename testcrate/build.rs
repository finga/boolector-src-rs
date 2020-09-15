extern crate boolector_src;

fn main() {
    let boolector = boolector_src::Build::new().prerequisites().build();
    println!(
        "cargo:rustc-link-search=native={}",
        boolector.lib_dir().display()
    );
    println!("cargo:rustc-link-lib=static={}", boolector.lib());
}
