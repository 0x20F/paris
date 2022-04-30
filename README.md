<h1 align="center">paris</h1>

<p align="center">Simple way to output beautiful text in your CLI applications. Only limit is your imagination.</p>


<p align="center">
   <img alt="Build status badge" src="https://github.com/0x20F/paris/workflows/build/badge.svg"/>
   <img alt="Crates io badge" src="https://img.shields.io/crates/v/paris.svg"/>
   <img alt="Repository version badge" src="https://img.shields.io/badge/master-v1.5.11-blue.svg">
   <a href="https://opensource.org/licenses/MPL-2.0"><img alt="License badge" src="https://img.shields.io/badge/License-MPL%202.0-brightgreen.svg"/></a>
</p>


<p align="center">
   <img src="https://github.com/0x20F/paris/blob/master/example/paris_example.gif?raw=true"/>
</p>

<br/>
<br/>

<h2 align="center">Notice</h2>

If the provided _logging_ API is too simple for your use case (it's definitely not meant for anything too complex) but you still like the formatting style (`<blue></>` etc) then you can still make use of this crate! See [here](https://github.com/0x20F/paris/issues/38#issuecomment-950874653) for a quick example.

By importing the `format` module, you get access to the `Formatter` structure itself. This is what the `Logger` uses in the back end anyway, you're basically stripping away the API.

For even more ease of use the `formatter` module includes a `colorize_string` function which you can use to directly colorize whatever without initializing the formatter. Drawbacks and caveats may apply, read the docs.

###### See [the formatter module](https://docs.rs/paris/1.5.7/paris/formatter/index.html) for more info if needed.
 

<br/>

## How to use
```toml
[dependencies]
paris = "1.5"
```

```rust
use paris::Logger;

let mut log = Logger::new();

log.info("It's that simple!");
```

#### Optional features

##### Timestamps 
If you'd like timestamps with all your logs you'll
have to enable the feature when adding the crate as a dependency. 

Notice: This will also include `chrono` as a dependency.
```toml
[dependencies]
paris = { version = "1.5", features = ["timestamps"] }
```

##### Macros
Every common function has a macro. To make use of these
macros you'll need to enable the macros feature.
```toml
[dependencies]
paris = { version = "1.5", features = ["macros"] }
```

##### No logger
If you'd prefer to only use the macros and not even have
the `Logger` struct included in your package, that's definitely possible!
All you need to do is enable the `no_logger` feature.
```toml
[dependencies]
paris = { version = "1.5", features = ["no_logger"] }
```


### Simple API
```rust
// You can have icons at the start of your message!
log.info("Will add ℹ at the start");
log.error("Will add ✖ at the start");

// or as macros
info!("Will add ℹ at the start");
error!("Will add ✖ at the start");
```
###### See [the Logger struct](https://docs.rs/paris/) for all methods and their macro equivalents


### Chaining
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


### Customisation
Outputting text is cool. Outputting text with a colored icon
at the start is even cooler! But this crate is all about
customisation, about making the logs feel like home, if you will.
Included in the crate are a variety of keys you can use
to colorize your logs just the way you want them to be.
```rust
log.info("I can write normal text or use tags to <red>color it</>");
log.warn("Every function can contain <on-green><black>tags</>");

log.info("If you don't write them <bleu>correctly</>, you just get an ugly looking tag");
```

There's a key for all colors supported by the terminal `(white, black, red, blue, magenta, etc.)`
If you add the word `on` to any of those colors, it becomes the
background color instead `(on-red, on-blue, on-green)`.
```rust
// How useful...
log.info("<on-red> This has red background</>");
```

Maybe you'd like to use your terminals brighter colors, if that's the case
you just have to add `bright` to your tag. Makes sense.
```rust
log.info("<blue><on-bright-red> This text is blue on a bright red background</> it's a pain");
```

If you feel like writing a lot of colors by hand is too tedious, or if you know you're going
to be using the same combination of colors over and over again you can create a `custom style`
that encapsulates all those colors.
```rust
log.add_style("lol", vec!["green", "bold", "on-bright-blue"]);

// '<lol>' is now a key that you can use in your strings
log.info("<lol>This is has all your new styles</>");
```

###### Scroll down for a full list of keys if you're not feeling confident in your ability to name colors. It happens.


### Resetting
You've probably seen the `</>` tag in the above logs. It's not there to
_"close the previously opened tag"_ no no. You can open as many tags as you want
and only use `</>` once, it's just the _"reset everything to default"_ tag, You might
decide you don't ever want to use it. It's up to you.

However, resetting everything to default might not be what you want. Most of the time
it'll be enough, but for those times when it isn't there are a few other tags such as:

* `<///>` only resets the background
* `<//>` only reset the foreground


### Macros
With the macros feature enabled, you get access to macro equivalents
of the logger functions.

Advantages of using macros:
* You don't have to instantiate the logger `Logger::new()`
* Simple to write
* Can format parameters like `print!` and `println!`

Disadvantages of using macros:
* Can't chain calls
* Manual newlines and tabs with `\n` and `\t`
* There's no loading animation for macros

You get to decide whether you want to use macros or not.
Every macro has the same functionality as its `Logger`
equivalent. Colors and icon keys work just the same.
###### See [the Logger struct](https://docs.rs/paris/) for all methods and their macro equivalents


## Color keys
To use a key just add the color name surrounded by `<`, `>` to your log string. If you don't like the dashes(`-`),
you can use underlines(`_`) or spaces(` `)

#### Foreground
`black`, `red`, `green`, `yellow`, `blue`, `cyan`, `magenta`, `white`

##### Bright
`bright-black`, `bright-red`, `bright-green`, `bright-yellow`, `bright-blue`, `bright-cyan`, `bright-magenta`,
 `bright-white`


#### Background
`on-black`, `on-red`, `on-green`, `on-yellow`, `on-blue`, `on-cyan`, `on-magenta`, `on-white`

##### Bright
`on-bright-black`, `on-bright-red`, `on-bright-green`, `on-bright-yellow`, `on-bright-blue`, `on-bright-cyan`, 
`on-bright-magenta`, `on-bright-white`

#### Styles
`bold`(`b`), `underline`(`u`), `dimmed`(`d`), `italic`(`i`), `blink`(`l`), `reverse`(`r`), `hidden`(`h`), `strikethrough`(`s`)

Styles are a bit different, they all have their usual keys, the long and painful to write ones. But they 
also have shorthand keys (in parenthesis).

And while they all may reset using one of the reset keys above, if you're looking to turn off a specific
style you've opened, you can just use the exact same key but with a slash `/` in front of it.

Example: `<bold>` gets closed by `</bold>` 

#### Icons
`info`, `cross`, `warn`, `tick`, `heart`
