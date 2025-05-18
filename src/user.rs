use crate::{block::User, consensus::poa::PoA};

pub struct UsersList {
    pub users:Vec<User>,
}

impl UsersList {
    pub fn new() -> UsersList {
        UsersList { users: Vec::new() }
    }
    pub fn add_user(&mut self,uname:&str,pwd:&str,initial_balance:u64,validator_status:bool,poa:&mut PoA) {
        let new_user = User::new(uname,pwd,initial_balance,validator_status);
        if new_user.is_validator {
            poa.validator_list.push(new_user.public_key.clone())
        }
        self.users.push(new_user)
    }
    pub fn show_all_user(&self) {
        for user in self.users.iter() {
            println!("username: {},public key: {},balance: {}, is validator: {}", user.username,user.public_key,user.balance,user.is_validator)
        }
    }
    
    
}