# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## プロジェクト概要

`toy_ufdb`: Union-Find（DSU / 素集合データ構造）を核にしたRust製のトイDB。CLI(REPL)経由でキーのみを管理する（値の付与や永続化はv1以降）。

## バージョニング方針

メジャーバージョンごとにリポジトリを分ける。

- **v0（このリポジトリ）**: オンメモリのみ。ストレージ（永続化）は実装しない
- **v1（別リポジトリ）**: ストレージ（永続化）を実装する

このリポジトリのスコープは常にv0（オンメモリ）であることを前提に作業する。永続化が必要な機能を思いついても、このリポジトリには実装しない。

## 構成

- `src/union_find.rs` — `UnionFind`構造体。`usize`インデックスのみを扱う純粋なUnion-Find実装（`parent`/`size`の2つの`Vec`）。文字列キーの存在は知らない
- `src/lib.rs` — `Ufdb`構造体。`HashMap<String, usize>`でキーとインデックスを橋渡しし、`UnionFind`に処理を委譲する
- `src/main.rs` — REPL本体。`clap`（derive API）でコマンドをパースし、標準入力を1行ずつ読んで実行する

詳細な設計判断・データ構造の説明は `README.md`、実装計画・進捗は `ROADMAP.md` を参照。

## コマンド

- `cargo build`
- `cargo test`
- `cargo run`（起動後はREPLで `INSERT` / `MERGE` / `SAME` / `GROUPS` / `EXIT` などを1行ずつ入力する）

## 作業の進め方

このプロジェクトの実装コードは基本的にユーザー自身が書く。Claude Codeの役割は:

- `ROADMAP.md` / `README.md` / `CLAUDE.md` の作成・更新
- 設計上の概念について説明する（コードを渡すのではなくSocratic的に）
- ユーザーが書いたコードのレビュー・指摘
- `cargo build` / `cargo test` によるビルド・動作確認

ユーザーから明示的に依頼されない限り、実装コードを直接書かない。
