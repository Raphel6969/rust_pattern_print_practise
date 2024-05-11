fn main() {
    _pattern34(5);
}

fn _pattern34(rows: i32) {
    let mut charas = 'E' as u8;
    for i in (1..=rows).rev() {
        for j in 0..i {
            print!("{} ", (charas - j as u8) as char);
        }
        charas -= 1;
        println!();
    }
}

fn _pattern33(rows: i32) {
    let mut charas = 'a' as u8;
    let mut num = 1;
    for i in 1..=rows {
        for _j in 0..i {
            if num % 2 == 0 {
                let cap = (charas as u8) as char;
                print!("{} ", cap.to_uppercase());
                num += 1;
                charas += 1;
            } else {
                let cap = (charas as u8) as char;
                print!("{} ", cap.to_lowercase());
                num += 1;
                charas += 1;
            }
        }
        println!();
    }
}

fn _pattern32(rows: i32) {
    for i in 1..=rows {
        let charas = 'E' as u8;
        for j in (0..i).rev() {
            print!("{}", (charas - j as u8) as char)
        }
        println!();
    }
}

fn _pattern31(rows: i32) {
    let og_rows = rows;
    fn min_of_four(a: i32, b: i32, c: i32, d: i32) -> i32 {
        let min_ab = if a < b { a } else { b };
        let min_cd = if c < d { c } else { d };
        if min_ab < min_cd {
            min_ab
        } else {
            min_cd
        }
    }

    let rows = rows * 2 - 1;
    for i in 1..rows {
        for j in 1..rows {
            let min_index = og_rows - min_of_four(i, j, rows - i, rows - j);
            print!("{} ", min_index)
        }
        println!();
    }
}

fn _pattern30(rows: i32) {
    for i in 1..=rows {
        for _ in 1..=(rows - i) * 2 {
            print!(" ");
        }
        for j in (1..=i).rev() {
            print!("{} ", j)
        }
        for j in 2..=i {
            print!("{} ", j)
        }
        println!();
    }
}

fn _pattern29(rows: i32) {
    for i in 1..=rows {
        for j in 1..rows * 2 {
            if j <= i || j >= rows * 2 - i {
                print!("*")
            } else {
                print!(" ");
            }
        }
        println!();
    }
    for i in (1..=rows - 1).rev() {
        for j in 1..rows * 2 {
            if j <= i || j >= rows * 2 - i {
                print!("*")
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn _pattern28(rows: i32) {
    for i in 1..=rows {
        for _ in 1..=rows - i {
            print!(" ");
        }
        for _ in 1..=i {
            print!("* ")
        }
        println!();
    }
    for i in (1..=rows - 1).rev() {
        for _ in 1..=rows - i {
            print!(" ");
        }
        for _ in 1..=i {
            print!("* ")
        }
        println!();
    }
}

fn _pattern27(rows: i32) {
    // TODO : yeh karna baki hai
    // rethink on it
    println!("{}", rows);
}

fn _pattern26(rows: i32) {
    let mut num = 1;
    for i in (1..=rows).rev() {
        for _ in 1..=i {
            print!("{} ", num);
        }
        num += 1;
        println!()
    }
}

fn _pattern25(rows: i32) {
    for i in (1..=rows).rev() {
        for _ in 1..=i {
            print!(" ");
        }
        for j in 1..=rows {
            if j == 1 || j == rows || i == 1 || i == rows {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn _pattern24(rows: i32) {
    for i in 1..rows {
        for _j in 1..2 * rows {
            if _j == 1 || _j == i || _j == 2 * rows - 1 || _j == 2 * rows - i {
                print!("*");
            } else {
                print!(" ");
            }
        }
        if i != rows {
            println!();
        }
    }
    for i in (1..rows).rev() {
        for _j in 1..2 * rows {
            if _j == 1 || _j == i || _j == 2 * rows - 1 || _j == 2 * rows - i {
                print!("*");
            } else {
                print!(" ");
            }
        }
        if i != rows {
            println!();
        }
    }
}

fn _pattern23(_rows: i32) {
    // give it a time to think upon it
}

fn _pattern22(rows: i32) {
    let mut bool = true;
    let mut num = 1;
    for i in 1..=rows {
        for _ in 1..=i {
            print!("{} ", num);
            if bool == true {
                bool = false;
                num = 0;
            } else {
                bool = true;
                num = 1;
            }
        }
        println!();
    }
}

fn _pattern21(rows: i32) {
    let mut num: i32 = 1;
    for i in 1..=rows {
        for _ in 1..=i {
            print!("{} ", num);
            num += 1;
        }
        println!();
    }
}

fn _pattern20(rows: i32) {
    let width: i32 = rows - 1;
    for i in 1..=rows {
        for j in 1..=width {
            if i == 1 || i == rows || j == 1 || j == width {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn _pattern19(rows: i32) {
    for i in (0..rows).rev() {
        for _ in 0..rows - i {
            print!("*");
        }
        for _ in 0..i * 2 {
            print!(" ");
        }
        for _ in (0..rows - i).rev() {
            print!("*");
        }
        if i != rows {
            println!();
        }
    }
    for i in 0..rows {
        if i == 0 {
            continue;
        }
        for _ in 0..rows - i {
            print!("*");
        }
        for _ in 0..i * 2 {
            print!(" ");
        }
        for _ in (0..rows - i).rev() {
            print!("*");
        }
        if i != rows {
            println!();
        }
    }
}

fn _pattern18(rows: i32) {
    for i in 0..rows {
        for _ in 0..rows - i {
            print!("*");
        }
        for _ in 0..i * 2 {
            print!(" ");
        }
        for _ in (0..rows - i).rev() {
            print!("*");
        }
        if i != rows {
            println!();
        }
    }
    for i in (0..rows).rev() {
        for _ in 0..rows - i {
            print!("*");
        }
        for _ in 0..i * 2 {
            print!(" ");
        }
        for _ in (0..rows - i).rev() {
            print!("*");
        }
        if i != rows {
            println!();
        }
    }
}

fn _pattern17(rows: i32) {
    for i in 1..=rows {
        for _ in 1..=rows - i {
            print!(" ");
        }
        for k in (1..=i).rev() {
            print!("{}", k);
        }
        for k in 2..=i {
            print!("{}", k);
        }
        println!();
    }
}

fn _pattern16(rows: i32) {
    // Writing the factorial
    fn _factorial(num: i32) -> i32 {
        if num == 0 || num == 1 {
            1
        } else {
            let mut result: i32 = 1;
            for i in 1..=num {
                result *= i;
            }
            result
        }
    }

    // printing the pattern
    for i in 0..rows {
        for _ in 0..rows - i {
            print!("  ");
        }
        for j in 0..=i {
            let coefficient = _factorial(i) / (_factorial(j) * _factorial(i - j));
            print!("{:2}   ", coefficient);
        }
        println!();
    }
}

fn _pattern15(rows: i32) {
    for i in 1..=rows {
        for _ in 1..=rows - i {
            print!(" ");
        }
        for j in 1..=2 * i - 1 {
            if j == 1 || j == 2 * i - 1 {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!();
    }
    for i in (1..=rows).rev() {
        for _ in 1..=rows - i {
            print!(" ");
        }
        for j in 1..=2 * i - 1 {
            if j == 1 || j == 2 * i - 1 {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn _pattern14(rows: i32) {
    for i in (1..=rows).rev() {
        for _ in 1..=rows - i {
            print!(" ");
        }
        for j in 1..=2 * i - 1 {
            if j == 1 || j == 2 * i - 1 || i == rows {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn _pattern13(rows: i32) {
    for i in 1..=rows {
        // Print spaces
        for _ in 1..=rows - i {
            print!(" ");
        }
        // Print stars
        for j in 1..=2 * i - 1 {
            if j == 1 || j == 2 * i - 1 || i == rows {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn _pattern12(rows: i32) {
    for i in (1..=rows).rev() {
        for _ in 1..=rows - i {
            print!(" ");
        }
        for j in 1..=2 * i - 1 {
            if j % 2 == 0 {
                print!(" ");
            } else {
                print!("*");
            }
        }
        println!();
    }
    for i in 1..=rows {
        for _ in 1..=rows - i {
            print!(" ");
        }
        for j in 1..=2 * i - 1 {
            if j % 2 == 0 {
                print!(" ");
            } else {
                print!("*");
            }
        }
        println!();
    }
}

fn _pattern11(rows: i32) {
    for i in (1..=rows).rev() {
        for _ in 1..=rows - i {
            print!(" ");
        }
        for j in 1..=2 * i - 1 {
            if j % 2 == 0 {
                print!(" ");
            } else {
                print!("*");
            }
        }
        println!();
    }
}

fn _pattern10(rows: i32) {
    for i in 1..=rows {
        for _ in 1..=rows - i {
            print!(" ");
        }
        for j in 1..=2 * i - 1 {
            if j % 2 == 0 {
                print!(" ");
            } else {
                print!("*");
            }
        }
        println!();
    }
}

fn _pattern9(rows: i32) {
    for i in (1..=rows).rev() {
        for _ in 1..=rows - i {
            print!(" ");
        }
        for _ in 1..=2 * i - 1 {
            print!("*");
        }
        println!();
    }
}

fn _pattern8(rows: i32) {
    for i in 1..=rows {
        for _ in 1..=rows - i {
            print!(" ");
        }
        for _ in 1..=(2 * i - 1) {
            print!("*");
        }
        println!();
    }
}

fn _pattern7(rows: i32) {
    for i in (1..=rows).rev() {
        for _ in 1..=rows - i {
            print!(" ")
        }
        for _ in 1..=i {
            print!("*")
        }
        println!();
    }
}

fn _pattern6(rows: i32) {
    for i in 1..=rows {
        for _j in 1..=rows - i {
            print!(" ");
        }
        for _k in 1..=i {
            print!("*");
        }
        println!();
    }
}

fn _pattern5(rows: i32) {
    for i in 1..=rows {
        for _ in 1..=i {
            print!("*");
        }
        println!();
    }

    for i in (1..rows).rev() {
        for _ in 1..=i {
            print!("*");
        }
        println!();
    }
}

fn _pattern4(rows: i32) {
    for i in 1..=rows {
        for j in 1..=i {
            print!("{}", j);
        }
        println!();
    }
}

fn _pattern3(rows: i32) {
    for i in (1..=rows).rev() {
        for _ in 1..=i {
            print!("*");
        }
        println!();
    }
}

fn _pattern2(rows: i32) {
    for i in 1..=rows {
        for _ in 1..=i {
            print!("*");
        }
        println!();
    }
}

fn _pattern1(rows: i32) {
    for _i in 1..=rows {
        for _j in 1..=rows {
            print!("*");
        }
        println!();
    }
}
