/*
ゴール：
1. データをコピーしなくても、greetを呼び出せるようにしてください
2. "Hello dear rustaceans"ではなく、"Hello rustaceans"となるように、
   2回目のgreetの呼び出しを行なってください
3. 2をスライスを使って実現してください
*/

fn main() {
    let name = format!("dear rustaceans");
    let refname = &name;
    greet(&name); /* だいたいクローンしとけばなんとかなる，けど一瞬所有権を返してもらうみたいなことをしたい． */
    greet(refname);
    greet(refname); /* エラー! */
}

fn greet(name: &String) {
    println!("Hello {}", name);
}

