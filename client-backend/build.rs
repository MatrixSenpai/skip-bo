fn main() {
    cynic_codegen::register_schema("skip_bo")
        .from_sdl_file("schemas/sb.graphql")
        .unwrap()
        .as_default()
        .unwrap();

    tauri_build::build();
}
