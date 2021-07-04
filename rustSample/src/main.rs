// メイン関数
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
}

/**
 * print関数
 */
fn print (s: Box<[u8]>) {
    println!("{:?}", s);
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
