use console::Term;
use rand::distributions::{Alphanumeric, DistString};
use std::{thread, time::{Duration, SystemTime}};

struct Game {
    score: i32,
    start_time: SystemTime,
    term: Term
}

impl Game {
    fn new() -> Self {
        Self {
            score: 0,
            start_time: SystemTime::now(),
            term: Term::stdout()
        }
    }

    fn refresh_score_bar(&self, intro: bool) -> () {
        if !intro {
            self.term.clear_last_lines(1).unwrap();
        }
        println!("------------------ Score: {} ------------------", self.score);
    }

    fn intro_screen(&self) {
        println!("Welcome to my typing game");
        println!("When you are ready in front of your keyboard, press enter!");
        loop {
            if let Some(key) = self.read_character(false) {
                if key == '\n' {
                    self.term.clear_last_lines(1).unwrap();
                    self.term.clear_last_lines(1).unwrap();
                    return;
                }
            }
        }   
    }

    fn countdown(mut self, time_in_seconds: usize) -> Self {
        for i in (1..=time_in_seconds).rev() {
            println!("{}", i);
            thread::sleep(Duration::from_secs(1));
            self.term.clear_last_lines(1).unwrap();
        }
        self.start_time = SystemTime::now();
        return self
    }

    fn read_character(&self, remove_line: bool) -> Option<char> {
        match self.term.read_char() {
            Ok(c) => {
                if remove_line{
                    self.term.clear_last_lines(1).unwrap();
                }
                Some(c)
            }
            Err(_) => None,
        }
    }

    fn generate_random_character() -> char {
        Alphanumeric
            .sample_string(&mut rand::thread_rng(), 1)
            .chars()
            .next()
            .unwrap_or('`')
    }

    fn play_round(&mut self) {
        let random_char = Self::generate_random_character();
        println!("Press {}", random_char);

        if let Some(answer) = self.read_character(true) {
            if random_char.to_ascii_lowercase() == answer.to_ascii_lowercase() {
                self.score += 1;
                self.refresh_score_bar(false);
            }
        }
    }

    fn calculate_bonus(&self) -> i32 {
        let time_elapsed = self.start_time.elapsed().unwrap_or(Duration::from_secs(10));
        let time_bonus = (10000.0 - time_elapsed.as_millis() as f32).max(0.0) / 1000.0;
        time_bonus as i32
    }

    fn display_final_score(&self, bonus: i32) {
        let total_score = self.score + bonus;

        match total_score {
            1 => println!("You have scored {} point!", total_score),
            _ => println!("You have scored {} points!", total_score),
        }
    }
}

const GAME_DURATION: usize = 10; 

fn main() {
    let mut game = Game::new();

    game.intro_screen();
    game = game.countdown(3);

    game.refresh_score_bar(true);

    for _ in 0..GAME_DURATION {
        game.play_round();
    }

    let bonus = game.calculate_bonus();

    match bonus {
        1 => println!("Bonus: {} point", bonus),
        _ => println!("Bonus: {} points", bonus),
    }

    game.display_final_score(bonus);
}
