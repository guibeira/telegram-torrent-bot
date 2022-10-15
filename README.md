# movie-bot
A telegram bot to search and download your movies
![sample](https://raw.githubusercontent.com/GuilhermeVBeira/telegram-torrent-bot/main/sample.png)

## Quickstart

create your bot using the [botfather](https://t.me/botfather)

- This project uses [rqbit](https://github.com/ikatson/rqbit) as download server, make sure you have it running


### Installation Instructions
```
wget https://github.com/GuilhermeVBeira/telegram-torrent-bot/releases/download/1.0/movie-bot
sudo cp movie-bot /usr/local/bin/

```

### Build from source code

```sh
git clone git@github.com:GuilhermeVBeira/telegram-torrent-bot.git
cd telegram-torrent-bot
cargo build --realease
sudo cp target/release/movie-bot /usr/local/bin

```

### Running 

Export your telegram token 
```
export TELOXIDE_TOKEN=<telegram-token>
```

then run:

```sh
movie-bot
```
