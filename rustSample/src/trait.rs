trait Tweet {
    // 関数
    fn tweet(&self);

    fn tweet_twice(&self) {
        self.tweet();
        self.tweet();
    }

    fn shout(&self) {
        println!("Uooooooooohhh!!!!!!!");
    }
}

// 構造体を宣言する。
struct Dove;
struct Duck;

// トレイトを適用する。
impl Tweet for Dove {
    fn tweet(&self) {
        println!("Coo!");
    }
}

// トレイトを適用する。
impl Tweet for Duck {
    fn tweet(&self) {
        println!("Quack!");
    }
}

/**
 * メイン関数
 */
fn main() {
    // Dove型の変数を用意する。
    let dove = Dove {};
    dove.tweet();
    dove.tweet_twice();
    dove.shout();
    // Duck型の変数を用意する。
    let duck = Duck {};
    // vec型の変数を用意する。
    let bird_vec: Vec<Box<dyn Tweet>> = vec![Box::new(dove), Box::new(duck)];
    // ループ
    for bird in bird_vec {
        bird.tweet();
    }
}