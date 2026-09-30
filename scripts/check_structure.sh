#!/usr/bin/env bash
# docs/PROJECT_CONVENTIONS.md のうち機械的に判定できる規約を検査する。
# git で管理されているファイルだけを対象にする。違反があれば終了コード 1。

set -u
cd "$(git rev-parse --show-toplevel)" || exit 2

errors=0
warnings=0

error() { echo "ERROR: $*"; errors=$((errors + 1)); }
warn()  { echo "WARN:  $*"; warnings=$((warnings + 1)); }

files=$(git -c core.quotePath=false ls-files)

# 1. 必須ファイル
for f in README.md README_ja.md .gitignore CLAUDE.md docs/PROJECT_CONVENTIONS.md; do
  [ -f "$f" ] || error "必須ファイルがありません: $f"
done

# 2. 推奨ファイル
[ -f LICENSE ] || warn "LICENSE がありません（README でライセンスを表明している場合は追加してください）"

# 3. git に入れてはいけないもの
forbidden='(^|/)(target|node_modules|dist|__pycache__)/|(^|/)\.env(\..+)?$|(^|/)(\.DS_Store|Thumbs\.db|desktop\.ini)$'
while IFS= read -r f; do
  [ -n "$f" ] || continue
  case "$f" in */.env.example|.env.example) continue ;; esac
  error "git に入れてはいけないファイルです: $f"
done < <(printf '%s\n' "$files" | grep -E "$forbidden" | head -20)

# 4. 使える文字（半角英数字・_ - . と区切りの /）
while IFS= read -r f; do
  [ -n "$f" ] || continue
  error "使えない文字（スペース・全角など）を含む名前です: $f"
done < <(printf '%s\n' "$files" | LC_ALL=C grep -vE '^[A-Za-z0-9._/-]+$')

# 5. フォルダ名は小文字（ドットで始まるものは除く）
while IFS= read -r d; do
  [ -n "$d" ] || continue
  error "フォルダ名は小文字にしてください: $d"
done < <(printf '%s\n' "$files" | grep '/' | sed 's|/[^/]*$||' | tr '/' '\n' | sort -u \
           | grep -v '^\.' | LC_ALL=C grep -vE '^[a-z0-9_-]+$')

# 6. Rust のソースは snake_case.rs
while IFS= read -r f; do
  [ -n "$f" ] || continue
  error "Rust のファイル名は snake_case.rs にしてください: $f"
done < <(printf '%s\n' "$files" | grep -E '\.rs$' | grep -vE '(^|/)[a-z0-9_]+\.rs$')

# 7. docs/ 直下の Markdown は UPPER_SNAKE_CASE.md か YYYY-MM-DD_snake_case.md（日本語版は _ja）
while IFS= read -r f; do
  [ -n "$f" ] || continue
  error "docs のファイル名が規約に合いません（UPPER_SNAKE_CASE.md / YYYY-MM-DD_snake_case.md）: $f"
done < <(printf '%s\n' "$files" | grep -E '^docs/[^/]+\.md$' | sed 's|^docs/||' \
           | grep -vE '^([A-Z0-9]+(_[A-Z0-9]+)*|[0-9]{4}-[0-9]{2}-[0-9]{2}_[a-z0-9]+(_[a-z0-9]+)*)(_ja)?\.md$' \
           | sed 's|^|docs/|')

echo
echo "検査結果: エラー ${errors} 件 / 警告 ${warnings} 件"
[ "$errors" -eq 0 ]
