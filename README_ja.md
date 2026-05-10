🛡️ Prexus Connect (OSS Edition)

🌐 English | 日本語

Prexus Connect は、分散システムにおける「責任共有モデル」を具現化し、ベンダーと発注者の双方をシステム障害や法的リスクから守るために設計された、Rust製の次世代軽量APIゲートウェイです。

本OSS版は、Prexus SaaSエコシステムの「入り口（フリーミアム）」として、開発者が異常系テストを容易に行える環境を提供し、デバッグ工数の削減とシステム堅牢性の向上を支援します。データベース（DB）への依存を完全に排除したインメモリ設計のため、面倒なセットアップは一切不要。コマンド一発で誰でも数秒で起動できます。

✨ コア機能 (Core Features)

1. Prexus Crucible (カオスエンジニアリング・モック)

一般的なWeb開発で回避が困難な「異常系」を意図的に引き起こし、アプリケーションのエラーハンドリングと耐障害性を運用前にテストします。

Timeout Simulation: 通信スパイクや遅延（5秒）を再現。

Conflict Simulation: データ書き換え競合（HTTP 409）を再現。

Data Loss Simulation: サーバー/DBの物理障害（HTTP 500）を再現。

2. Sentinel Audit Logger (コンプライアンス・ロガー)

すべての通信を自動で記録し、機密情報の漏洩を防ぐインテリジェント・ロギングを提供します。

Auto-Masking: URLパスに含まれるメールアドレス等（@を含む文字列など）の機密情報を検知し、自動で masked_path に置換して安全に記録。

Performance Tracking: リクエストごとのレイテンシをミリ秒単位で計測。

3. Billing & Rate Limiter (多層型キャップ制 / フェイルセーフ)

外部からの異常なトラフィックや、バグによる無限ループからシステムと財務を守るオート・スロットリング機能です。

Tenant Isolation: x-tenant-id ヘッダーによるテナントごとの利用状況の個別集計。

Safety Valve: 過度なリクエスト（上限超過）を検知した場合、後続の処理を行わず通信を物理レベルで遮断。

🛠️ クイックスタート

必須環境

Rust / Cargo (最新の安定版を推奨)

インストールと起動

# リポジトリのクローン
git clone [https://github.com/your-username/prexus-connect-oss.git](https://github.com/your-username/prexus-connect-oss.git)
cd prexus-connect-oss

# サーバーの起動
cargo run


🚀 Prexus Connect (OSS) started on port 8080... と表示されれば準備完了です。

🧪 テスト実行例 (Usage)

別ターミナルを開き、以下のコマンドでAPIゲートウェイの挙動を確認できます。

平常時の導通確認

curl -v -X POST http://localhost:8080/api/crucible/test


遅延（Timeout）のシミュレーション

curl -v -X POST http://localhost:8080/api/crucible/test -H "x-crucible-scenario: timeout"


データ競合（Conflict 409）のシミュレーション

curl -v -X POST http://localhost:8080/api/crucible/test -H "x-crucible-scenario: conflict"


監査ログの自動マスキング機能の確認
（サーバー側の標準出力に masked_path として記録されます）

curl -v -X GET "http://localhost:8080/api/mock/generic?user=test@example.com"


💡 よくある質問 (Q&A)

Q. このOSSで検証できるのはRust製のアプリケーションだけですか？
A. いいえ、対象となるアプリケーションの言語は「一切問いません」。
Prexus Connectは独立したAPIゲートウェイ（Webサーバー）として動作するため、Python、Node.js、PHP、Ruby、Go、Javaなど、あらゆる言語・フレームワークで開発されたアプリの前段に配置して検証・連携が可能です。

🔮 展望：Prexusエコシステムについて (Roadmap)

この Prexus Connect (OSS Edition) は、私たちが構想する「Prexus エコシステム」のエントランス（入口）に過ぎません。本OSS版は、Prexusが提唱する「保険的価値」の一部を無料で体験できるものです。

将来的に、より高度な要件を満たすためのエンタープライズ向け機能の提供を予定しています。

Crucible Pro: 医療・金融グレードの高度なテストシナリオ拡張パック。

Crucible Verified: テストをクリアした強固なシステムに対する公式の堅牢性証明書の発行。

The Vault: 法的証拠能力を持ち、1文字の改ざんも許さない絶対安全なデータ保存庫の統合。

まずはこのOSSゲートウェイを通じて、あなたのアプリケーションの堅牢性とコンプライアンスを極限まで高めてみてください。

📄 ライセンス

本プロジェクトは MITライセンス の下で公開されています。

Developed by Akio Akasaka (Prexus Founder)
Empowering Trust through Resilient Architecture.