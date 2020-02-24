### TODO:
* *"parse"* function to build custom combinations and styles


# A logger for CLI applications | [crates.io](https://crates.io/crates/paris) | ![Build](https://github.com/SirTheViking/logger/workflows/Logger%20integration%20tests/badge.svg)

#### Basic example
```rust
use logger::Logger;

let logger = Logger::new(false);

logger.info("This is a very basic example");
// ℹ This is a very basic example
```

#### API

The crate is built so that everything can be chained as many times as you need.
Maybe you want to start a very intensive task, show a loading animation
and as soon as that task is finished show a success message and then another
loading animation:
```rust
use logger::Logger;

let logger = Logger::new(false);

logger.loading("Doing a lot of things now...");

// Do the things

logger
    .success("The things I did went well!")
    .loading("But now I'm doing more things");

// Do more things

logger.error("Something broke this time...");
``` 

If you want a timestamp with all your logs you can pass `true` to the
constructor
```rust
let logger = Logger::new(true); // true means "add timestamp"
```

Here's a list of all the functions (they're not that many)

```rust
logger.info("message");     // ℹ message

logger.error("message");    // ✖ message

logger.warn("message");     // ⚠ message

logger.success("message");  // ✔ message

logger.log("message");      // message

logger.done(); // Just stops the loading animation

logger
    .indent(3)
    .info("Indent just adds specified amount of tabs");
// \t\t\t ℹ Indent just adds specified amount of tabs

logger.newline(2).info("Newline is like .indent but with newlines");
// \n\n ℹ Newline is like .indent but with newlines

logger.same().log(".same() forces the next log to not have a newline after it");

```

To start a loading animation you can run `.loading()` and to end that
animation you can run any other function (except for `.same()`) and the 
text that was visible when loading will be replaced with your new log
```rust
logger.loading("Loading animation!!");

// will replace "Loading animation!!" with a success message
logger.success("Loading was a success.");
```
