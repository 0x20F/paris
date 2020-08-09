#[cfg(not(feature = "no_logger"))]
use paris::Logger;

#[cfg(feature = "macros")]
use paris::{error, info, success, warn};

use paris::LogIcon;

#[test]
#[cfg(not(feature = "no_logger"))]
fn add_custom_styles() {
    let mut logger = Logger::new();
    logger.add_style("lol", vec!["green", "bold", "on_blue"]);

    logger.log("<lol>This is custom colored</>");
}

#[test]
#[cfg(not(feature = "no_logger"))]
fn log() {
    let mut logger = Logger::new();
    logger.log("This is the most basic of texts.");
}

#[test]
#[cfg(not(feature = "no_logger"))]
fn info() {
    let mut logger = Logger::new();
    logger.info("This is some info, it should work");
}

#[test]
#[cfg(feature = "macros")]
fn info_macro() {
    info!("This is some info from a macro");
}

#[test]
#[cfg(not(feature = "no_logger"))]
fn success() {
    let mut logger = Logger::new();
    logger.success("You did it and nothing broke!!");
}

#[test]
#[cfg(feature = "macros")]
fn success_macro() {
    success!("This is some success from a macro");
}

#[test]
#[cfg(not(feature = "no_logger"))]
fn error() {
    let mut logger = Logger::new();
    logger.error("This is an error, but test should still work");
}

#[test]
#[cfg(feature = "macros")]
fn error_macro() {
    error!("This is an error from the macro");
}

#[test]
#[cfg(not(feature = "no_logger"))]
fn warning() {
    let mut logger = Logger::new();
    logger.warn("This is a warning, watch it");
}

#[test]
#[cfg(feature = "macros")]
fn warning_macro() {
    warn!("This is a warning from the macro!");
}

#[test]
#[cfg(not(feature = "no_logger"))]
fn loading() {
    let mut logger = Logger::new();
    logger.loading("Parsing 500 files");
    logger.success("Parsed 500 files"); // Should call done automatically

    // Showing that .done() can also be called
    logger.loading("Parsing another 500 files");
    logger.done().error("Failed parsing another 500 files");
}

#[test]
fn icons() {
    println!(
        "{} All {} Of {} Them {} At {} Once",
        LogIcon::Info,
        LogIcon::Tick,
        LogIcon::Cross,
        LogIcon::Warning,
        LogIcon::Heart
    );
}
