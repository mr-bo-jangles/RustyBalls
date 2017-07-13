extern crate irc;
extern crate regex;
extern crate rand;
extern crate serde;
extern crate serde_json;

use std::collections::HashMap;
use std::default::Default;
use std::time::{Duration, SystemTime};
use std::thread;
use irc::client::prelude::*;
use std::fs::File;
use std::sync::mpsc::channel;

//fn save_last_seen(last_seen_hash: HashMap<String, SystemTime>) {
//    let mut f = File::open("last_seen.json")?;
//    let serialised_json = serde_json::to_str(&last_seen_hash).unwrap();
//    f.write(serialised_json);
//}
//
//fn load_last_seen(last_seen_hash: HashMap<String, SystemTime>) {
//    let mut f = File::open("last_seen.json")?;
//    let serialised_json = f.write(serialised_json);
//    *last_seen_hash = serde_json::from_str(&serialised_json).unwrap();
//}

fn perform_timeout(timeouts: &mut HashMap<String, SystemTime>, timeout_key: &str, timeout: u64) -> bool {
    let time_now = SystemTime::now();
    let mut timeouts_entry = timeouts.entry(String::from(timeout_key)).or_insert(time_now - Duration::new(1000, 0));
    let duration_since = timeouts_entry.elapsed().unwrap();
    let seconds_waited = duration_since.as_secs();
    println!("{:?}", seconds_waited);
    if seconds_waited >= timeout {
        *timeouts_entry = time_now;
        return true;
    } else {
        println!("You need to wait {:?} more seconds to use this command again", timeout - seconds_waited);
        return false;
    }
}

fn fucks(target: &str, message_array: Vec<&str>, server: &IrcServer) {
    let message_array_length = message_array.len();
    if message_array_length >= 2 {
        server.send_privmsg(target, format!("Exactly {} fucks are given", message_array[1]).as_str()).unwrap();
    } else {
        server.send_privmsg(target, "You need to tell me how many fucks are given like, !fucks 12").unwrap();
    }
}

fn test(target: &str, server: &IrcServer) {
    println!("Target is: {}", target);
    server.send_privmsg(target, "Hi!").unwrap();
}

fn gym(target: &str, server: &IrcServer) {
    println!("Target is: {}", target);
    server.send_privmsg(target, "/w phanxbot !gym next").unwrap();
}

fn train(target: &str, server: &IrcServer) {
    println!("Target is: {}", target);
    server.send_privmsg(target, "/w phanxbot !train 1000").unwrap();
}

fn victory(target: &str, server: &IrcServer) {
    println!("Target is: {}", target);
    server.send_privmsg(target, "!victory").unwrap();
}

fn throw_lures(target: &str, server: &IrcServer, items: &mut HashMap<String, i32>) {
    println!("Target is: {}", target);
    server.send_privmsg(target, "!rare").unwrap();
    let thirty_seconds = Duration::from_millis(30000);
    thread::sleep(thirty_seconds);
    server.send_privmsg(target, "!lure").unwrap();
}

fn throw_ball(target: &str, server: &IrcServer, items: &mut HashMap<String, i32>) {
    println!("Target is: {}", target);
    let poke_balls = items.get("Pokeballs");
    match poke_balls {
        None => {

        },
        Some(v) => {
            if v > &0 {
                if rand::random() {
                    server.send_privmsg(target, "!throw").unwrap();
                } else {
                    server.send_privmsg(target, "!catch").unwrap();
                }
            }
        }
    }
}

fn last_seen(target: &str, message_array: Vec<&str>, server: &IrcServer, last_seen_hash: &mut HashMap<String, SystemTime>) {
    let message_array_length = message_array.len();
    if message_array_length >= 2 {
        let looking_for = message_array[1];
        let user_last_seen_time = last_seen_hash.get(looking_for);
        match user_last_seen_time {
            None => {
                server.send_privmsg(target, format!("I have never seen {}", looking_for).as_str()).unwrap();
            },
            Some(v) => {
                let time_elapsed = v.elapsed();
                match time_elapsed {
                    Ok(v) => {
                        server.send_privmsg(target, format!("I last saw {} about {:?} mins ago.", looking_for, v.as_secs()/60).as_str()).unwrap();
                    },
                    Err(e) => {
                        server.send_privmsg(target, format!("I have seen {} but some weird time shit's going on right now. {}", looking_for, e).as_str()).unwrap();
                    }
                }
            }
        }
    } else {
        server.send_privmsg(target, "You need to tell me who you're looking for dude, like, !lastsaw your_mum").unwrap();
    }
}

fn update_last_seen(last_seen_hash: &mut HashMap<String, SystemTime>, nickname: &str) {
    let time_now = SystemTime::now();
    *last_seen_hash.entry(String::from(nickname)).or_insert(time_now) = time_now;
    println!("Updating {} in the last seen DB at {:?}", nickname, time_now);
    println!("I'm now tracking {:?} people", last_seen_hash.len())
}

fn main() {

    let server = IrcServer::new("config.json").unwrap();
    server.identify().unwrap();

    let mut timeouts: HashMap<String, SystemTime> = HashMap::new();
    let mut items: HashMap<String, i32> = HashMap::new();
    let mut last_seen_hash: HashMap<String, SystemTime> = HashMap::new();

//    load_last_seen(last_seen_hash);

    while true {
        let message = server.poll();
        match message {
            None => ( break ),
            Some(T) => {

            },
            Ok(T) => {

            },
            Err(E) => {
                println!("{:?}",E)
            }
        }
    }

    server.for_each_incoming(|message| {
        // Do message processing.
        match message.command {
            Command::PRIVMSG(ref target, ref msg) => {
                let message_string = msg.to_string();
                let message_array = message_string.split(" ").collect::<Vec<_>>();
                println!("Timeouts are: {:?}", timeouts);
                println!("{:?}", message_array);
                match message_array[0] {
                    "!test" => {
                        println!("Test was called");
                        if perform_timeout(&mut timeouts, &"test", 10) {
                            test(target, &server)
                        }
                    }
                    "!fucks" => {
                        println!("Fucks was called");
                        if perform_timeout(&mut timeouts, &"fucks", 10) {
                            fucks(target, message_array, &server)
                        }
                    }
                    "!gym" => {
                        println!("Gym was called");
                        if perform_timeout(&mut timeouts, &"gym", 310) {
                            gym(target, &server)
                        }
                    }
                    "!train" => {
                        println!("Train was called");
                        if perform_timeout(&mut timeouts, &"train", 310) {
                            train(target, &server)
                        }
                    }
                    "!lastseen" => {
                        if perform_timeout(&mut timeouts, &"last_seen", 10) {
                            last_seen(target, message_array, &server, &mut last_seen_hash)
                        }
                    },
                    "!victory" => {
                        if perform_timeout(&mut timeouts, &"victory", 500) {
                            victory(target, &server)
                        }
                    }
                    "!quit" => {

                    }
                    _ => {
                        println!("{:?}", message_array);
                        if message_array.len() >= 4 {
                            if message_array[2] == "has" && message_array[3] == "escaped" {
                                throw_lures(target, &server, &mut items);
                            }
                        }
                        if message_array.len() >= 6 {
                            if message_array[2] == "is" && message_array[3] == "about" && message_array[4] == "to" && message_array[5] == "escape!" {
                                throw_ball(target, &server, &mut items);
                            }
                        }
                    }
                };
                update_last_seen(&mut last_seen_hash, message.source_nickname().unwrap());
            }
            _ => (),
        }
    }).unwrap();

//    save_last_seen(last_seen_hash);
}