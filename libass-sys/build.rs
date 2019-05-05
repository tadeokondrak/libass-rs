extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=ass");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .whitelist_function("^ass_.*")
        .blacklist_function("ass_set_message_cb")
        .whitelist_type("^(ASS|ass).*")
        .blacklist_type("__.*")
        .blacklist_type("va_list")
        .bitfield_enum("ASS_OverrideBits")
        .rustified_enum("ASS_DefaultFontProvider")
        .rustified_enum("ASS_Hinting")
        .rustified_enum("ASS_YCbCrMatrix")
        .rustified_enum("ASS_ShapingLevel")
        .rustified_enum("IMAGE_TYPE.*")
        .rustified_enum("TRACK_TYPE.*")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
