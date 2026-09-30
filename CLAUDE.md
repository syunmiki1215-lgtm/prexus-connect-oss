# CLAUDE.md

## ファイル・フォルダの規約

ファイルやフォルダを作成・改名するときは、必ず `docs/PROJECT_CONVENTIONS.md` に従うこと。

- ファイル名・フォルダ名は半角英数字と `_` `-` `.` のみ（スペース・日本語は不可）
- Rust のソースは `snake_case.rs`、OSS 版モジュールには `oss_` 接頭辞
- ドキュメントは `docs/` に置き、日本語版は末尾に `_ja` を付ける
- `docs/` に文書を追加・改名・削除したら、同じコミットで `docs/INDEX.md` の目次も更新する
- ビルド生成物（`target/` など）や `.env` はコミットしない

変更後は `bash scripts/check_structure.sh` を実行して違反がないことを確認すること。

## ビルド

```
cargo build
cargo run   # http://127.0.0.1:8080 で起動
```
