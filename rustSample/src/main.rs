// Eqを実装する。
#[derive(Eq, PartialEq)]
struct A(i32);
// PartialOrdを実装するためにPartialEqが必要
#[derive(PartialEq, PartialOrd)]
struct B(f32);
// Copyを実装するためにCloneを実装する。
#[derive(Copy, Clone)]
struct C;
#[derive(Clone)]
struct D;
#[derive(Debug)]
struct E;
#[derive(Default)]
struct F;
// スレッドライブラリをインポートする。
use std::thread;
// Rcライブラリをインポートする。(シングルスレッド用)
use std::rc::Rc;
// Arcライブラリをインポートする。(マルチスレッド用)
use std::sync::{Arc, Mutex};

/**
 * メイン関数
 */ 
fn main() {
    println!("Hello, world!");
    // 文字列の例
    let s1: String = String::from("Hello, world!");
    let s2: &str = &s1;
    let s3: String = s2.to_string();
    // タプルの例
    let mut t = (1, "2");
    t.0 = 2;
    t.1 = "3";
    // 配列の例
    let mut a: [i32; 3] = [0, 1, 2];
    let b: [i32; 3] = [0; 3];
    a[1] = b[1];
    a[2] = b[2];
    print!("{:?}", &a[1..3]);
    // ユーザー定義型変数の例
    struct Person {
        name: String,
        age: u32,
    }
    // Person型の変数を初期化する。
    let p = Person {
        name: String::from("John"),
        age: 8,
    };
    // 列挙型変数の例
    enum Event {
        Quit,
        KeyDown(u8),
        MouseDown { x: i32, y: i32 },
    }
    // 変数を用意する。
    let e1 = Event::Quit;
    let e2 = Event::MouseDown  { x: 10, y: 10 };
    // 以下頻出するライブラリの型の例
    // Optionの例(データの存在有無を表す)
    pub enum Option<T> {
        None,
        Some(T)
    }
    // ベクタ型の例
    let v1 = vec![1, 2, 3, 4, 5];
    // 0を5つ埋めて初期化する。
    let v2 = vec![0; 5];
    print!("{}", v1[0]);
    // すべての要素を出力させる。
    for element in &v1 {
        print!("{}", element);
    }
    // Boxの例
     let byte_array = [b'h', b'e', b'l', b'l', b'o'];
    // print関数を呼ぶ
    print(Box::new(byte_array));
    // 以下、matchの例
    enum Color {
        Red,
        Blue,
        Green,
    }

    let c = Color::Red;
    // match文
    match c {
        Color::Red => println!("Red"),
        Color::Blue => println!("Blue"), 
        Color::Green => println!("Green"), 
    }
    // Iteratorの例
    // Iter型の変数を用意する。
    let it = Iter {
        current: 0,
        max: 10,
    };
    // ループ文
    for num in it {
        println!("{}", num);
    }
    // プログラム外のリソースにアクセスするマクロ
    println!("definded in file: {}", file!());
    println!("definded on line: {}", line!());
    println!("is test: {}", cfg!(unix));
    println!("CARGO_HOME: {}", env!("CARGO_HOME"));
    // アサーション用のマクロ (cargo run --releaseで実行しないとエラーになる。)
    assert!(true);
    debug_assert!(false);
    assert_eq!(1, 1);
    debug_assert_eq!(1, 1);
    assert_ne!(1, 0);
    debug_assert_ne!(1, 0);
    // Aは、一致比較可能
    println!("{:?}", A(0) == A(1));
    // Bは、大小比較可能
    println!("{:?}", B(1.0) > B(0.0));
    // Cは、ムーブではなくコピーされる
    let c0 = c;
    let _c1 = c0;
    // let _c2 = c0;
    // Dは、clone可能
    let d0 = D;
    let _d1 = d0.clone();
    // Eは、デバックプリント可能
    println!("{:?}", E);
    // Fは、default可能
    let _f = F::default();
    // ジェネリクスの実装例
    let t1 = make_tuple(1, 2);
    let t2 = make_tuple("Hello", "World!");
    let t3 = make_tuple(vec![1, 2, 3], vec![4, 5, 6]);
    let t4 = make_tuple(3, "years old ");
    // Droppable型の変数を定義する。
    {
        let d = Droppable;
    }
    println!("The Droppable should be released at the end of block.");

    // 以下、スレッドプログラミングの例
    // Vec型の変数を用意する。
    let mut handles = Vec::new();
    // vec型の変数を用意する。
    let mut data = Arc::new(Mutex::new(vec![1; 10]));
    // 10回ループする。
    for x in 0..10 {
        let data_ref = data.clone();
        // スレッドを10個生成する。
        handles.push(thread::spawn(move || {
            // lockを使ってdataへの可変参照を得る(排他的制御のため)
            let mut data = data_ref.lock().unwrap();
            data[x] += 1;
        }));
    }
    // 各スレッドの終了を待つ
    for handle in handles {
        let _ = handle.join();
    }
    dbg!(data);
}

struct Iter {
    current: usize,
    max: usize,
}

struct Droppable;

/**
 * print関数
 */
fn print (s: Box<[u8]>) {
    println!("{:?}", s);
}

/**
 * ジェネリクス用の関数
 */
fn make_tuple<T, S> (t:T, s:S) -> (T, S) {
    (t, s)
}

/**
 * Iteratorトレイトを適用させる。
 */
impl Iterator for Iter {
    // 出力する型の紐付けをする。
    type Item = usize;
    // next()メソッドを実装する。
    fn next(&mut self) -> Option<usize> {
        self.current += 1;
        // 条件分岐
        if self.current - 1 < self.max {
            Some(self.current - 1)
        } else {
            None
        }
    }
}

/**
 * Dropトレイトを適用させる。
 */
impl Drop for Droppable {
    fn drop(&mut self) {
        println!("Resource will be released!");
    }
}

/*
// Resultの例(処理結果を表す変数)
fn result() {
    pub enum Result<i32, String> {
        Ok(i32),
        Err(String),
    }
    let result: Result<i32, String> = Ok(200);
    // パターンマッチの例
    match result {
        Ok(code) => print!("code: {}", code),
        Err(err) => print!("err: {}", err),
    };
    if let Ok(code) = result {
        print!("code: {}", code);
    };
    // unwrap_or()の例
    print!("code: {}", result.unwrap_or(-1));
    let result: Result<i32, String> = Err("error".to_string());
    print!("code: {}", result.unwrap_or(-1));
    
    let result: Result<i32, String> = Ok(200);
    let next_result = result.and_then(func);
    let result: Result<i32, String> = Err("error".to_string());
    let next_result = result.and_then(func);
}

// and_then()の例
fn func (code: i32) -> Result<i32, String> {
    print!("code: {}", code);
    Ok(100);
}
 */
