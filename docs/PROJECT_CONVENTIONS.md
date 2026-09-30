# プロジェクト規約（ファイル名・フォルダ構成）

どのプロジェクトでも同じ場所に同じ種類のファイルがあり、名前を見れば中身の種類がわかる状態を目指すための規約です。
このリポジトリは、この規約を最初に適用した見本です。

規約のうち機械的に判定できるものは `scripts/check_structure.sh` が検査し、GitHub Actions（`.github/workflows/structure_check.yml`）でも push / PR のたびに自動実行されます。

---

## 1. 標準フォルダ構成

```
<project-root>/
├── .github/
│   └── workflows/        # CI 設定
├── docs/                 # 設計書・規約・議事録などのドキュメント
├── scripts/              # 開発・運用の補助スクリプト
├── src/                  # ソースコード
├── tests/                # テストコード（必要になったら作成）
├── .gitattributes
├── .gitignore
├── CLAUDE.md             # AI アシスタント向けの作業ルール
├── README.md             # 英語版 README
└── README_ja.md          # 日本語版 README
```

言語固有のファイル（Rust なら `Cargo.toml` / `Cargo.lock`）はルート直下に置きます。

### 必須ファイル

| ファイル | 役割 |
|---|---|
| `README.md` | 英語の概要・使い方 |
| `README_ja.md` | 日本語の概要・使い方 |
| `.gitignore` | ビルド生成物・秘密情報を git に入れないための設定 |
| `CLAUDE.md` | AI アシスタントがこの規約に従うための指示 |
| `docs/PROJECT_CONVENTIONS.md` | この規約書 |
| `docs/INDEX.md` | `docs/` の目次（各文書に何が書いてあるか） |

### 推奨ファイル

| ファイル | 役割 |
|---|---|
| `LICENSE` | ライセンス本文（README でライセンスを表明している場合は必ず置く） |

---

## 2. 命名規則

### 共通ルール（すべてのファイル・フォルダ）

- 使える文字は **半角英数字・`_`・`-`・`.` のみ**。
- **スペース・全角文字（日本語を含む）は使わない。** OS やツールによって文字化け・パスの不具合が起きるため。
- 日本語で説明したい内容はファイルの中に書く。

### フォルダ名

- **すべて小文字**（`src`, `docs`, `scripts`）。単語の区切りは `_`。
- 例外: `.github` のようにドットで始まるツール指定のフォルダ。

### ソースコード

言語の標準に従います。

| 言語 | ルール | 例 |
|---|---|---|
| Rust | `snake_case.rs` | `oss_audit_logger.rs` |
| Python | `snake_case.py` | `billing_monitor.py` |
| TypeScript / JavaScript | `kebab-case.ts`（React コンポーネントは `PascalCase.tsx`） | `rate-limiter.ts` |

#### 接頭辞（プレフィックス）

OSS 版として公開するモジュールには `oss_` を付けます（例: `oss_crucible_env.rs`）。
エントリポイント（`main.rs` など）には付けません。

### ドキュメント（`docs/` 配下）

| 種類 | 形式 | 例 |
|---|---|---|
| 常に最新版を保つ文書（規約・設計書） | `UPPER_SNAKE_CASE.md` | `PROJECT_CONVENTIONS.md`, `ARCHITECTURE.md` |
| 日付のある記録（議事録・調査メモ） | `YYYY-MM-DD_snake_case.md` | `2026-09-30_rate_limit_research.md` |

### 目次（`docs/INDEX.md`）

- `docs/` に文書を追加・改名・削除したら、**同じコミットで `docs/INDEX.md` も更新する**。
- 各文書について「ファイルへのリンク」と「何が書いてあるかの一行説明」を書く。
- 一行説明には、後で検索しそうな日本語のキーワードを含める（ファイル名は英数字のみのため、日本語で探す手がかりはここに置く）。
- `docs/` 直下の文書が `INDEX.md` から参照されていないと、検査スクリプトがエラーにする。

### 多言語版

- 英語版を基本の名前にし、日本語版は **末尾に `_ja`** を付ける。
- 例: `README.md` / `README_ja.md`、`ARCHITECTURE.md` / `ARCHITECTURE_ja.md`

### ルート直下で大文字を使ってよいファイル

慣例で大文字にするファイルのみ: `README*.md`, `CLAUDE.md`, `LICENSE`, `CHANGELOG.md`, `CONTRIBUTING.md`, `Cargo.toml`, `Cargo.lock` など。

---

## 3. git に入れないもの

| 対象 | 理由 |
|---|---|
| ビルド生成物（`target/`, `node_modules/`, `dist/`, `__pycache__/`） | 再生成できる・容量が大きい・差分が読めなくなる |
| 秘密情報（`.env`, 鍵ファイル） | 漏えい防止 |
| OS・エディタの自動生成物（`.DS_Store`, `.vscode/` など） | 個人環境に依存する |

---

## 4. 既存プロジェクトへの適用手順

1. このファイル・`CLAUDE.md`・`scripts/check_structure.sh`・CI 設定をコピーする。
2. `bash scripts/check_structure.sh` を実行し、違反を一覧で確認する。
3. 名前の変更は `git mv 旧名 新名` で行う（履歴が引き継がれる）。
4. 誤って git に入っている生成物は `.gitignore` に追加したうえで `git rm -r --cached <パス>` で管理対象から外す（手元のファイルは消えない）。
5. 1 リポジトリにつき 1 つの PR で行う。
