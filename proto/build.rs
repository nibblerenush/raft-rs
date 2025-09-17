// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

use protobuf_codegen::{Codegen, Customize};
use std::path::PathBuf;

fn main() {
    let base = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let proto_root = PathBuf::from(&base).join("proto");
    let include_dir = proto_root.clone();

    let mut inputs = Vec::<PathBuf>::new();
    for e in walkdir::WalkDir::new(&proto_root)
        .into_iter()
        .filter_map(Result::ok)
    {
        if e.file_type().is_file() && e.path().extension().and_then(|s| s.to_str()) == Some("proto")
        {
            inputs.push(e.path().to_owned());
            println!("cargo:rerun-if-changed={}", e.path().display());
        }
    }

    println!("cargo:rerun-if-changed={}", include_dir.display());

    let protoc = protoc_bin_vendored::protoc_bin_path().expect("vendored protoc");
    let google_includes = protoc_bin_vendored::include_path().expect("vendored google protos");

    let customize = Customize::default()
        .tokio_bytes(true)
        .generate_accessors(true)
        .gen_mod_rs(true);

    Codegen::new()
        .protoc()
        .protoc_path(&protoc)
        .includes(&[include_dir, google_includes])
        .inputs(&inputs)
        .customize(customize)
        .cargo_out_dir("protos")
        .run_from_script();
}
