use std::{
    env,
    path::{Path, PathBuf},
};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=wrapper.h");

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let workspace_root = manifest_dir
        .parent()
        .and_then(Path::parent)
        .expect("crates/simapi-sys should be under workspace/crates")
        .to_path_buf();

    let simapi_src = workspace_root.join("third_party/simapi");

    assert!(
        simapi_src.exists(),
        "simapi source not found at {}",
        simapi_src.display()
    );

    let dst = cmake::Config::new(&simapi_src).profile("Release").build();

    let include_dirs = [
        simapi_src.join("simapi"),
        simapi_src.join("include"),
        dst.join("include"),
    ];

    // CMake usually installs libraries into either lib or lib64.
    let lib_dirs = [dst.join("lib"), dst.join("lib64")];

    for dir in &lib_dirs {
        if dir.exists() {
            println!("cargo:rustc-link-search=native={}", dir.display());
        }
    }

    // Upstream library name is expected to be libsimapi.{so,a}.
    println!("cargo:rustc-link-lib=simapi");

    // On Linux, simapi uses POSIX APIs. These may be needed depending on how
    // the upstream CMake links things.
    println!("cargo:rustc-link-lib=dl");
    println!("cargo:rustc-link-lib=pthread");

    let mut builder = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .derive_debug(true)
        .derive_default(true)
        .generate_comments(true)
        .allowlist_type("SimData")
        .allowlist_type("LapTime")
        .allowlist_type("CarData")
        .allowlist_type("ProximityData")
        .allowlist_type("SimInfo")
        .allowlist_type("SimMap")
        .allowlist_type("SimulatorAPI")
        .allowlist_type("SimulatorEXE")
        .allowlist_type("SimAPIError")
        .allowlist_type("SimDataType")
        .allowlist_type("SIMAPI_.*")
        .allowlist_var("SIMAPI_VERSION")
        .allowlist_var("MAXCARS")
        .allowlist_var("PROXCARS")
        .allowlist_function("simapi_.*")
        .allowlist_function("is_pid_running")
        .layout_tests(true);

    for include_dir in &include_dirs {
        if include_dir.exists() {
            builder = builder.clang_arg(format!("-I{}", include_dir.display()));
        }
    }

    let bindings = builder
        .generate()
        .expect("failed to generate simapi bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("failed to write bindings.rs");
}
