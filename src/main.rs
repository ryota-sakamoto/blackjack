extern crate rand;

use std::io::{
    stdin,
    stdout,
    Write,
};
use Card::*;
use Command::*;
use rand::{
    Rng,
    thread_rng,
};

#[derive(Debug)]
enum Card {
    Spade(u8),
    Clover(u8),
    Heart(u8),
    Diamond(u8),
    Down,
}

#[derive(Debug)]
struct User {
    name: String,
    money: u64,
    cards: Vec<Card>
}

impl User {
    fn new(name: &str) -> User {
        User {
            name: name.to_string(),
            money: 10_000,
            cards: Vec::new(),
        }
    }

    fn sum(&self) -> u8 {
        self.cards.iter().map(|v| {
            let n = match v {
                &Spade(n) => n,
                &Clover(n) => n,
                &Heart(n) => n,
                &Diamond(n) => n,
                _ => 0,
            };
            if n > 10 {
                10
            } else {
                n
            }
        }).sum()
    }

    fn show(&self) {
        println!("# {} is {:?}({})", self.name, self.cards, self.sum());
    }
}

enum Command {
    Hit,
    Stand,
    Surrender,
    Invalid,
}

fn main() {
    let mut cards = create_cards();  
    let mut dealer = User::new("Dealer");
    println!("- BlackJack Start -");

    let mut user = create_user();
    print!("# How much will you bed?
> ");
    stdout().flush();
    let mut bed = String::new();
    stdin().read_line(&mut bed);
    let bed: u64 = bed.trim().parse().unwrap();

    if user.money < bed {
        panic!("Too large bed");
    }
    user.money -= bed;

    for _ in 0..2 {
        if let Some(card) = cards.pop() {
            user.cards.push(card);
        }

        if let Some(card) = cards.pop() {
            dealer.cards.push(card);
        }
    }

    println!("{} is {:?}", dealer.name, [&dealer.cards[0], &Down]);
    println!("{} is {:?}", user.name, user.cards);
    
    let mut is_game = true;
    loop {
        match get_command() {
            Hit => {
                println!("# Draw card");
                let card = cards.pop().unwrap();
                user.cards.push(card);

                println!("# {} is {:?}", user.name, user.cards);
                if user.sum() > 21 {
                    println!("# You lose");
                    is_game = false;
                    break;
                }
            },
            Stand => {
                break;
            },
            Surrender => {
                is_game = false;
                user.money += bed / 2;
                break;
            },
            Invalid => continue,
        }
    }

    while is_game && dealer.sum() < 17 {
        let card = cards.pop().unwrap();
        dealer.cards.push(card);

        if dealer.sum() > 21 {
            dealer.show();
            println!("# Dealer lose");
            is_game = false;
            user.money += bed * 2;
            break;
        }
    }

    if is_game {
        dealer.show();
        user.show();

        let user_sum = user.sum();
        let dealer_sum = dealer.sum();
        if user_sum == dealer_sum {
            println!("# Result: Draw");
        } else if user_sum < dealer_sum {
            println!("# Result: {} lose", user.name);
        } else {
            println!("# Result: {} win", user.name);
            user.money += bed * 2;
        }
    }
    
    println!("You have {}", user.money);
}

fn create_cards() -> Vec<Card> {
    let mut rng = thread_rng();
    let mut cards = Vec::new();
    for n in 1..14 {
        cards.push(Spade(n));
        cards.push(Clover(n));
        cards.push(Heart(n));
        cards.push(Diamond(n));
    }
    rng.shuffle(&mut cards);
    cards
}

fn create_user() -> User {
    print!("# Input your name
> ");
    stdout().flush();
    let mut name = String::new();
    stdin().read_line(&mut name);

    User::new(name.trim())
}

fn get_command() -> Command {
    print!("What are you doing?
1. Hit
2. Stand
3. Surrender
> ");
    stdout().flush();
    let mut num = String::new();
    stdin().read_line(&mut num);
    let num: u8 = num.trim().parse().unwrap();

    match num {
        1 => Hit,
        2 => Stand,
        3 => Surrender,
        _ => Invalid,
    }
}