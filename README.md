# logging-bot
Discord bot to send command line output to Discord channels

## usage
```shell
cargo run -- path_to_script mode arguments
```

the bot supports `.sh`, `.bat` and `.cmd` files

use `stdout` or `stderr` to specify the output mode

`arguments` is optional, use this to give input arguments to the script you want to log

use the `!log` command in a Discord channel to start logging
