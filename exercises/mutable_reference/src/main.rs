/*
ゴール：
1. concatを参照を使って書き換えてください
2. 1で書き換えたconcatを次のように呼び出すと何が起きるか確認してください
   concat(&mut word_a, &word_b)
3. もし2で問題が起きたなら、それを修正してください
*/

fn main() {
    let (mut word_a, word_b) = words();
    println!("単語1：{:?}\n単語2：{:?}", word_a, word_b);
    concat(&mut word_a, &word_b);
    println!("結合された結果：{:?}", word_a);
}

fn words() -> (String, String) {
    (format!("こんにちは"), format!("世界"))
}

fn concat(prefix: &mut String, postfix: &String) {
    /* 2つ引数をもらったとき，返り値がどこまで生きるかを書くのがライフタイムパラメタらしい */
    prefix.push(' ');
    for c in postfix.chars() {
        prefix.push(c);
    }
}

