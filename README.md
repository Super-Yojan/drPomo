# drPomo

drPomo is a simple Pomodoro timer written in Rust. You can use it on multiple
tools like polybar, tmux etc.


## Build / Compile

```
git clone https://github.com/Super-Yojan/drPomo
cd drPomo
```
```
cargo build --release 
cp target/release/pomo ~/.local/bin
```

## Configs

### Config for Tmux 
``` tmux
set -g status-right "#(pomo <session-time> <break-time>)"
```

### Config for polybar
```

[module/pomodoro]
type = custom/script
exec = ~/.local/bin/pomo 25 5 
tail = true
```


### ToDo

- [ ] Read Notification from config file

