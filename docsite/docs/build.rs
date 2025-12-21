use mdbook_gen::generate_router_build_script;
use std::{env::current_dir, path::PathBuf};

fn main() {
    // re-run only if the "example-book" directory changes
    println!("cargo:rerun-if-changed=./docs-src/");

    make_docs();
}

fn make_docs() {
    let mdbook_dir = PathBuf::from("./docs-src");
    let out_dir = current_dir().unwrap().join("src/docs");
    let mut out = generate_router_build_script(mdbook_dir);
    out.push_str("\nuse super::*;\n");

    let filename = format!("router.rs");

    // Don't try to trap this error - if the directory already exists, we don't care.
    // If the directory can't be created, then the `write` call afterwards will fail, and we'll
    // still see the error.
    let _ = std::fs::create_dir(&out_dir);
    std::fs::write(out_dir.join(filename), out).unwrap();
}
