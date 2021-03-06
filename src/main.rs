use get_the_streamer_game::GameState;
use ggez::conf::WindowMode;
use ggez::{event, ContextBuilder};
use std::sync::mpsc::channel;
use std::thread;
use twitch_chat_wrapper::chat_message::ChatMessage;

fn main() {
    let (send_to_game, receive_from_twitch) = channel::<ChatMessage>();
    let (send_to_twitch, receive_from_game) = channel::<String>();
    let twitchchat_thread = thread::spawn(move || {
        twitch_chat_wrapper::run(receive_from_game, send_to_game).unwrap();
    });
    let game_thread = thread::spawn(move || {
        let (context, event_loop) =
            &mut match ContextBuilder::new("Get the Streamer", "Brooks Patton")
                .window_mode(WindowMode::default().dimensions(1920.0, 1080.0))
                .build()
            {
                Ok((context, event_loop)) => (context, event_loop),
                Err(error) => panic!(error),
            };

        let game_state = &mut GameState::new(send_to_twitch, receive_from_twitch, context).unwrap();
        match event::run(context, event_loop, game_state) {
            Ok(_) => println!("Thanks for playing!"),
            Err(error) => println!("Error occurred: {}", error),
        };
    });
    twitchchat_thread.join().unwrap();
    game_thread.join().unwrap();
}
