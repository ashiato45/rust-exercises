/*
ゴール：
1. コンパイルし、実行してください
2. worldを自分の名前に置き換えてみてください
3. 変数 name を使って2を実現してください
4. println!("{}", name)を試しください
5. println!("{}", name)とprintln!("{:?}", name)の違いを確認してください
6. greetingsを呼ぶように変更してください
*/
#![allow(dead_code)]

fn main() {
    // let name = "ashiato45"; /* このときnameはこの文字列へのポインタになるらしい */
    // let name = "ashiato45".to_string();
    let name = "ashiato45".to_string(); /* 型注釈はこうする */
    println!("Hello, {}!", name);
    println!("{}", name);
    println!("{:?}", name);
    greetings(name);
    greetings(name); /* これは失敗する．関数呼出をすると「所有権」は相手に移って，終わったら消滅するのでここで使うことはできない． */
}

fn greetings(name: String) {
    println!("Hello, {}!", name);
}