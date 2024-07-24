struct Bank {
    users: Vec<User>,
    amount: u128,
    fee: u8,
    transactions: Vec<Transaction>,
}

impl Bank {
    // fn new(fee: u8)->Self
    fn new(fee: u8) -> Self {
        Self {
            users: Vec::new(),
            amount: 0,
            fee,
            transactions: Vec::new(),
        }
    }

    fn add_user(&mut self, new_user: User) {
        self.users.push(new_user);
    }

    fn add_transaction(&mut self, transaction: Transaction) {
        self.transactions.push(transaction);
    }


    fn process_transfer(&mut self, sender_id: &str, receiver_id: &str, amount: u32) {
        let sender_index = self.users.iter().position(|user| user.account_id == sender_id);
        let receiver_index = self.users.iter().position(|user| user.account_id == receiver_id);
       // println!("{:?}{:?}", sender_index, receiver_index);
        if let (Some(s_idx), Some(r_idx)) = (sender_index, receiver_index) {
            if s_idx != r_idx {
                let (left, right) = self.users.split_at_mut(std::cmp::max(s_idx, r_idx));
                let (sender, receiver) = if s_idx < r_idx {
                    (&mut left[s_idx], &mut right[0])
                } else {
                    (&mut right[0], &mut left[r_idx])
                };


                sender.transfer(receiver, amount);
                let transaction = Transaction {
                    sender: sender.clone(),
                    receiver: receiver.clone(),
                    amount,
                };
                self.add_transaction(transaction);
            }
        }
                
            
        }
    
    
}

#[derive(Debug, Clone)]
struct User {
    name: String,
    account_id: String,
    balance: u128,
}


impl User {
    fn new(name: String, account_id: String, balance: u128) -> Self {
        Self {
            name,
            account_id,
            balance,
        }
    }

    fn add_balance(&mut self, amount: u32) {
        self.balance += amount as u128
    }

    fn transfer(&mut self, receiver: &mut User, amount: u32) {
        self.balance -= amount as u128;
        receiver.balance += amount as u128;
        println!(
            "{} balance:{}, {} balance: {}",
            self.name, self.balance, receiver.name, receiver.balance
        )
    }
}

#[derive(Debug, Clone)]
struct Transaction {
    sender: User,
    receiver: User,
    amount: u32,
}

struct PaymentProcessor {
    banks: Vec<Bank>,
}

impl PaymentProcessor {
    fn new() -> Self {
        Self { banks: Vec::new() }
    }

    fn add_bank(&mut self, bank: Bank) {
        self.banks.push(bank);
    }
}

fn main() {
    let mut bank1 = Bank::new(2);
    let mut bank2 = Bank::new(3);

    let mut user_bir = User::new("Sada".to_string(), "1".to_string(), 100);
    let mut user_iki = User::new("Asga".to_string(), "2".to_string(), 0);
    
    //user_bir.add_balance(100);
    bank1.add_user(user_bir);
    bank1.add_user(user_iki);
    bank1.process_transfer("1", "2", 20);
    bank1.process_transfer("2", "1", 1);
    println!("bank 1 users {:?}", bank1.users);
    println!("bank1 tranzaksiyalar {:?}", bank1.transactions)
}
  

