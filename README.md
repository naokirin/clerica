# Clerica

Mac向けファイル整理・検索ツール

## 概要

Clericaは、複数のディレクトリにまたがる大量のファイルを一元管理し、柔軟なタグ分類と高速検索を提供するデスクトップアプリケーションです。

## 主な機能

- **複数ディレクトリ管理**: 任意の複数ディレクトリを管理対象として登録
- **タグ分類**: ファイルごとに自由なタグを付与・編集・削除
- **高速検索**: ファイル名による部分一致・ファジー検索
- **ファイル監視**: リアルタイムでのファイル変更追跡
- **ファイル操作**: 削除・移動・Finder連携

## 技術スタック

- **バックエンド**: Rust + Tauri
- **フロントエンド**: React + TypeScript
- **データベース**: SQLite (SQLx)
- **ファイル監視**: notify-rs
- **検索エンジン**: Fuse.js

## 開発環境のセットアップ

### 前提条件

- Rust 1.70以上
- Node.js 18以上
- npm または yarn

### インストール

1. リポジトリをクローン
```bash
git clone <repository-url>
cd clerica
```

2. 依存関係をインストール
```bash
# フロントエンド依存関係
npm install

# Rust依存関係（初回のみ）
cargo build
```

3. 開発サーバーを起動
```bash
# フロントエンド開発サーバー
npm run dev

# 別ターミナルでTauri開発サーバー
cargo tauri dev
```

## ビルド

### 開発ビルド
```bash
cargo build
npm run build
```

### 本番ビルド
```bash
cargo tauri build
```

## プロジェクト構造

```
clerica/
├── src-tauri/           # Rust バックエンド
│   ├── src/
│   │   ├── main.rs      # エントリーポイント
│   │   ├── database.rs  # データベース操作
│   │   ├── file_manager.rs # ファイル管理
│   │   ├── search.rs    # 検索機能
│   │   └── watcher.rs   # ファイル監視
│   ├── migrations/      # データベースマイグレーション
│   ├── Cargo.toml       # Rust依存関係
│   └── tauri.conf.json  # Tauri設定
├── src/                 # React フロントエンド
│   ├── App.tsx          # メインコンポーネント
│   ├── main.tsx         # エントリーポイント
│   └── index.css        # スタイル
├── package.json         # フロントエンド依存関係
├── vite.config.ts       # Vite設定
└── README.md
```

## データベーススキーマ

### テーブル構成

- **directories**: 管理対象ディレクトリ
- **files**: ファイル情報
- **tags**: タグ定義
- **file_tags**: ファイルとタグの関連（多対多）

### 主要なインデックス

- ファイル名検索用インデックス
- タグ検索用インデックス
- 更新日時ソート用インデックス

## 開発ガイドライン

### コード品質

- Rust: `cargo clippy` でコード品質を確認
- TypeScript: strict モードを有効に保つ
- データベース: 適切なインデックスとトランザクションを使用

### テスト

```bash
# Rust テスト
cargo test

# フロントエンドテスト（実装予定）
npm test
```

## ライセンス

MIT License - 詳細は [LICENSE](LICENSE) ファイルを参照してください。

## 貢献

1. このリポジトリをフォーク
2. 機能ブランチを作成 (`git checkout -b feature/amazing-feature`)
3. 変更をコミット (`git commit -m 'Add some amazing feature'`)
4. ブランチにプッシュ (`git push origin feature/amazing-feature`)
5. プルリクエストを作成
