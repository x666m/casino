use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Casino!");
    let winner = 21;
    let pnumber_one = rand::thread_rng().gen_range(1..=11);
    let pnumber_two = rand::thread_rng().gen_range(1..=11);

    let dnumber_one = rand::thread_rng().gen_range(1..=11);
    let dnumber_two = rand::thread_rng().gen_range(1..=11);

    println!("{} and {}", pnumber_one, pnumber_two);
    println!("Hit or Stay?");

    loop {
        let mut ante = String::new();

        io::stdin().read_line(&mut ante).expect("Try again.");

        let ante: String = match ante.trim().parse() {
            Ok(str) => str,
            Err(_) => continue,
        };

        if ante == "Stay" {
            let total = pnumber_one + pnumber_two;
            match total.cmp(&winner) {
                Ordering::Less => {
                    println!("Total: {}", total);
                    println!("Let's see what the dealer has!");
                    println!("Dealer's Hand: {} and {}", dnumber_one, dnumber_two);
                    if dnumber_one + dnumber_two == 21 {
                        println!("Dang, not your day.");
                        break;
                    } else if dnumber_one + dnumber_two == total {
                        println!("Push! Nice.");
                        break;
                    } else if dnumber_one + dnumber_two < total {
                        println!("You win! Nice");
                        break;
                    }
                }
                Ordering::Greater => println!("This is not possible, cunt."),
                Ordering::Equal => {
                    println!("Blackjack! {} and {}", pnumber_one, pnumber_two);
                    println!("Let's see what the dealer has...");
                    println!("Dealer's Hand: {} and {}", dnumber_one, dnumber_two);
                    if dnumber_one + dnumber_two != 21 {
                        println!("Congratulations! You win! (for now)");
                        break;
                    }
                }
            }
        }

        if ante == "Hit" {
            let pnumber_three = rand::thread_rng().gen_range(1..=14);
            let total = pnumber_one + pnumber_two + pnumber_three;

            match total.cmp(&winner) {
                Ordering::Less => {
                    //ENTERING META HERE
                    println!("{}, {}, and {}", pnumber_one, pnumber_two, pnumber_three);
                    println!("Hit or Stay?");
                    let mut ante = String::new();
                    io::stdin().read_line(&mut ante).expect("Try again.");
                    let ante: String = match ante.trim().parse() {
                        Ok(str) => str,
                        Err(_) => continue,
                    };
                    if ante == "Hit" {
                        let pnumber_four = rand::thread_rng().gen_range(1..=14);
                        let total = pnumber_one + pnumber_two + pnumber_three + pnumber_four;
                        match total.cmp(&winner) {
                            Ordering::Less => {
                                println!(
                                    "{}, {}, {}, and {}",
                                    pnumber_one, pnumber_two, pnumber_three, pnumber_four
                                );
                                println!("Hit or Stay?");
                                let mut ante = String::new();
                                io::stdin().read_line(&mut ante).expect("Try again.");
                                let ante: String = match ante.trim().parse() {
                                    Ok(str) => str,
                                    Err(_) => continue,
                                };
                            }
                            Ordering::Greater => {
                                println!(
                                    "{}, {}, {}, and {}. Bust",
                                    pnumber_one, pnumber_two, pnumber_three, pnumber_four
                                );
                                break;
                            }
                            Ordering::Equal => {
                                println!(
                                    "Winner! {}, {}, {}, and {}",
                                    pnumber_one, pnumber_two, pnumber_three, pnumber_four
                                );
                                println!("Let's see what the dealer has!");
                                println!("Dealer's Hand: {} and {}", dnumber_one, dnumber_two);
                                if dnumber_one + dnumber_two != 21 {
                                    println!("Congratulations! You win! (for now)");
                                    break;
                                } else {
                                    println!("Looks like it's a draw! What are the odds?");
                                    println!("Restart for a new hand.");
                                    break;
                                }
                            }
                        }
                    }

                    if ante == "Stay" {
                        let total = pnumber_one + pnumber_two + pnumber_three;
                        match total.cmp(&winner) {
                            Ordering::Less => {
                                println!("Total: {}", total);
                                println!("Let's see what the dealer has!");
                                println!("Dealer's Hand: {} and {}", dnumber_one, dnumber_two);
                                if dnumber_one + dnumber_two == 21 {
                                    println!("Dang, not your day.");
                                    break;
                                } else if dnumber_one + dnumber_two == total {
                                    println!("Push! Nice.");
                                    break;
                                } else if dnumber_one + dnumber_two < total {
                                    println!("You win! Nice");
                                    break;
                                }
                            }
                            Ordering::Greater => {
                                println!("You Lose");
                                break;
                            }
                            Ordering::Equal => {
                                println!(
                                    "Woo! {}, {}, and {}",
                                    pnumber_one, pnumber_two, pnumber_three
                                );
                                println!("Let's see what the dealer has!");
                                println!("Dealer's Hand: {} and {}", dnumber_one, dnumber_two);
                                if dnumber_one + dnumber_two != 21 {
                                    println!("Congratulations! You win! (for now)");
                                    break;
                                } else {
                                    println!("Push. Yikes.");
                                    break;
                                }
                            }
                        }
                    }
                }

                Ordering::Greater => {
                    println!(
                        "{}, {}, and {}. Bust.",
                        pnumber_one, pnumber_two, pnumber_three
                    );
                    break;
                }
                Ordering::Equal => {
                    println!(
                        "Blackjack! {}, {}, and {}",
                        pnumber_one, pnumber_two, pnumber_three
                    );
                    println!("Let's see what the dealer has!");
                    println!("Dealer's Hand: {} and {}", dnumber_one, dnumber_two);
                    if dnumber_one + dnumber_two != 21 {
                        println!("Congratulations! You win! (for now)");
                        break;
                    } else {
                        println!("Looks like it's a draw! What are the odds?");
                        println!("Restart for a new hand.");
                        break;
                    }
                }
            }
            if ante == "Stay" {
                let total = pnumber_one + pnumber_two;
                match total.cmp(&winner) {
                    Ordering::Less => {
                        println!("Total: {}", total);
                        println!("Let's see what the dealer has!");
                        println!("Dealer's Hand: {} and {}", dnumber_one, dnumber_two);
                        if dnumber_one + dnumber_two == 21 {
                            println!("Dang, not your day.");
                            break;
                        } else if dnumber_one + dnumber_two == total {
                            println!("Push! Nice.");
                            break;
                        } else if dnumber_one + dnumber_two < total {
                            println!("You win! Nice");
                            break;
                        }
                    }
                    Ordering::Greater => println!("This is not possible, cunt."),
                    Ordering::Equal => {
                        println!("Blackjack! {} and {}", pnumber_one, pnumber_two);
                        println!("Let's see what the dealer has!");
                        println!("Dealer's Hand: {} and {}", dnumber_one, dnumber_two);
                        if dnumber_one + dnumber_two != 21 {
                            println!("Congratulations! You win! (for now)");
                            break;
                        } else {
                            println!("Push. Bummer.");
                            break;
                        }
                    }
                }
            }
        }

        if ante == "Quit" {
            break;
        }
    }
}
