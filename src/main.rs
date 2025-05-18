use block::Transaction;
use blockchain::Blockchain;
use consensus::poa::PoA;
use consensus::pow::PoW;
use consensus::module::HybridConsensus;
use user::UsersList;
use std::io;
mod block;
mod utils;
mod user;
mod consensus;
mod blockchain;



fn main() {
    // start the block
    let mut chain=Blockchain::new();
    let mut user=UsersList::new();
    let mut poa=PoA::new();
    let pow=PoW {difficulty:7};
    // greeting and menu
    loop {
        println!("Hello, what will you do today?");
        let main_menus= ["1. add new user", "2. show all user", "3. add transaction", "4. exit"];
        println!("enter a number");
        for main_menu in &main_menus {
            println!("{}",main_menu)
        }

        let mut main_choose=String::new();
        io::stdin()
            .read_line(&mut main_choose)
            .expect("failed to get main choose");
        let main_choose=main_choose.trim();
        match main_choose {
            "1" => loop {
                println!("create username: ");
                let mut username=String::new();
                io::stdin()
                    .read_line(&mut username)
                    .expect("failed to get username");
                let username=username.trim();
                println!("create password: ");
                let mut password=String::new();
                io::stdin()
                    .read_line(&mut password)
                    .expect("failed to get passsword");
                let password=password.trim();
                println!("enter initial balance: ");
                let mut balance=String::new();
                io::stdin()
                    .read_line(&mut balance)
                    .expect("failed to get balance");
                let balance:u64= match balance.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        eprint!("input wasn't a number. using balance=0");
                        0}
                };
                println!("Are you validator? [yes/no]");
                let mut is_validator=String::new();
                io::stdin()
                    .read_line(&mut is_validator)
                    .expect("failed to get value");
                let is_validator:bool = match is_validator.trim().to_lowercase().as_str() {
                    "yes"=> true,
                    "no" => false,
                    _=> {
                        eprint!("Invalid input for validator status. please type yes or no.");
                        return;
                    }
                };
                user.add_user(username, password, balance, is_validator, &mut poa);
                println!("want to add more user? [y/n]");
                let mut add_user_confirmation=String::new();
                io::stdin()
                    .read_line(&mut add_user_confirmation)
                    .expect("failed to get confirmation");
                let add_user_confirmation= add_user_confirmation.trim();
                match add_user_confirmation {
                    "y" => continue,
                    "n" => break,
                    _ => eprintln!("invalid input. Please type y/n")
                }
            },
            "2" => user.show_all_user(),
            "3" => loop {
                let transaction_menus = ["1. send token","2. show all blocks","3. save block to file","4. exit"];
                for transaction_menu in &transaction_menus {
                    println!("{}",transaction_menu)
                };
                let mut choose_transaction_menu = String::new(); 
                io::stdin()
                    .read_line(&mut choose_transaction_menu)
                    .expect("failed to get option");
                let choose_transaction_menu = choose_transaction_menu.trim();
                match choose_transaction_menu {
                    "1" => { let mut all_transaction = Vec::new();
                        loop {
                            println!("type sender public key");
                            let mut sender_input = String::new();
                            io::stdin()
                                .read_line(&mut sender_input)
                                .expect("can't get the sender");
                            let sender_input = sender_input.trim();
                            println!("type receiver public key");
                            let mut receiver_input = String::new();
                            io::stdin()
                                .read_line(&mut receiver_input)
                                .expect("failed to get receiver");
                            let receiver_input =receiver_input.trim();
                            println!("type amount: ");
                            let mut amount = String::new();
                            io::stdin()
                                .read_line(&mut amount)
                                .expect("failed to get amount");
                            let amount:u64 =match amount.trim().parse() {
                                Ok(num) => num,
                                Err(_) => continue
                            };
                            let mut transaction = Transaction::new(sender_input,receiver_input,amount);
                            println!("procced transaction? [y/n]");
                            let mut proccess_transaction = String::new();
                            io::stdin()
                                .read_line(&mut proccess_transaction)
                                .expect("failed to get input");
                            let proccess_transaction = proccess_transaction.trim();
                            match proccess_transaction {
                                "y" => {
                                    let user_in_vec = user.users.iter()
                                        .find(|sender_user|&sender_user.public_key==sender_input)
                                        .expect("can't find the user");
                                    println!("type your password: ");
                                    let mut password = String::new();
                                    io::stdin()
                                        .read_line(&mut password)
                                        .expect("failed to get password");
                                    let password= password.trim();
                                    match user_in_vec.sign_transaction(&mut transaction, password) {
                                        Ok(()) => {
                                            let balance_status=transaction.verify_balance(user_in_vec);
                                            let sign_status=transaction.verify_sign();
                                            if balance_status && sign_status {
                                                all_transaction.push(transaction);
                                                println!("transaction added successfully");
                                            } else {
                                                println!("Failed to process transaction: balance or signature invalid.");
                                            }
                                        },
                                        Err(e) => {println!("Signing failed:{}",e)}

                                    }
                                },
                                "n" => {
                                    println!("transaction aborted");
                                    continue;
                                },
                                _ => println!("invalid input. Try again")
                            }
                            println!("want to add more transaction?[y/n]");
                            let mut add_more_transaction = String::new();
                            io::stdin()
                                .read_line(&mut add_more_transaction)
                                .expect("failed to get value");
                            let add_more_transaction= add_more_transaction.trim();
                            match add_more_transaction {
                                "y" => continue,
                                "n" => break,
                                _ => eprintln!("invalid input. type y/n")
                            }
                        }
                        let total_transaction:u64=all_transaction.iter().map(|tx|tx.amount).sum(); 
                        let mut consensus =if total_transaction > 1000 {
                            HybridConsensus::PoW(pow.clone())
                        } else {
                            HybridConsensus::PoA(poa.clone())
                        };
                        chain.add_block(all_transaction, &mut consensus, &mut user);
                    },
                    "2" => chain.get_all_blocks(),
                    "3" => {
                        println!("enter the file name: ");
                        let mut  name_file = String::new();
                        io::stdin()
                            .read_line(&mut name_file)
                            .expect("failed to get name file");
                        let name_file= name_file.trim();
                        match chain.save_to_file(name_file) {
                            Ok(()) => println!("file saved successfully"),
                            Err(e) => println!("failed to save: {}",e)
                        }
                    },
                    "4" => break,
                    _ => eprintln!("type a valid input")
                }
            },
            "4" => {
                println!("see you!");
                break
            },
            _ => {eprintln!("invalid input!")}
            }
        }

    }

