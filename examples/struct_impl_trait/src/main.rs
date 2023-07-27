// struct 关键字用于定于一个数据结构，可以类比面向对象的class
// impl 关键字可以为struct实现关联的成员方法
// trait （特征）是对公共行为的抽象，类比面向对象与严重的接口。


// struct 定义一个银行账户的结构体(类)，包含账户id和余额两个字段
struct BankAcount {
    id: u64,
    balance: i64,
}

// impl 银行账户支持新建/存款/取款/转账等操作，我们使用impl给BankAcount结构提关联这些方法的实现。

impl BankAcount {
    // 新建一个余额为0的账户，此为静态方法
    pub fn new(id: u64) -> Self {
        Self {
            id, 
            balance: 0i64,
        }
    }

    // 成员方法，存入
    pub fn deposit(&mut self, amount: u32) {
        println!("before deposit: {:?}", self);
        self.balance += amount as i64;
        println!("after deposit {amount}: {:?}", self);
    }

    // 成员方法，取出
    pub fn withdraw(&mut self, amount: u32) {
        let temp_balance = self.balance;
        self.balance -= amount as i64;
        println!("ba{} 取出 {amount}, {} -> {}", self.id, temp_balance, self.balance);
    }

    // 成员方法，对target账户转账
    pub fn transfer(&mut self, target: &mut BankAcount, amount: u32) -> bool {
        let amount = amount as i64;

        if self.balance >= amount {
            self.balance -= amount;
            target.balance += amount;

            println!("ba{} transfer amount {} to {} success", self.id, amount, target.id);
            true

        } else {

            println!("ba{} transfer amount {} to {} failed, balance is not enough", self.id, amount, target.id);
            false

        }
    }
}

// 让BankAcount对象可以被println!打印出来
impl std::fmt::Debug for BankAcount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BankAcount({}, {})", self.id, self.balance);
        Ok(())
    }
}

fn main() {
    let mut ba1 = BankAcount::new(1);
    let mut ba2 = BankAcount::new(2);

    // ba1 存 100
    ba1.deposit(100);
    // ba1 取 50
    ba1.withdraw(50);
    // ba1 转 50 给 ba2 成功
    let result = ba1.transfer(&mut ba2, 50);
    println!("ba1 转 50 给 ba2 返回状态：{result}");
    // ba1 再转 50 给 ba2 失败
    let result = ba1.transfer(&mut ba2, 50);
    println!("ba1 再转 50 给 ba2 返回状态：{result}");
}
