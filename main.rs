fn main() {
    let mut commands = Vec::new();
    println!("Generating the 4-bit arithmatic machine.");
    for number1 in 1..15{
        for number2 in 1..15{ 
            commands.push(format!("{}+{}", number1, number2));
            commands.push(format!("{}", number1 + number2));
        }
    } 
    for number1 in 1..15{
        for number2 in 1..15{ 
            commands.push(format!("{}-{}", number1, number2));
            commands.push(format!("{}", number1 - number2));
        }
    }
    for number1 in 1..15{
        for number2 in 1..15{ 
            commands.push(format!("{}*{}", number1, number2));
            commands.push(format!("{}", number1 * number2));
        }
    }
    for number1 in 1..15{
        for number2 in 1..15{ 
            commands.push(format!("{}/{}", number1, number2));
            commands.push(format!("{}", number1 / number2));
        }
    }


    loop{
        let mut command_input = String::new();
        std::io::stdin().read_line(&mut command_input).expect("Could not read from stdio.");
        let mut print = false;
        for command in commands.clone(){
            if print{
                println!("{command}");
                break;
            }
            if command.trim() == command_input.trim(){
                print = true;
            }
        }
    }
}
