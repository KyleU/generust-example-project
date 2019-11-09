#![feature(option_expect_none)]
#![feature(fn_traits)]
#![feature(unboxed_closures)]
#![warn(anonymous_parameters)]
#![warn(bare_trait_objects)]
#![warn(elided_lifetimes_in_paths)]
#![warn(missing_debug_implementations)]
#![warn(single_use_lifetimes)]
#![warn(trivial_casts)]
#![warn(unreachable_pub)]
#![warn(unsafe_code)]
#![warn(unused_extern_crates)]
#![warn(unused_import_braces)]
#![warn(unused_qualifications)]
#![warn(unused_results)]
#![warn(variant_size_differences)]
#![warn(missing_docs)]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/generust-example-project/generust-example-project/master/crates/assets/embed/favicon.ico")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/generust-example-project/generust-example-project/master/crates/assets/embed/favicon.png")]
#![doc(issue_tracker_base_url = "https://github.com/generust-example-project/generust-example-project/issues/")]
#![windows_subsystem = "windows"]

//! `generust-example-project` is a work in progress. Docs soon.
//! - [generust-example-project-assets](generust_example_project_assets)
//! - [generust-example-project-client](generust_example_project_client)
//! - [generust-example-project-controllers](generust_example_project_controllers)
//! - [generust-example-project-core](generust_example_project_core)
//! - [generust-example-project-service](generust_example_project_service)
//! - [generust-example-project-templates](generust_example_project_templates)

mod app;
mod args;
mod cfg;
mod log;
mod server;

#[cfg(test)]
pub mod tests;

/// Application entrypoint, creates and starts the server
pub fn go() -> generust_example_project_core::Result<()> {
  let cfg = crate::cfg::cfg_from_args();
  crate::app::start(cfg)
}

/// External app entrypoint, calls `go()` directly and swallows errors
#[no_mangle]
pub extern "C" fn libgo() {
  match go() {
    Ok(_) => println!("Successfully started [{}]", generust_example_project_core::APPNAME),
    Err(e) => println!("Error starting [{}]: {}", generust_example_project_core::APPNAME, e)
  };
}

#[cfg(target_os = "android")]
#[allow(non_snake_case)]
pub mod android {
  extern crate jni;

  use self::jni::objects::JClass;
  use self::jni::JNIEnv;
  use super::go;

  #[no_mangle]
  #[allow(unsafe_code)]
  pub unsafe extern "C" fn Java_com_generust-example-project_generust_example_project_generust_example_project_go(env: JNIEnv<'_>, _: JClass<'_>) {
    println!("Android!");
    go();
  }
}
