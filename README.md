# MsgFilter
### A simple bot to filter messages that contain bad words

Invite [here](https://bit.ly/msgfilter)


## Self Hosting 

1. Install `gcc`
2. Clone this repository
3. Make sure you have rust installed (If not go [here](https://rustup.rs) to install)
4. `cd` into repository
5. Create a `config.json` in the root of the repository 
```js
  {
   "token": "<YOUR BOT TOKEN HERE>",
    "prefix": "<YOUR DESIRED PREFIX>",
    "shards": 1 // <- This could be any number [REMOVE THIS COMMENT]
  }
```
6. Run `cargo run` or `cargo build --release` to build for your operating system
7. If you used cargo build, move the binary in `./target/release/msg-filter` into the root of the repo