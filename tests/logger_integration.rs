use paris::Logger;
use paris::LogIcon;


#[test]
fn log() {
    let mut logger = Logger::new(false);
    logger.log("This is the most basic of texts.");
}


#[test]
fn info() {
    let mut logger = Logger::new(false);
    logger.info("This is some info, it should work");
}


#[test]
fn success() {
    let mut logger = Logger::new(true);
    logger.success("You did it and nothing broke!!");
}


#[test]
fn error() {
    let mut logger = Logger::new(false);
    logger.error("This is an error, but test should still work");
}


#[test]
fn warning() {
    let mut logger = Logger::new(true);
    logger.warn("This is a warning, watch it");
}


#[test]
fn loading() {
    let mut logger = Logger::new(false);
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