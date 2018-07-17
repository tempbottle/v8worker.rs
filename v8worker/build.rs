extern crate cc;
use std::env;
use std::path::PathBuf;
use std::string::String;

#[cfg(target_os = "linux")]
fn get_v8lib_path() -> String {
    let cwd = env::current_dir().unwrap();
    let mut path = cwd.clone();
    path.push("v8-libs");
    path.push("linux");
    String::from(path.to_str().unwrap())
}

#[cfg(target_os = "macos")]
fn get_v8lib_path() -> String {
    let cwd = env::current_dir().unwrap();
    let mut path = cwd.clone();
    path.push("v8-libs");
    path.push("macos");
    String::from(path.to_str().unwrap())
}

#[cfg(target_env = "msvc")]
fn get_v8lib_path() -> String {
    let cwd = env::current_dir().unwrap();
    let mut path = cwd.clone();
    path.push("v8-libs");
    path.push("win64-msvc");
    String::from(path.to_str().unwrap())
}

#[cfg(target_env = "msvc")]
fn link_msvc() {
    println!("cargo:rustc-link-lib=DbgHelp");
    println!("cargo:rustc-link-lib=Winmm");
    println!("cargo:rustc-link-lib=Shlwapi");
    println!("cargo:rustc-link-lib=static=v8_base_0");
    println!("cargo:rustc-link-lib=static=v8_base_1");
    println!("cargo:rustc-link-lib=static=v8_base_2");
    println!("cargo:rustc-link-lib=static=v8_base_3");
}

#[cfg(not(target_env = "msvc"))]
fn link_gcc() {
    println!("cargo:rustc-link-lib=static=v8_base");
}

// cargo build -vv to dispaly all console output, include C/C++ compiler output.
// on windows, use chcp 65001 to change language CodePage to utf-8
fn main() {
    let v8_path = PathBuf::from(&get_v8lib_path());
    let incl_path = v8_path.join("include");
    let lib_path = v8_path.join("lib");

    let mut build = cc::Build::new();

    #[cfg(target_env = "msvc")]
    {
        build
            .cpp(true)
            .flag("/EHsc")
            .warnings_into_errors(false)
            .warnings(false)
            .include(incl_path)
            .flag("/std:c++14")
            .file("src/binding.cc")
            .compile("binding");

        link_msvc();
    }

    #[cfg(not(target_env = "msvc"))]
    {
        build
            .cpp(true)
            .warnings_into_errors(false)
            .warnings(false)
            .include(incl_path)
            .flag("-std=c++11")
            .flag("-fPIC")
            .file("src/binding.cc")
            .compile("binding");

        link_gcc();
    }

    //use v8libs build from nodejs deps/v8 codebase, build with vcbuild.bat on windows
    println!(
        "cargo:rustc-link-search=native={}",
        lib_path.to_str().unwrap()
    );
    println!("cargo:rustc-link-lib=static=icudata");
    println!("cargo:rustc-link-lib=static=icui18n");
    println!("cargo:rustc-link-lib=static=icustubdata");
    //println!("cargo:rustc-link-lib=static=icutools");
    println!("cargo:rustc-link-lib=static=icuucx");
    println!("cargo:rustc-link-lib=static=v8_init");
    println!("cargo:rustc-link-lib=static=v8_initializers");
    println!("cargo:rustc-link-lib=static=v8_libbase");
    println!("cargo:rustc-link-lib=static=v8_libplatform");
    println!("cargo:rustc-link-lib=static=v8_libsampler");
    println!("cargo:rustc-link-lib=static=v8_nosnapshot");
    //println!("cargo:rustc-link-lib=static=v8_snapshot");
}
