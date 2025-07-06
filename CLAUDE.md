# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Clericaは、Mac向けファイル整理・検索ツールです。複数のディレクトリにまたがる大量のファイルを一元管理し、柔軟なタグ分類と高速検索を提供するTauriベースのデスクトップアプリケーションです。

## Architecture

### Frontend (SvelteKit + TypeScript)
- **エントリーポイント**: `src/app.html`
- **メインコンポーネント**: `src/routes/+page.svelte` - ディレクトリ管理、ファイル表示、検索、タグ管理の機能を統合
- **スタイル**: `src/lib/App.css`, `src/index.css`
- **ビルドツール**: Vite（固定ポート1420使用）

### Backend (Rust + Tauri)
- **エントリーポイント**: `src-tauri/src/main.rs`
- **モジュール構成**:
  - `src-tauri/src/database/mod.rs` - SQLiteデータベース操作とマイグレーション
  - `src-tauri/src/file_manager/mod.rs` - ディレクトリ/ファイル管理、CRUD操作
  - `src-tauri/src/search/mod.rs` - 検索機能、タグ管理
  - `src-tauri/src/watcher.rs` - リアルタイムファイル監視

### Database (SQLite + SQLx)
- **データベースファイル**: `src-tauri/clerica.db`
- **マイグレーション**: `src-tauri/migrations/` ディレクトリで管理
- **主要テーブル**: directories, files, tags, file_tags（多対多関連）

## Common Development Commands

### Development Server
```bash
npm run dev  # フロントエンド開発サーバー起動
```

### Build Commands
```bash
# フロントエンドビルド（TypeScript型チェック含む）
npm run build

# Rustバックエンドビルド
cd src-tauri && cargo build

# 統合ビルド（本番）
npm run tauri build
```

### Code Quality
```bash
# Rust静的解析
cd src-tauri && cargo clippy

# Rustテスト
cd src-tauri && cargo llvm-cov
```

### Testing
```bash
# ユニットテスト実行
npm run test

# E2Eテスト実行
npm run test:e2e

# E2EテストUI実行
npm run test:e2e:ui

# E2Eテストデバッグ実行
npm run test:e2e:debug
```

## Key Technical Details

### Tauri Command Architecture
- フロントエンドから`invoke()`でRust関数を呼び出し
- 主要なコマンド: `add_directory`, `get_files`, `search_files`, `update_file_tags`, `start_watching`
- 全てのTauriコマンドは`src-tauri/src/main.rs`で登録

### Frontend architecture
- MVVMアーキテクチャを採用
- Rust側の呼び出しはlib/api以下に集約

### Frontend State Management
- SvelteKitの$state等でローカル状態管理
- Tauri APIを直接呼び出してバックエンドと通信
- 検索結果はFuse.jsを使用してファジー検索を実装

### Database Schema
- `directories`: 管理対象ディレクトリ情報
- `files`: ファイルメタデータ（パス、サイズ、作成日時等）
- `tags`: タグ定義（名前、色）
- `file_tags`: ファイルとタグの多対多関連

### File Watching
- notify-rsライブラリでファイルシステム監視
- リアルタイムでファイル変更を追跡してDBに反映

## Development Guidelines

### 必須チェック
変更後は必ず以下のコマンドを実行してビルドエラーがないことを確認:
1. `cd src-tauri && cargo build` - Rustコンパイル確認
2. `npm run build` - TypeScriptコンパイル確認
3. `npm run tauri build` - 統合ビルド確認

### Code Quality
- TypeScript strict モード有効
- Rustは`cargo clippy`でコード品質確認
- SQLクエリの安全性確保（SQLx使用）
- 適切なエラーハンドリング実装
- テストのカバレッジは90%以上を目指す

### Performance Considerations
- ファイル監視機能のパフォーマンスに注意
- SQLiteクエリの最適化（適切なインデックス使用）
- 大量ファイル処理時のメモリ使用量管理