# src
This is my first rust project lol

I've been using [colored](https://crates.io/crates/colored) to make everything pretty.
## cli
This file contains code relating to user input.

It is responsible for declaring how the user will interact with the tool, getting input 
from the user and returning it in a way which is nice to use for my other functions.

I'm quite happy with how this is working so far.

Using [clap](https://crates.io/crates/clap), [rprompt](https://crates.io/crates/rprompt) and [rpassword](https://crates.io/crates/rpassword).

## client
This file contains code relating to doing http requests and submitting code etc.

Currently, I've been able to
 - Log in to my profile using username and password
 - Store cookies
 - Load cookies again
 - Parse problems and store them in folders (omg it's so fast also)

I don't think code submission will be too hard. (for another day)

I still want to implement "cf race", and then also the results of a submission. 
(I might have to wait until submission is done?? how??)

Using [reqwest](https://crates.io/crates/reqwest), [reqwest-cookie-store](https://crates.io/crates/reqwest_cookie_store), 
[scraper](https://crates.io/crates/scraper).

## config
This file (kinda) contains code relating to configuration.

The functions here actually _do_ the configuring and the reading and writing of files etc.

[serde](https://crates.io/crates/serde_derive) and [serde_json](https://crates.io/crates/serde_json) has been super useful.

## coderunner
This file (todo) contains code relating to running user code.

The functions here actually _do_ the commands.

## main.rs
The entry point.
