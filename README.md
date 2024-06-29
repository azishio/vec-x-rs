# vec-x-rs

固定長の配列を管理するための構造体`VecX`を提供し、`VecX`同士やスカラー値との数値演算や代入演算をサポートします。

Provides a structure `VecX` to manage fixed-length arrays, supporting numeric operations and assignment operations
between `VecX` and scalar values.

その他、要素の一括キャストなど、便利なメソッドも提供します。

Other useful methods, such as batch casting of elements, are also provided.

## 使い方 (Usage)

```rust
use vec_x::{VecX, IndexedVecXs};

fn main() {

    // i32型の要素を3つ持つ配列を作成
    // Create an array with three elements of type i32
    let vec: VecX<i32, 3> = VecX::new([1, 2, 3]);
    let vec: VecX<i32, 3> = VecX::from([1, 2, 3]);
    let vec: VecX<i32, 3> = VecX::from([1; 3]);
    let vec: VecX<i32, 3> = VecX::from(1);


    // 型エイリアスを使用してインスタンスを作成
    // Create an instance using a type alias
    type XYZ = VecX<f64, 3>;
    type RGBA = VecX<u8, 4>;

    let point = XYZ::new([1., 2., 3.]);
    let red = RGBA::new([255, 0, 0, 255]);


    // 配列の要素にアクセス
    // Accessing elements of an array
    let vec = VecX::new([1, 2, 3]);

    assert_eq!(vec[0], 1);
    assert_eq!(vec[1], 2);
    assert_eq!(vec[2], 3);


    // 数値演算(+, -, *, /, %)
    // Numeric operations (+, -, *, /, %)
    let a = VecX::new([1, 2, 3]);
    let b = VecX::new([4, 5, 6]);

    assert_eq!(a + b, VecX::new([5, 7, 9]));

    // スカラーとの演算(+, -, *, /, %)
    // Operations with scalars (+, -, *, /, %)
    let a = VecX::new([1, 2, 3]);

    assert_eq!(a + 1, VecX::new([2, 3, 4]));


    // 代入演算(+, -, *, /, %)
    // Assignment operations (+, -, *, /, %)
    let mut a = VecX::new([1, 2, 3]);

    a += VecX::new([4, 5, 6]);

    assert_eq!(a, VecX::new([5, 7, 9]));

    // スカラーとの代入演算(+, -, *, /, %)
    // Assignment operations with scalars (+, -, *, /, %)
    let mut a = VecX::new([1, 2, 3]);

    a += 1;

    assert_eq!(a, VecX::new([2, 3, 4]));

    // 比較
    // Comparison
    let vec1 = VecX::new([1, 2, 3]);
    let vec2 = VecX::new([1, 2, 3]);
    assert_eq!(vec1, vec2);
    assert!(vec1 <= vec2);
    assert!(vec1 >= vec2);

    let vec1 = VecX::new([1, 2, 3]);
    let vec2 = VecX::new([4, 5, 6]);
    assert!(vec1 < vec2);

    let vec1 = VecX::new([1, 2, 3]);
    let vec2 = VecX::new([1, 2, 2]);
    assert_ne!(vec1, vec2);
    assert!(vec1 > vec2);

    // 要素のキャスト
    // Element casting
    let vec = VecX::new([1, 2, 3]);
    let vec_f64: VecX<f64, 3> = vec.as_();
}
```

## ライセンス (License)

Licensed under either of

+ Apache License, Version 2.0, ([LICENSE-APACHE](../vec-x-rs/LICENSE-APACHE)
  or http://www.apache.org/licenses/LICENSE-2.0)
+ MIT license ([LICENSE-MIT](../vec-x-rs/LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

(The English in the README and comments in the source code were translated by DeepL.)
