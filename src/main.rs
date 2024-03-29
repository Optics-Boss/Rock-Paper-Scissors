use rand::Rng;

fn main() {
    println!("Rock Paper Scissors");
    let player_choice = get_input_user();
    let opponent_choice = get_rock_papers_scissors();
    get_winner(player_choice, opponent_choice);
}

fn get_input_user() -> i32 {
    let mut input_user = String::new();

    println!("Pick your hand with using the number");
    println!("Rock = 1 Paper = 2 Scissors = 3");

    std::io::stdin()
        .read_line(&mut input_user)
        .expect("Unknown input");

    let input_user = input_user.trim().parse::<i32>().expect("Cant parse this number");

    let result = number_to_hand(input_user);
    println!("Your choice: {}", result);

    return input_user;
}

fn get_rock_papers_scissors() -> i32 {
    let mut random_generator = rand::thread_rng();
    let rps_number = random_generator.gen_range(1..4);

    let result = number_to_hand(rps_number);

    println!("Opponent choice: {}", result);

    return rps_number;
}

fn get_winner(player_choice: i32, opponent_choice: i32) {
    if player_choice == opponent_choice {
        return println!("It's a tie");
    } 

    let player_choice = number_to_hand(player_choice);
    let opponent_choice = number_to_hand(opponent_choice);

    if player_choice == "Rock" {
        match opponent_choice {
            "Scissor" => println!("You won"),
            "Paper" => println!("You lose"),
            _ => println!("Unknown"),
        };
    } else if player_choice == "Paper" {
        match opponent_choice {
            "Rock" => println!("You won"),
            "Scissor" => println!("You lose"),
            _ => println!("Unknown"),
        };
    } else if player_choice == "Scissor" {
        match opponent_choice {
            "Paper" => println!("You won"),
            "Rock" => println!("You lose"),
            _ => println!("Unknown"),
        };
    }
}

fn number_to_hand(number: i32) -> &'static str {
    match number {
        1 => "Rock",
        2 => "Paper",
        3 => "Scissor",
        _ => "Unknown"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_to_hand() {
        assert_eq!(number_to_hand(1), "Rock");
    }

}
