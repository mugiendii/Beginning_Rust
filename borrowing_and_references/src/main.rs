// borrowing_and_references/
//references and borrowing allow you to refer to some data without taking ownership of it.
//This is useful when you want to pass large data structures to functions without incurring the cost of copying them.
//References are immutable by default, meaning you cannot modify the data they point to unless you explicitly declare them as mutable references.
//


fn main() {
    let  mut my_account = bankAccount::new(
        String::from("Alice"),
        123456,
        9876543210123456,
        1000.0,
    );
    //immutable borrow to check balance
    println!("Initial balance: ${}", my_account.get_balance());
    //mutable borrow to deposit money
    my_account.deposit(500.0);
    println!("Balance after deposit: ${}", my_account.get_balance());

    my_account.withdraw(200.0); 
    println!("Balance after withdrawal: ${}", my_account.get_balance());

    my_account.withdraw(2000.0); // Attempt to withdraw more than the balance
}
   
struct bankAccount {
    name: String,
    account_number: u32,
    card_number: u64,
    balance: f64,
}
impl bankAccount {
    fn new(name: String, account_number: u32, card_number: u64, balance: f64) -> bankAccount {
        bankAccount {
            name,
            account_number,
            card_number,
            balance,
        }
    }

    fn deposit(&mut self, amount: f64) {
        self.balance += amount;
    }

    fn withdraw(&mut self, amount: f64) {
        if amount <= self.balance {
            self.balance -= amount;
        } else {
            println!("Insufficient funds");
        }
    }

    fn get_balance(&self) -> f64 {
        self.balance
    }
    
}