# Advent of Code Solutions in Rust


Welcome to my repository where I'm tackling the [Advent of Code](https://adventofcode.com/) challenges using the Rust programming language. This is both a coding challenge and a learning journey for me as I delve deeper into Rust.

## About Advent of Code

Advent of Code is an annual set of Christmas-themed programming challenges that can be solved in any programming language. The challenges range from simple tasks to complex algorithms, making it a perfect platform to improve coding skills and learn new languages.

## My Journey with Rust

As I'm new to Rust, this project is also a part of my learning process. Rust is known for its safety, speed, and concurrency capabilities. Through these challenges, I aim to get a better grasp of Rust's unique features like ownership, lifetimes, and the borrow checker.

## Repository Structure

Each day's challenge is placed in its own directory:

```
/adventofcode-2023
    /data
        /day1
            input.txt
        /day2
            input.txt
        ...
    /src
        /day1
            part1.rs
            part2.rs
            mod.rs
            README.rs
        /day2
        ...
        /utils
        main.rs
    .gitignore
    Cargo.toml
    LICENSE
    README.md

```

Inside each directory, you'll find:

- partN.rs: The Rust file containing my solution for the specific part.
- README.md: A brief overview of the challenge and my approach to solving it.

Inside each `/data` directory, you'll find:

- input.txt: The input data for the challenge.

## How to Run

To run any of the solutions:

1. Ensure you have Rust installed. You can download it from [here](https://www.rust-lang.org/tools/install).
2. Clone this repository.
3. Run `cargo run DAY PART` in the terminal. DAY is the day that we want to solve [1-25], PART is the part we want to solve of that day [1-2].

## Progress

Here's my progress so far:

- [x] Day 1: Trebuchet?!
- [x] Day 2: Cube Conundrum
- [x] Day 3: Gear Ratios
- [x] Day 4: Scratchcards
- [x] Day 5: If You Give A Seed A Fertilizer
- [x] Day 6: Wait For It
- [x] Day 7: Camel Cards
- [x] Day 8: Haunted Wasteland
- [x] Day 9: Mirage Maintenance
- [x] Day 10: Pipe Maze
- [x] Day 11: Cosmic Expansion
- [x] Day 12: Hot Springs
- [x] Day 13: Point of Incidence
- [x] Day 14: Parabolic Reflector Dish
- [x] Day 15: Lens Library
- [x] Day 16: The Floor Will Be Lava
- [x] Day 17: Clumsy Crucible
- [x] Day 18: Lavaduct Lagoon
- [x] Day19: Aplenty
- [x] Day 20: Pulse Propagation
- [x] Day 21: Step Counter
- [x] Day 22: Sand Slabs
- [x] Day 23: A Long Walk
- [x] Day 24: Never Tell Me The Odds
- [x] Day 25: Snowverload


## Learning Resources

I've been using the following resources to learn Rust:

- [The Rust Programming Language Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/stable/rust-by-example/)
- [Rustlings Course](https://github.com/rust-lang/rustlings/)


## Contributing


While this is primarily a personal learning project, I'm open to suggestions and feedback. Feel free to open an issue or a pull request if you have ideas on how I can improve my solutions.

## License

This project is open-sourced under the [GNU GENERAL PUBLIC LICENSE](https://github.com/ristemingov/adventofcode-2023/blob/main/LICENSE).