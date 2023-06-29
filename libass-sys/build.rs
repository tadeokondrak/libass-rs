extern crate bindgen;
extern crate metadeps;

use std::path::PathBuf;
use std::{env, fs};

fn main() {
    let libs = metadeps::probe().unwrap();
    let headers = libs.get("libass").unwrap().include_paths.clone();

    let mut builder = bindgen::builder()
        .header("data/libass.h")
        .allowlist_function("^ass_.*")
        .blocklist_function("ass_set_message_cb")
        .allowlist_type("^(ASS|ass).*")
        .blocklist_type("__.*")
        .blocklist_type("va_list")
        .bitfield_enum("ASS_OverrideBits")
        .rustified_enum("ASS_DefaultFontProvider")
        .rustified_enum("ASS_Hinting")
        .rustified_enum("ASS_YCbCrMatrix")
        .rustified_enum("ASS_ShapingLevel")
        .rustified_enum("IMAGE_TYPE.*")
        .rustified_enum("TRACK_TYPE.*");

    for header in headers {
        builder = builder.clang_arg("-I").clang_arg(header.to_str().unwrap());
    }

    // Manually fix the comment so rustdoc won't try to pick them
    let s = builder
        .generate()
        .unwrap()
        .to_string()
        .replace("/**", "/*")
        .replace("/*!", "/*");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    fs::write(out_path.join("libass.rs"), s).unwrap();
}
