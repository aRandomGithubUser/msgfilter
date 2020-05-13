# MsgFilter
### A simple bot to filter messages that contain bad words

Invite [here](https://github.com)


## Self Hosting 
1. Clone this repository
2. Make sure you have rust installed (If not go [here](https://rustup.rs) to install)
3. `cd` into repository
4. Create a config.json in the root of the repository 
    ```json
      {
        "token": "<YOUR BOT TOKEN HERE>",
        "prefix": "<YOUR DESIRED PREFIX>",
        "shards": 1 // This could be any number
      }
    ```
5. Run `cargo run` or `cargo build` to build for your operating system