use colored::*;
use rand::Rng;
use std::{
    cmp::Ordering,
    io,
    ops::{Div, Mul, Sub},
};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    println!("{:?}", args);
    const PROGRAMS: [&str; 2] = ["guess_game", "geometry"];
    let program = args.last().expect("Infussient arguments").as_str();
    match program {
        "guess_game" => guess_game(),
        "geometry" => geometry(),
        _ => {
            println!(
                "{}",
                "Welcome to @KD_MARK Rusty ground !\n Select and run programs with the following commands: "
                    .blue()
            );
            for pp in PROGRAMS {
                println!("{}", format!("> cargo run {}", pp).cyan());
            }
            if args.len() > 1 {
                println!("{}", format!("# {} program not found !", program).red());
                return;
            }
        }
    }
}

fn guess_game() {
    println!("Guess the number !");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small!".yellow()),
            Ordering::Greater => println!("{}", "Too big!".yellow()),
            Ordering::Equal => {
                println!("{}", "You win!".green());
                break;
            }
        }
    }
}

enum TriangleCategory {
    Equilateral,
    Isosceles(u32, u32), //(eq, uneq)
    Scalene,
}
struct Triangle {
    sides: (u32, u32, u32),
    category: TriangleCategory,
}

fn geometry() {
    println!(
        "{}",
        "Let's do a bit of geomery.
        \n Enter your triangle's sides length and know what type of triangle you got"
            .cyan()
    );
    let mut triangle = Triangle {
        category: TriangleCategory::Scalene,
        sides: (0, 0, 0),
    };
    read_input_sides(&mut triangle);
    set_cat(&mut triangle);
    let triangle_area = get_area(&triangle);
    println!(
        "{}",
        format!("Your triangle is {:?}", triangle.sides).blue()
    );
    println!(
        "{}",
        format!("Your triangle area is {}", triangle_area).blue()
    );

    fn get_area(triangle: &Triangle) -> f64 {
        let (a, b, c) = triangle.sides;
        match triangle.category {
            TriangleCategory::Equilateral => 3.0_f64.sqrt().div(4.0).mul((a.pow(2)) as f64),
            TriangleCategory::Isosceles(eq, uneq) => {
                0.25.mul(((4 * eq.pow(2)).sub(uneq.pow(2))) as f64)
            }
            TriangleCategory::Scalene => {
                let semi_perimeter = (get_perimeter(&triangle) as f64).div(2.0);
                println!(
                    "{}",
                    format!("Your triangle perimeter is {}", semi_perimeter*2.0).blue()
                );
                (semi_perimeter
                    .mul(semi_perimeter.sub(a as f64))
                    .mul(semi_perimeter.sub(b as f64))
                    .mul(semi_perimeter.sub(c as f64)))
                .sqrt()
            }
        }
    }

    fn get_perimeter(triangle: &Triangle) -> u32 {
        let (a, b, c) = triangle.sides;
        a + b + c
    }

    fn set_cat(triangle: &mut Triangle) {
        let (a, b, c) = triangle.sides;
        triangle.category = if a == b && b == c {
            TriangleCategory::Equilateral
        } else if a == b || b == c || a == c {
            let (eq, uneq) = if a == b {
                (a, c)
            } else if b == c {
                (b, a)
            } else {
                (c, b)
            };
            TriangleCategory::Isosceles(eq, uneq)
        } else {
            TriangleCategory::Scalene
        }
    }

    fn read_input_sides(triangle: &mut Triangle) {
        loop {
            println!("Enter your triangle side");
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            let input: u32 = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
            let (a, b, c) = triangle.sides;
            triangle.sides = if a == 0 {
                (input, b, c)
            } else if b == 0 {
                (a, input, c)
            } else {
                (a, b, input)
            };
            if triangle.sides.0 != 0 && triangle.sides.1 != 0 && triangle.sides.2 != 0 {
                break;
            }
        }
    }
}
