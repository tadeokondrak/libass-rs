extern crate bindgen;
extern crate metadeps;

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    let libs = metadeps::probe().unwrap();
    let headers = libs.get("libass").unwrap().include_paths.clone();

    let mut builder = bindgen::builder()
        .header("data/libass.h")
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

    let mut file = File::create(out_path.join("libass.rs")).unwrap();

    let _ = file.write(s.as_bytes());
}
