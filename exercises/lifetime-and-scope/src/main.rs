#![allow(unused_mut)]
/*
ゴール：
- コンパイルエラーを修正してください
- 1でコメントアウトされている部分を戻すと、何が起こるか確認してください
- コンパイルエラーが発生した場合は、それを修正してください
*/
fn main() {
    let source = "hello, world".to_string();
    {
        let text = &source;
        p(text, "text");
        p(text, text);
    }


    let a;
    let mut text;
    {
        a = "new source".to_string();

        p(&a, "&source");
        text = &a;
        p(text, "text");
    } /* error occurs because the target of text disappears */
    p(text, "text");
}

fn p(text: &str, source: &str) {
    println!("{:?} from {}", text, source);
}