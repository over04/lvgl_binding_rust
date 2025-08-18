use std::env;
use std::path::{Path, PathBuf};

fn main() {
    let project_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let shims_dir = project_dir.join("shims");
    let vendor = project_dir.join("vendor");
    let lvgl_src = vendor.join("lvgl").join("src");
    let incl_extra =
        env::var("LVGL_INCLUDE").unwrap_or("/usr/include,/usr/local/include".to_string());
    let lv_config_dir = env::var("DEP_LV_CONFIG_PATH")
        .map(PathBuf::from)
        .expect("Please Set DEP_LV_CONFIG_PATH");
    let lv_conf = lv_config_dir.join("lv_conf.h");
    if !lv_conf.exists() {
        panic!("lv_conf.h not found in $DEP_LV_CONF_PATH");
    } else {
        println!("cargo:rerun-if-changed={}", lv_conf.to_str().unwrap());
    }

    let mut cfg = cc::Build::new();
    add_c_files(&mut cfg, &lvgl_src);
    add_c_files(&mut cfg, &lv_config_dir);
    add_c_files(&mut cfg, &shims_dir);

    cfg.define("LV_CONF_INCLUDE_SIMPLE", Some("1"))
        .include(&lvgl_src)
        .include(&vendor)
        .include(&lv_config_dir);
    cfg.includes(incl_extra.split(','));

    cfg.warnings(false).compile("lvgl");

    let mut cc_args: Vec<String> = vec![
        "-DLV_CONF_INCLUDE_SIMPLE=1".into(),
        "-I".into(),
        lv_config_dir.to_str().unwrap().into(),
        "-I".into(),
        vendor.to_str().unwrap().into(),
        "-fvisibility=default".into(),
    ];
    #[cfg(feature = "sdl2")]
    cc_args.push("-DINCLUDE_SDL=1".into());

    for include in incl_extra.split(',') {
        cc_args.push("-I".into());
        cc_args.push(include.into());
    }

    if let Ok(sysroot) = env::var("LVGL_SYSROOT") {
        cc_args.push("--sysroot".into());
        cc_args.push(sysroot);
    }
    let target = env::var("TARGET").expect("Cargo build scripts always have TARGET");
    let host = env::var("HOST").expect("Cargo build scripts always have HOST");
    if target != host {
        cc_args.push("-target".into());
        cc_args.push(target);
    }

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    println!(
        "cargo:rerun-if-changed={}",
        shims_dir.join("lvgl_sys.h").to_str().unwrap()
    );
    let bindings =
        bindgen::Builder::default().header(shims_dir.join("lvgl_sys.h").to_str().unwrap());
    let bindings = bindings
        .generate_comments(false)
        .derive_default(true)
        .layout_tests(false)
        .use_core()
        .clang_args(&cc_args)
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Can't write bindings!");
}

fn add_c_files(build: &mut cc::Build, path: impl AsRef<Path>) {
    for e in path.as_ref().read_dir().unwrap() {
        let e = e.unwrap();
        let path = e.path();
        if e.file_type().unwrap().is_dir() {
            add_c_files(build, e.path());
        } else if path.extension().and_then(|s| s.to_str()) == Some("c") {
            build.file(&path);
        }
    }
}
