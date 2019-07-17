# インタフェース（取得系）

## これまで

　取得系メソッドのインタフェースは以下を想定していた。

```rust
impl<T> LinkedList<T> {
    pub fn get(&mut self, index: u32) -> &mut T { return &mut self.head.as_mut().unwrap().item; }
    pub fn next(&mut self) -> &mut T { return &mut self.head.as_mut().unwrap().item; }
}
```

　でも、よく考えたら、以下のような要件があった。

* 所有権をムーブしたいときと、したくないときがある
* 全体からある位置にある要素をひとつだけ取得したいときと、ひとつずつすべて取得したいときがある
* 配列のように`list[index]`という糖衣構文を実装したい（`list.as_mut_from_index(index);`とかは嫌）

## パターン分類

所有権ムーブ|取得対象|メソッド
------------|--------|--------
する|1個のみ|`next()`
する|すべて(1個ずつ)|`into_iter()`
しない|1個のみ|`as_ref_from_index()`, `as_mut_from_index()`
しない|すべて(1個ずつ)|`iter()`, `iter_mut()`


