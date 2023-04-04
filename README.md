# tae
Opens urls based on rules in config file

## Usage
To use, just execute tae with one or more urls as the argument. The program will
expect a config file at `$XDG_CONFIG_HOME/tae/tae.toml` or in the current
directory as `tae.toml`. tae will then open the links with the specified
commands.

## Configuration
The config file consists of a list of rules for opening links.

Here is an example where youtube is opened in mpv, all other websites are opened
in firefox, and gemini capsules are openen in amfora.
```toml
[[rules]]
domain = "youtube.com"
command = "mpv"

[[rules]]
scheme = "https?"
command = "firefox"

[[rules]]
scheme = "gemini"
command = "amfora"
```

tae will check is rules one by one until is finds a rule that matches. When the
first match is found, the corresponding command will be run with the url
appended.

Each rules is comprised of a list of attributes that can be tested. Each
attribute corresponds with a part of the url. This part will be checked to see
if the given regex matches.

| Attribute |                     Example                    |
|:---------:|:----------------------------------------------:|
|    url    | https://github.com/jo1gi/tae?a=1#configuration |
|   scheme  |                      https                     |
|   domain  |                   github.com                   |
|    path   |                   /jo1gi/tae                   |
|   query   |                       a=1                      |
|  fragment |                  configuration                 |
