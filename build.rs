use protox::prost::Message;
use std::{fs::File, io::Write, path::PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=protos/todo.proto");
    let file_descriptor = protox::Compiler::new(["protos"])
        .unwrap()
        .include_imports(true)
        .include_source_info(true)
        .open_file(PathBuf::from("protos/todo.proto"))
        .unwrap()
        .file_descriptor_set();
    let out = PathBuf::from("src/pb");

    // 将 file_descriptor 输出到文件中
    let descriptor_file = out.join("todo_descriptors.bin");
    let mut descriptor_file = File::create(descriptor_file).unwrap();
    let descriptor_encode = file_descriptor.encode_to_vec();
    descriptor_file.write_all(&descriptor_encode).unwrap();

    tonic_prost_build::configure()
        .out_dir(out)
        .compile_fds(file_descriptor)
        .unwrap();
}
