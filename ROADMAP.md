# toy_ufdb ロードマップ

Union-Find（素集合データ構造）を核にしたトイDB。Rust製、CLI(REPL)、キーのみ管理、v1は永続化なし。

## Phase 0: プロジェクト初期化
- [x] `cargo init --name toy_ufdb`（バイナリクレート）
- [x] `src/lib.rs`（コアロジック）と `src/main.rs`（REPL）に分割する構成にする
- [x] `.gitignore` に `/target` を追加

## Phase 1: Union-Findコア (`src/lib.rs` or `src/union_find.rs`)
データ構造:
- `HashMap<String, usize>` … キー → 内部インデックス
- `Vec<usize>` parent配列
- `Vec<usize>` rank または size配列（union by rank/size用）

実装する操作:
- `make_set(&mut self, key: &str)` … 未登録キーなら新しい集合として追加。既存なら何もしない
- `find(&mut self, key: &str) -> Option<String>` … 代表元のキーを返す。path compression込み。存在しないキーは `None`
- `union(&mut self, a: &str, b: &str)` … 2つの集合を統合。存在しないキーは内部で `make_set` して扱う
- `connected(&mut self, a: &str, b: &str) -> bool` … 同じ集合か判定

設計判断ポイント（実装しながら決める）:
- `find` の再帰 vs ループ実装（再帰は大量データでスタックオーバーフローの可能性あり → ループ推奨）
- union by rank と union by size のどちらにするか（size の方が後の「グループ一覧・グループサイズ」機能と相性が良い）
- 存在しないキーに対する `union`/`connected` の挙動（自動登録 or エラー）を最初に決めて一貫させる

単体テストで最低限カバーすべきケース:
- 単純な union → find で同じ代表元になる
- 推移的な union（a-b, b-c → a と c が connected）
- 未登録キーの find/connected
- 同じキーを2回 union しても壊れない（冪等性）
- path compression 後も find の結果が変わらないこと

## Phase 2: REPL / CLI (`src/main.rs`)
- コマンドのパースには `clap`（derive API）を使う
  - `Cli { command: Commands }` を `#[derive(Parser)]`、`Commands` を `#[derive(Subcommand)]` で定義
  - REPLは標準入力を1行ずつ読み、空白分割した引数の先頭にダミーのプログラム名を足してから `Cli::try_parse_from(...)` に渡す（clapは本来argv全体を1回パースする前提のため、REPLの1行ごとの入力もこの形に合わせる）
  - パースエラーは clap が生成するメッセージをそのまま表示できる（パニックさせない）
- 対応コマンド（サブコマンドとして定義。詳細はREADMEの「CLIコマンド」参照）:
  - `INSERT <key>` … `make_set`
  - `MERGE <a> <b>` … `union`
  - `SAME <a> <b>` … `connected`
  - `GROUPS` … 代表元ごとにグループをまとめて一覧表示
  - `exit` / `quit`
  - `help` は clap が自動生成するので個別実装は不要
  - `FIND`（代表元キーをそのまま返すコマンド）は採用しない。代表元はunion by sizeの内部ロジック次第でユーザーからは予測できない値になるため、コマンドとして見せる価値が薄いと判断（詳細はREADME参照）。`find` 自体は `unite`/`connected`/`GROUPS` の内部実装としては引き続き使う
- `Cargo.toml` に `clap`（`features = ["derive"]`）を追加する

## Phase 3: グループ機能の仕上げ
- [ ] `GROUPS` コマンド: `HashMap<代表元, Vec<キー>>` を組み立てて表示
- [ ] （任意）`size <key>`: キーが属するグループのサイズを表示

## Phase 4: テスト・仕上げ
- [ ] `tests/` 以下に統合テスト（ライブラリAPIを直接呼ぶ形でOK）
- [ ] `cargo clippy` / `cargo fmt` を通す
- [ ] README に使い方（コマンド例）を記載

## 既知の制約（v1ではやらない・やれないこと）
- **削除・分割不可**: 古典的Union-Findは一度unionした集合を分割する操作をサポートしない。「union解除」が欲しくなったら別データ構造（もしくはunion履歴を全部保持してロールバックするワークアラウンド）が必要になる点は最初から認識しておく。
- **永続化なし**: プロセスを終了するとデータは消える。
- **値（value）なし**: 要素はキーのみ。KVとして使いたくなったら Phase 5 で拡張。

## Phase 5以降（将来の拡張候補、v1には含めない）
- 各要素への value 付与 + union 時のマージルール
- ファイルへのスナップショット保存/読み込み（永続化）
- 簡易TCPサーバー化（redis風の行プロトコル）
- union履歴のロールバック（union-findの限界を回避する別実装が必要）
- CSVインポート機能（大量データの一括union投入）。REPLは標準入力を1行ずつ読むだけなので、`union a,b` 形式の行を並べたファイルを `toy_ufdb < data.txt` のようにリダイレクトすれば当面は代用可能。ヘッダー行やクォート、キー以外のカラム（value）を扱いたくなったら本格的なCSVパーサーを検討する
- GUI（Tauri）。コアAPIが安定するまでは同一リポジトリのCargoワークスペース内に `gui/` 等として置き、別リポジトリへの切り出しはAPI安定後に判断する
