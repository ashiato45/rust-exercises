#![allow(dead_code)]
fn main() {
    let (word_a, word_b) = words();
    let message = format!("{}{}！", word_a, word_b);
    output(message);
    // ゴール4：次の行をアンコメントしても実行できるようにしてください
    //output(message);
}

fn words() -> (String, String) {
    (format!("こんにちは世界"), format!("世界"))
}

fn output(text: String) {
    let (text, kanji_only) = remove_hiragana(text);
    println!("{}", kanji_only);
    /*
    ゴール2：次の行をアンコメントすると何がおきるでしょうか？
    これをコンパイルを通すにはどうすれば良いでしょうか？
    エラーコードE0382でぐぐると詳細なエラーメッセージがわかるぽい．
    →貸したものは返してもらう!1つの方法は，返り値として返してもらうこと．
    */
    println!("ひらがなを抜き取ると：{:?} → {:?}", text, kanji_only);

    /*
    ゴール3：データをコピーせずにコンパイルを通すにはどおすれば良いでしょうか？
    所有権の移動のみを使って解決してください
    */
}

fn remove_hiragana(text: String) -> (String, String) {
    /*
     ゴール1：コンパイルを通すには何を変更すれば良いでしょうか
     Rustの変数は基本的に書き換えられない!result.pushは変更しているのでコンパイルエラーになる．
    */
    let mut result = String::new();
    for c in text.chars() {
        if c < 'ぁ' || 'ん' < c {
            result.push(c);
        };
    }
    (text, result) /* returnって書いてないけどこれが返り値 */
}
