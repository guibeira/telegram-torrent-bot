mod piratebay;
extern crate reqwest;
use reqwest::header::HeaderMap;

use teloxide::{
    dispatching::{dialogue, dialogue::InMemStorage, UpdateHandler},
    prelude::*,
    types::{InlineKeyboardButton, InlineKeyboardMarkup},
    utils::command::BotCommands,
};

type MyDialogue = Dialogue<State, InMemStorage<State>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[derive(Clone, Default)]
pub enum State {
    #[default]
    Start,
    ReceiveMovieName,
    ReceiveMovieChoice {
        movie_name: String,
    },
}

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
enum Command {
    #[command(description = "Mostra esse dialogo")]
    Help,
    #[command(description = "Iniciar busca.")]
    Start,
    #[command(description = "Cancelar busca.")]
    Cancel,
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting movie bot...");

    let bot = Bot::from_env();

    Dispatcher::builder(bot, schema())
        .dependencies(dptree::deps![InMemStorage::<State>::new()])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

fn schema() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    use dptree::case;

    let command_handler = teloxide::filter_command::<Command, _>()
        .branch(
            case![State::Start]
                .branch(case![Command::Help].endpoint(help))
                .branch(case![Command::Start].endpoint(start)),
        )
        .branch(case![Command::Cancel].endpoint(cancel));

    let message_handler = Update::filter_message()
        .branch(command_handler)
        .branch(case![State::ReceiveMovieName].endpoint(receive_movie_name))
        .branch(dptree::endpoint(invalid_state));

    let callback_query_handler = Update::filter_callback_query()
        .branch(case![State::ReceiveMovieChoice { movie_name }].endpoint(receive_movie_selection));

    dialogue::enter::<Update, InMemStorage<State>, State, _>()
        .branch(message_handler)
        .branch(callback_query_handler)
}

async fn start(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, "Qual o nome do filme?")
        .await?;
    dialogue.update(State::ReceiveMovieName).await?;
    Ok(())
}

async fn help(bot: Bot, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, Command::descriptions().to_string())
        .await?;
    Ok(())
}

async fn cancel(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, "Cancelling the dialogue.")
        .await?;
    dialogue.exit().await?;
    Ok(())
}

async fn invalid_state(bot: Bot, msg: Message) -> HandlerResult {
    bot.send_message(
        msg.chat.id,
        "unable to handle the message. type /help to see the usage.",
    )
    .await?;
    Ok(())
}

async fn receive_movie_name(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    match msg.text().map(ToOwned::to_owned) {
        Some(full_name) => {
            let response = piratebay::search(&full_name).await;
            let response = response.unwrap();

            let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];

            let max = if response.len() > 10 {
                10
            } else {
                response.len()
            };
            for versions in response[0..max].chunks(1) {
                let row = versions
                    .iter()
                    .map(|version| {
                        InlineKeyboardButton::callback(
                            version.name.to_owned(),
                            version.magnet_link.to_owned(),
                        )
                    })
                    .collect();

                keyboard.push(row);
            }

            bot.send_message(msg.chat.id, "Selecione uma das opções abaixo:")
                .reply_markup(InlineKeyboardMarkup::new(keyboard))
                .await?;

            dialogue
                .update(State::ReceiveMovieChoice {
                    movie_name: full_name,
                })
                .await?;
        }
        None => {
            bot.send_message(
                msg.chat.id,
                "Por favor, digite o nome do filme que deseja assistir.",
            )
            .await?;
        }
    }
    Ok(())
}

async fn receive_movie_selection(
    bot: Bot,
    dialogue: MyDialogue,
    movie_name: String,
    q: CallbackQuery,
) -> HandlerResult {
    if let Some(magnet_link) = &q.data {
        let mut headers = HeaderMap::new();
        headers.insert(
            "Content-Type",
            "application/x-www-form-urlencoded".parse().unwrap(),
        );
        let client = reqwest::Client::new();
        client
            .post("http://127.0.0.1:3030/torrents")
            .headers(headers)
            .body(format!("{}", magnet_link))
            .send()
            .await?;
        bot.send_message(
            dialogue.chat_id(),
            format!("{movie_name}, esta sendo baixado, aguarde para finalizar!"),
        )
        .await?;
        dialogue.exit().await?;
    }

    Ok(())
}
