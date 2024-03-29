use super::*;

#[derive(Debug)]
enum TriangleCategory {
    Equilateral,
    Isosceles(u32, u32), //(eq, uneq)
    Scalene,
}

impl TriangleCategory {
    fn from(triangle: &Triangle) -> Self {
        let Triangle(a, b, c) = *triangle;
        if a == b && b == c {
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
}

#[derive(Debug)]
struct Triangle(u32, u32, u32);

impl Triangle {
    fn input(&mut self, input: u32) {
        let Triangle(a, b, c) = *self;
        *self = if a == 0 {
            Triangle(input, b, c)
        } else if b == 0 {
            Triangle(a, input, c)
        } else {
            Triangle(a, b, input)
        }
    }

    fn perimeter(&self) -> u32 {
        let Triangle(a, b, c) = *self;
        a + b + c
    }

    fn area(&self) -> f64 {
        let Triangle(a, b, c) = *self;
        let category = TriangleCategory::from(self);
        match category {
            TriangleCategory::Equilateral => 3.0_f64.sqrt().div(4.0).mul((a.pow(2)) as f64),
            TriangleCategory::Isosceles(eq, uneq) => {
                let ar = (4 * (eq as i32).pow(2)).sub((uneq as i32).pow(2)) as f64;
                if ar <= 0.0 {
                    println!("{}", format!("It's not possible to form a valid triangle with the given side lengths {:?}", self).red());
                    return 0.0;
                }
                0.25.mul(ar.sqrt())
            }
            TriangleCategory::Scalene => {
                let sp = (self.perimeter() as f64).div(2.0);
                println!(
                    "{}",
                    format!("Your triangle perimeter is {}", sp * 2.0).blue()
                );
                if sp <= a as f64 || sp <= b as f64 || sp <= c as f64 {
                    println!("{}", format!("it's not possible to form a valid triangle with the given side lengths {:?}", self).red());
                    return 0.0;
                }
                (sp.mul(sp.sub(a as f64))
                    .mul(sp.sub(b as f64))
                    .mul(sp.sub(c as f64)))
                .sqrt()
            }
        }
    }
}

pub fn geometry() {
    println!(
        "{}",
        "Let's do a bit of geometry.
        \n Enter your triangle's sides length and know what type of triangle you got"
            .cyan()
    );

    let mut triangle = Triangle(0, 0, 0);
    read_input_sides(&mut triangle);
    let triangle_area = triangle.area();
    println!(
        "{}",
        format!(
            "Your geometry figure is {:?}, Area: {}, Type: {:?}",
            triangle,
            triangle_area,
            TriangleCategory::from(&triangle)
        )
        .blue()
    );

    fn read_input_sides(triangle: &mut Triangle) {
        loop {
            println!(
                "Enter your triangle {} side",
                if triangle.0 == 0 {
                    "first"
                } else if triangle.1 == 0 {
                    "second"
                } else {
                    "third"
                }
            );
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            let input: u32 = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
            triangle.input(input);
            if triangle.0 != 0 && triangle.1 != 0 && triangle.2 != 0 {
                break;
            }
        }
    }
}
