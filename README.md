# vec-x-rs

任意の固定長の配列を扱うためのライブラリです。
配列を管理するための型`VecX`を提供し、数値演算や代入演算をサポートしています。

また、一意な配列にインデックスを持たせて管理する方法も提供します。

## 使い方

```rust
use vec_x::{VecX, IndexedVecXs};

fn main() {

    // i32型の要素を3つ持つ配列を作成
    let vec: VecX<i32, 3> = VecX::new([1, 2, 3]);


    // 型エイリアスを使用してインスタンスを作成
    type XYZ = VecX<f64, 3>;
    type RGBA = VecX<u8, 4>;

    let point = XYZ::new([1., 2., 3.]);
    let red = RGBA::new([255, 0, 0, 255]);


    // 配列の要素にアクセス
    let vec = VecX::new([1, 2, 3]);

    assert_eq!(vec[0], 1);
    assert_eq!(vec[1], 2);
    assert_eq!(vec[2], 3);


    // 数値演算(+, -, *, /, %)
    let a = VecX::new([1, 2, 3]);
    let b = VecX::new([4, 5, 6]);

    assert_eq!(a + b, VecX::new([5, 7, 9]));


    // 代入演算(+, -, *, /, %)
    let mut a = VecX::new([1, 2, 3]);

    a += VecX::new([4, 5, 6]);

    assert_eq!(a, VecX::new([5, 7, 9]));

    // インデックスでの管理
    type RGB = VecX<u8, 3>;

    let unique_colors = vec![
        RGB::new([255, 0, 0]),
        RGB::new([0, 255, 0]),
        RGB::new([0, 0, 255]),
    ];

    let colors = vec![
        unique_colors[0], // VecX: [255, 0, 0]
        unique_colors[1], // VecX: [0, 255, 0]
        unique_colors[1], // VecX: [0, 255, 0]
        unique_colors[0], // VecX: [255, 0, 0]
        unique_colors[2], // VecX: [0, 0, 255]
        unique_colors[2], // VecX: [0, 0, 255]
    ];

    let indexed_colors = IndexedVecXs::from_vec(colors);

    let IndexedVecXs { values, indices } = indexed_colors;

    // 元データの一意な要素の出現順にインデックスされる
    assert_eq!(values[0], unique_colors[0]);
    assert_eq!(values[1], unique_colors[1]);
    assert_eq!(values[2], unique_colors[2]);

    assert_eq!(indices, vec![0, 1, 1, 0, 2, 2]);
}
```

## ライセンス

Licensed under either of

+ Apache License, Version 2.0, ([LICENSE-APACHE](../vec-x-rs/LICENSE-APACHE)
  or http://www.apache.org/licenses/LICENSE-2.0)
+ MIT license ([LICENSE-MIT](../vec-x-rs/LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

(The English in the README and comments in the source code were translated by DeepL.)
