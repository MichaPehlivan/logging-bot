# logging-bot
Discord bot to send cli output to Discord channels

## usage
```shell
cargo run -- path_to_script mode
```

the bot supports `.sh`, `.bat` and `.cmd` files
use `stdout` or `stderr` as arguments to specify the output mode

use the `!log` command in a Discord channel to start logging

## todo:
- add configuarion for stdout and stderr 
- make user input and commands possible
- allow command inputs before starting up
