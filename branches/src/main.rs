fn main() {
    {
        // let number = 3;
        let number = 7;

        // if number{
        //     println!("number was {number}");
        // }

        if number != 0 {
            println!("number was {number} other than zero");
        }

        if number < 5 {
            println!("condition was true");
        } else {
            println!("condition was false");
        }
    }

    {
        let number = 6;
        if number % 4 == 0 {
            println!("number is divisible by 4");
        } else if number % 3 == 0 {
            println!("number is divisible by 3");
        } else if number % 2 == 0 {
            println!("number is divisible by 2");
        } else {
            println!("number is not divisible by 4,3,or 2");
        }
    }

    {
        let condition = true;
        let number = if condition { 5 } else { 6 };
        // let number = if condition { 5 } else { "six" };
        println!("The value of number is:{number}");
    }

    {
        // loop {
        //     println!("angin!");
        // }

        let mut counter = 0;
        let result = loop {
            counter += 1;
            if counter == 10 {
                break counter * 2;
            }
            println!("The counter is {counter}");
        };
        println!("The result is {result}");
    }

    {
        let mut count = 0;
        'counting_up: loop {
            println!("count = {count}");
            let mut remaining = 10;
            loop {
                println!("remaining = {remaining}");
                if remaining == 9 {
                    break;
                }
                if count == 2 {
                    break 'counting_up;
                }
                remaining -= 1;
            }
            count += 1;
        }
        println!("End count is {count}");
    }

    {
        let a = [10, 20, 30, 40, 50];
        let mut index = 0;
        while index < 5 {
            println!("the value is {}", a[index]);
            index += 1;
        }

        for element in a {
            println!("the value is {}", element);
        }
    }

    {
        for number in (1..4).rev() {
            println!("{number}");
        }
        println!("LIFTOFF!");
    }
}
