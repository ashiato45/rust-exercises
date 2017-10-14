/*
ゴール：
1. 商品名とその価格を整形して表示する関数show_priceを実装してください
2. コメントアウトしてある行のコメントを外すと、何が起こるか確認してください
*/

use std::collections::HashMap;

fn main() {
    let mut catalog = HashMap::new();
    catalog.insert("オレンジ", 200);
    catalog.insert("りんご", 90);
    catalog.insert("パイナップル", 380);
    catalog.insert("びわ", 400);
    catalog.insert("メロン", 980);
    catalog.insert("スイカ", 2000);

    for item in catalog.keys() {
        let result = catalog.get(item);
        /* resultにはいまoptionがかかっている．Someならとりだし，noneならなにもしない */
        let price = match result{
            Some(price) => *price,
            None => 0
        };
        // match result{
        //     Some(price) => show_price(item, price),
        //     None => ()
        // }
        show_price(item, price);
        // show_price(item, catalog.get(item));
    }
    // show_price("鮭", catalog.get("鮭"));
}

fn aaa() -> Option<u32>{
    Some(10)
}

fn show_price(name: &str, price: u32) {
    /* これをよみましょう：https://qiita.com/tatsuya6502/items/cd41599291e2e5f38a4a */
    println!("{}は{}円です", name, price);
}

