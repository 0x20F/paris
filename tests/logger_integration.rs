use paris::Logger;
use paris::LogIcon;

#[cfg(feature = "macros")]
use paris::{ info, success, error, warn };


#[test]
fn log() {
    let mut logger = Logger::new();
    logger.log("This is the most basic of texts.");
}


#[test]
fn info() {
    let mut logger = Logger::new();
    logger.info("This is some info, it should work");

    #[cfg(feature = "macros")]
    info!("This is some info from a macro");
}


#[test]
fn success() {
    let mut logger = Logger::new();
    logger.success("You did it and nothing broke!!");

    #[cfg(feature = "macros")]
    success!("This is some success from a macro");
}


#[test]
fn error() {
    let mut logger = Logger::new();
    logger.error("This is an error, but test should still work");

    #[cfg(feature = "macros")]
    error!("This is an error from the macro");
}


#[test]
fn warning() {
    let mut logger = Logger::new();
    logger.warn("This is a warning, watch it");

    #[cfg(feature = "macros")]
    warn!("This is a warning from the macro!");
}


#[test]
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