use std::io::{self};
use std::fs::read_to_string;
struct FlashCard<'a> {
    question: &'a str,
    answer: &'a str,
    order: i32,
    correctness: i32
}


fn main() {
    let mut cards: Vec<FlashCard> = vec![];
    let binding = read_to_string("cards.txt").unwrap();
    for (i, line) in binding.lines().enumerate() {
        let mut parts = line.split("? ").take(2);
        cards.push(FlashCard { question: parts.next().unwrap(), answer: parts.next().unwrap(), order: i as i32, correctness: 0 })
    }
    loop {
        let min_correctness = cards.iter().min_by_key(|x| x.correctness).unwrap().correctness;
        println!("{}/5", min_correctness);
        if min_correctness > 2 {
            println!("Congrats! You finished the game");
            break;
        }
        cards.sort_by(|a, b| a.order.cmp(&b.order));
        let card = &mut cards[0];
        println!("{}?", card.question);
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        if guess.trim().to_lowercase() == card.answer.to_lowercase() {
            println!("CORRECT!");
            card.order += 15;
            card.correctness += 1;
        } else {
            println!("Wrong! It's {}", card.answer);
            card.order += 4;
            card.correctness -= 1;
        }
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line"); // wait for an OK
        print!("\x1B[2J\x1B[1;1H");
    }
    
}
