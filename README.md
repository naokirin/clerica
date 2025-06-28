# Clerica - Electronアプリケーション

Clericaは、モダンなUIを持つElectronデスクトップアプリケーションです。

## 機能
TBD

## セットアップ

### 前提条件

- Node.js (v16以上)
- npm または yarn

### インストール

1. 依存関係をインストール:
```bash
npm install
```

2. アプリケーションを起動:
```bash
npm start
```

3. 開発モードで起動（DevTools付き）:
```bash
npm run dev
```

## ビルド

### 配布用パッケージの作成

```bash
npm run build
```

### 実行可能ファイルの作成

```bash
npm run dist
```

## プロジェクト構造

```
clerica/
├── main.js              # メインプロセス
├── preload.js           # プリロードスクリプト
├── package.json         # プロジェクト設定
├── renderer/            # レンダラープロセス
│   ├── index.html       # メインHTML
│   ├── styles.css       # スタイルシート
│   └── renderer.js      # レンダラーJavaScript
└── README.md           # このファイル
```

## キーボードショートカット

- `Cmd/Ctrl + Q`: アプリケーションを終了
- `Cmd/Ctrl + R`: ページをリロード

## 技術スタック

- **Electron**: デスクトップアプリケーションフレームワーク
- **HTML5/CSS3**: モダンなUI
- **Vanilla JavaScript**: 軽量な実装
- **Electron Builder**: パッケージングツール

## セキュリティ

- Context Isolation を有効化
- Node Integration を無効化
- プリロードスクリプトによる安全なAPI公開

## ライセンス

MIT License

## 開発

### 開発モード

開発モードでは、DevToolsが自動的に開き、デバッグが容易になります。
