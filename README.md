# 📊 Rust & Dioxus 単位変換アプリ (Unit Converter with History)

Rust とクロスプラットフォームGUIフレームワーク **Dioxus** を使用して開発された、デスクトップ向けの高性能な単位変換アプリケーションです。
変換した履歴をリアルタイムで保存し、データの永続化（CSV管理）に対応しています。

## ✨ 主な機能

- **型安全で高速な単位変換**: Rustの強力な型システムによる、安全で高速な計算処理。
- **モダンなデスクトップGUI**: `Dioxus` を採用し、HTML/CSSライクな直感的で軽量なUIを実現。
- **データの永続化 (CSV保存)**: 変換履歴は自動的に `conversion_history.csv` に保存され、アプリを閉じても消えません。
- **履歴の一覧表示**: 過去に変換したデータをタイムスタンプ付きでテーブル表示。

## 🛠️ 開発環境 / 動作要件

- **Rust**: 1.70.0 以上
- **OS**: Windows / macOS / Linux

## 🚀 クイックスタート

### 1. リポジトリの準備
プロジェクトのディレクトリに移動します。
```bash
cd my_converter_app
2. 依存関係の確認 (Cargo.toml)
以下のパッケージが設定されていることを確認してください。

Ini, TOML
[dependencies]
dioxus = { version = "0.5", features = ["desktop"] }
plotly = "0.8"
chrono = "0.4"
3. アプリケーションの実行
初回の実行時は、依存ライブラリのコンパイルが行われるため少し時間がかかります。

Bash
cargo run
📂 プロジェクト構造
Plaintext
my_converter_app/
├── Cargo.toml               # 依存ライブラリ・機能の設定
├── README.md                # 本ドキュメント
├── conversion_history.csv  # 実行時に自動生成される履歴データベース
└── src/
    └── main.rs              # アプリケーションのメインロジック・UI
📈 今後の拡張予定 (Roadmap)
[ ] Plotly グラフの完全統合: 保存された履歴データをインタラクティブな折れ線グラフとして描画する機能の実装。

[ ] 多機能化: 「重さ（g, kg, lb）」や「リアルタイム通貨為替（API連携）」カテゴリーの追加。

[ ] WebAssembly (Wasm) 対応: コードを変更せずに、ブラウザ上で動くWebアプリとしてもビルド可能にする。

📝 ライセンス
本プロジェクトは MIT ライセンスの下で公開されています。
<img width="1918" height="1140" alt="スクリーンショット 2026-05-18 065559" src="https://github.com/user-attachments/assets/a0024445-ee62-421f-aee5-9e8254d31984" />
