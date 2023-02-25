use std::io;

use crossterm::{
    cursor::{Hide, MoveToColumn, Show},
    event::{read, Event, KeyCode},
    style::{Print, PrintStyledContent, Stylize},
    terminal::{disable_raw_mode, enable_raw_mode},
    ExecutableCommand, Result,
};

#[derive(Debug)]
enum Color {
    Yellow,
    Green,
    None,
}

#[derive(Debug)]
struct Block {
    char: String,
    color: Color,
}

fn main() -> Result<()> {
    let word = "CROOK";

    let mut stdout = io::stdout();
    stdout.execute(Hide)?;
    enable_raw_mode()?;

    for _ in 0..6 {
        let mut guess = String::new();
        stdout
            .execute(Print("  _  _  _  _  _ "))?
            .execute(MoveToColumn(0))?;

        loop {
            match read()? {
                Event::Key(event) => match event.code {
                    KeyCode::Char(c) => {
                        if guess.len() < 5 && c.is_alphabetic() {
                            guess = guess + &c.to_string().to_uppercase();
                        }
                    }
                    KeyCode::Backspace => {
                        if guess.len() > 0 {
                            guess = guess[..guess.len() - 1].to_string();
                        }
                    }
                    KeyCode::Enter => {
                        if guess.len() == 5 {
                            break;
                        }
                    }
                    _ => {}
                },
                _ => {}
            };

            stdout
                .execute(Print(
                    format!("{:_<5}", guess)
                        .split("")
                        .collect::<Vec<&str>>()
                        .join("  "),
                ))?
                .execute(MoveToColumn(0))?;
        }

        stdout.execute(MoveToColumn(0))?;

        if guess == word {
            let string = guess.split("").collect::<Vec<&str>>().join("  ");
            let len = string.len();
            stdout
                .execute(Print(" "))?
                .execute(PrintStyledContent(string[1..len - 1].black().on_green()))?
                .execute(Print("\n"))?
                .execute(MoveToColumn(0))?;
            break;
        }

        // First get the yellow hints
        let mut chars: Vec<char> = word.chars().collect();
        let mut blocks: Vec<Block> = vec![];

        for char in guess.chars() {
            let block;
            if chars.contains(&char) {
                if let Some(pos) = chars.iter().position(|x| *x == char) {
                    chars.remove(pos);
                    block = Block {
                        char: char.to_string(),
                        color: Color::Yellow,
                    };
                } else {
                    block = Block {
                        char: char.to_string(),
                        color: Color::None,
                    };
                }

                blocks.push(block);
            } else {
                blocks.push(Block {
                    char: char.to_string(),
                    color: Color::None,
                })
            }
        }

        // disable_raw_mode()?;
        // println!("{:?}", blocks);
        // break;

        // Then make the necessary ones green
        for i in 0..5 {
            if blocks[i].char == word.chars().nth(i).unwrap().to_string() {
                blocks[i].color = Color::Green;
            }
        }

        // Finally print the result
        stdout.execute(Print(" "))?;
        for block in blocks {
            match block.color {
                Color::Yellow => {
                    stdout.execute(PrintStyledContent(
                        format!(" {} ", block.char).black().on_yellow(),
                    ))?;
                }
                Color::Green => {
                    stdout.execute(PrintStyledContent(
                        format!(" {} ", block.char).black().on_green(),
                    ))?;
                }
                Color::None => {
                    stdout.execute(Print(format!(" {} ", block.char)))?;
                }
            };
        }

        stdout.execute(Print("\n"))?.execute(MoveToColumn(0))?;
    }

    stdout.execute(Show)?;
    disable_raw_mode()?;
    Ok(())
}
