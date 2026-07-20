fn main() {
    println!("cargo:rerun-if-changed=schema.graphql");
    cynic_codegen::register_schema("prita")
        .from_sdl_file("schema.graphql")
        .expect("read schema.graphql")
        .as_default()
        .expect("set default schema");
}
