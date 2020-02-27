# Paris | [crates.io](https://crates.io/crates/paris) | ![CI](https://github.com/SirTheViking/logger/workflows/CI/badge.svg)
Simple way to output beautiful text in your
CLI applications. Only limit is your imagination.


## How to use
```rust
use paris::Logger;

// false to exclude timestamps
let mut log = Logger::new(false);

log.info("It's that simple!");
```


## Simple methods
```rust
// You can have icons at the start of your message!
log.info("Will add ℹ at the start");
log.error("Will add ✖ at the start");
```
See [the Logger struct](./struct.Logger.html) for all methods


## Chaining
All methods can be chained together to build more intricate
log/message combinations, in hopes of minimizing the chaos
that every log string becomes when you have to concatenate
a bunch of strings and add tabs and newlines everywhere.
```rust
log.info("this is some info")
   .indent(4).warn("this is now indented by 4")
   .newline(5)
   .success("and this is 5 lines under all other messages");
```


## Customisation
Outputting text is cool. Outputting text with a colored icon
at the start is even cooler! But this crate is all about
customisation, about making the logs feel like home, if you will.
Included in the crate are a variety of keys you can use
to colorize your logs just the way you want them to be.
```rust
log.info("I can write normal text or use tags to <red>color it</>");
log.warn("Every function can contain <on green><black>tags</>");

log.info("If you don't write them <bleu>correctly</>, you just get the default colors");
```

There's a key for all colors supported by the terminal `(white, black, red, blue, magenta, etc.)`
If you add the word `on` to any of those colors, it becomes the
background color instead `(on red, on blue, on green)`.
```rust
// How useful...
log.info("<on red> This has red background </>");
```

Maybe you'd like to use your terminals brighter colors, if that's the case
you just have to add `bright` to your tag. Makes sense.
```rust
log.info("<blue><on bright red> This text is blue on a bright red background</> it's a pain");
```

###### Scroll down for a full list of keys if you're not feeling confident in your ability to name colors. It happens.

You've probably seen the `</>` tag in the above logs. It's not there to
_"close the previously opened tag"_ no no. You can open as many tags as you want
and only use `</>` it's just the _"reset color to default"_ tag, You might
decide you don't ever want to use it. It's up to you.


## Color keys
To use a key just add the color name surrounded by `<`, `>` to your log string. Include spaces
or use underlines instead if you wish.

#### Foreground
`black`, `red`, `green`, `yellow`, `blue`, `cyan`, `magenta`, `white`

#### Background
`on black`, `on red`, `on green`, `on yellow`, `on blue`, `on cyan`, `on magenta`, `on white`

#### Styles (Not Implemented Yet)
`bold`, `underline`, `dimmed`, `italic`, `strikethrough`,  