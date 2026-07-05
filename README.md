# toy_ufdb

Union-Find（素集合データ構造）を核にしたトイDB。Rust製。

## データ構造

### `UnionFind`（`src/union_find.rs`）

Union-Findのコアアルゴリズムだけを担当する、純粋にインデックス（`usize`）ベースの実装。文字列キーなどの存在は一切知らない。

内部は2つの配列で構成される:

- `parent: Vec<usize>` — 各要素がどの要素を親として指しているかを表す。`parent[i] == i` のとき、`i` はその集合の代表元（root）。
- `size: Vec<usize>` — 各要素が root のときにだけ意味を持つ、その集合に属する要素数。

主な操作:

| メソッド | 役割 |
| --- | --- |
| `add()` | 新しい要素を1つ追加し、自分自身だけの集合（サイズ1）として登録する。追加された要素のインデックスを返す |
| `find(x)` | `x` が属する集合の代表元（root）のインデックスを返す。辿る過程で経路圧縮（path compression）を行い、以降の呼び出しを高速化する |
| `unite(x, y)` | `x` と `y` の集合を1つにまとめる。union by size（要素数が少ない方を多い方へ繋ぐ）で木の高さを抑える。すでに同じ集合なら何もせず `false` を返す |
| `same(x, y)` | `x` と `y` が同じ集合に属するかを返す |
| `size(x)` | `x` が属する集合の要素数を返す |

要素の追加はすべて `add()` による1個ずつの動的な追加で、生成時に要素数をあらかじめ指定する必要はない。

### `Ufdb`（`src/lib.rs`）

`UnionFind` はインデックスしか扱えないため、その手前に文字列キーとインデックスを橋渡しする層として `Ufdb` を用意している。

```rust
pub struct Ufdb {
    keys: HashMap<String, usize>, // キー文字列 → UnionFind内部のインデックス
    uf: union_find::UnionFind,    // インデックスベースの本体
}
```

呼び出し側は `"apple"` のような任意の文字列キーだけを扱い、`Ufdb` がその都度 `HashMap` を介してインデックスに変換してから `UnionFind` に処理を委譲する。

- `make_set(key)` — キーが未登録なら `UnionFind::add()` で新しいインデックスを確保し `HashMap` に登録する（新規追加なら `true`、既存なら何もせず `false`）
