use std::io;
fn main() {
    println!("Hello, world!");
    loop {
        let mut fib_num = String::new();

        println!("What number of the fibonacci sequence do you want to find?");

        io::stdin()
                .read_line(&mut fib_num)
                .expect("Failed to read line");

        let fib_num: u32 = match fib_num.trim().parse() {
            Ok(num) => nth_fib_digit(num),
            Err(_) => continue,
        };

        println!("{fib_num}");
        break;
    }
    

    loop {
        let mut temp = String::new();

        println!("What temperature would you like to convert?");

        io::stdin()
                .read_line(&mut temp)
                .expect("Failed to read line");
        
        let temp: f64 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };


        let mut system = String::new();

        println!("What system is that in?");

        io::stdin()
                .read_line(&mut system)
                .expect("Failed to read line");

        let system:bool = if system.trim() == "C" {
            false
        } else if system.trim() == "F"{
            true
        } else {
            continue;
        };

        let converted: f64 = convert_temp(temp, system);

        println!("{converted}");
        break;
    }

    

    
    
}

fn nth_fib_digit(n:u16) -> u32 {
    if n == 0 {
        return 0;
    }

    let mut prev_nums = [0,1];
    let mut current_num = prev_nums[0] + prev_nums[1];
    let mut n = n;
    while n > 1 {
        current_num = prev_nums[0] + prev_nums[1];
        prev_nums[0] = prev_nums[1];
        prev_nums[1] = current_num;
        n = n - 1;
    }

    return current_num;
}

fn convert_temp(temp:f64, is_faren:bool) -> f64 {
    if is_faren {
        return (temp - 32.0) * (5.0/9.0);
    } else {
        return (temp * 1.8) + 32.0;
    }
}
