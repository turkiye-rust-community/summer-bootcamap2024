
struct Bank{
    users: Vec<User>,
    amount: u128,
    fee: u8,
    transactions: Vec<Transaction>
}

#[derive(Debug)]
struct User{
    name:String,
    account_id: String,
    balance: u128,
}

impl User {
    fn new(name:String, account_id: String, balance:u128) -> Self{
        Self{
            name,
            account_id,
            balance
        }
    }

    fn add_balance(&mut self, amount: u32) {
        self.balance += amount as u128
    }

    fn transfer(&mut self, receiver: &mut User, amount: u32) {
        self.balance-=amount as u128;
        receiver.balance += amount as u128;
        println!("{} balance:{}, {} balance: {}", self.name, self.balance, receiver.name , receiver.balance)
    }
}


struct Transaction{
    sender: User,
    receiver: User,
    amount:u32,
    bank: Bank,
}

struct PaymentProcessor{
    banks: Vec<Bank>
    
}



fn main () {
    let mut user_bir = User::new("Sada".to_string(),"1".to_string(), 0);
    let mut user_iki = User::new("Asga".to_string(),"2".to_string(), 0);
    user_bir.add_balance(100);
    user_bir.transfer(&mut user_iki, 10);

    println!("{:?}", user_bir);
    println!("{:?}", user_iki)
    
}