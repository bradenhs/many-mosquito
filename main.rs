// ⚠️ Only modify this file if you know what you're doing!

mod bot;

use std::{
    io::{Read, Write},
    os::unix::net::UnixStream,
};

fn main() {
    let stream = UnixStream::connect("/tmp/zilch.sock").unwrap();

    let mut message = get_next_message(&stream);

    if message.channel != "start" {
        panic!("start should be first message");
    }

    let parts: Vec<String> = message.data.split(',').map(|s| s.to_string()).collect();

    let bot = bot::Bot::new(bot::Config {
        player: parts[0].to_string(),
        color: parts[1].to_string(),
        game_time_limit: parts[2].parse::<usize>().unwrap(),
        turn_time_limit: parts[3].parse::<usize>().unwrap(),
    });

    write_response(&stream, &message, String::from(""));

    loop {
        message = get_next_message(&stream);

        if message.channel != "move" {
            panic!("expected move message");
        }

        let board: Vec<Vec<String>> = message
            .data
            .split('|')
            .map(|s| s.split(',').map(|s| s.to_string()).collect())
            .collect();

        let (x, y) = bot.next_move(board);
        write_response(&stream, &message, [x.to_string(), y.to_string()].join(","));
    }
}

fn get_next_message(mut stream: &UnixStream) -> Message {
    let mut buf = [0; 2048];
    let count = stream.read(&mut buf).unwrap();
    let message = String::from_utf8(buf[..count].to_vec()).unwrap();
    let parts: Vec<String> = message.splitn(3, ',').map(|s| s.to_string()).collect();
    let channel = parts[0].clone();
    let id = parts[1].clone();
    let data = parts[2].clone();
    Message { channel, id, data }
}

fn write_response(mut stream: &UnixStream, message: &Message, response: String) {
    stream
        .write_all(
            [message.channel.to_owned(), message.id.to_owned(), response]
                .join(",")
                .as_bytes(),
        )
        .unwrap();
}

struct Message {
    channel: String,
    id: String,
    data: String,
}
