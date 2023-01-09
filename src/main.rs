use std::io::stdin;

mod service;
use service::*;

fn main() {
    const BOXES_COUNT: usize = 5;
    println!(
        "The Hiding Cat Puzzle

A cat is hiding in one of {:} boxes. 
The boxes are numbered one to five and are all sitting in a row, 
lined up in order. Each night, the sneaky little cat hides in an adjacent box, 
exactly one box away from the box it's in during the day. 
Each morning, you can open exactly one box to see if the cat is in there. 
Can you say with certainty that you will find the cat?
    ",
        BOXES_COUNT
    );
    let strat = play(BOXES_COUNT);

    println!("BUT have you used the best strat?...",);
    println!("Want to know? (Y/N): ",);
    loop {
        let mut buffer = String::new();
        stdin().read_line(&mut buffer).unwrap();
        match buffer.trim() {
            "Y" => {
                check_strat(BOXES_COUNT, strat);
                return;
            }
            "N" => {
                println!("OK then!");
                return;
            }
            _ => println!("Incorrect input. Try again."),
        }
    }
}

fn play(boxes_count: usize) -> Vec<usize> {
    let mut boxes = Boxes::create_boxes(boxes_count);
    boxes.place_cat_in_random_box();

    let mut current_strat = vec![];
    let mut won = false;

    println!("Day 1: Choose box (1, 2, ...): ");
    let mut day = 1;
    while !won {
        let mut buffer = String::new();
        stdin().read_line(&mut buffer).unwrap();
        if let Ok(number) = buffer.trim().parse::<usize>() {
            if number <= boxes_count && number > 0 {
                current_strat.push(number - 1);
                won = boxes.open_box(number - 1);
                if !won {
                    println!("You guessed wrong. Try again\n");
                    day += 1;
                    println!("Day {:}: Choose box: ", day);
                }
                continue;
            }
        }
        println!("Incorrect input. Try again")
    }
    println!("Correct!");

    return current_strat;
}

fn check_strat(boxes_count: usize, current_strat: Vec<usize>) {
    println!("Cool! So, let me check...",);
    let best_strats = &find_best_strats(boxes_count, 100000, 20);

    'strats_checker: for best_strat in best_strats {
        for (i, box_index) in current_strat.iter().enumerate() {
            if best_strat.0[i] != *box_index {
                continue 'strats_checker;
            }
        }

        println!("Yes! You're natural =)");
        return;
    }
    println!("Well.. No.");
    println!(
        "The best strat is, for example: {:}. Cat for sure in {:} tries. Good to know!",
        best_strats[0]
            .0
            .iter()
            .map(|box_index| (box_index).to_string())
            .collect::<Vec<String>>()
            .join(", "),
        best_strats[0].1
    );
}
