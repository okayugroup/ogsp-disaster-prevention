# OGSP Disaster Prevention

## 紹介
OGSP Disaster Preventionは、[おかゆグループ地震計プロジェクト (OGSP)](https://ogsp.okayugroup.com)をはじめとした地震計のデータを利用した、災害対策を目的としたデスクトップアプリケーションです。

> [!WARNING]
> このプロジェクトは現在開発中です。機能が不完全である可能性があります。

## 機能
<details>
<summary>
現在開発中の機能
</summary>

- OGSPから地震計のデータを取得、リアルタイム表示
- 緊急地震速報の受信と表示
- 地震情報の受信と表示
- 震央分布図の3D表示
- 地図上に災害データを表示します。
</details>

## ドキュメント
このプロジェクトでは、ユーザー向けと開発者向けのドキュメントを提供しています。
詳しくは[Docs (README.md)](docs/README.md)をご覧ください。

それぞれのドキュメントへのアクセス：
- [ユーザー向け](docs/user/_index.md)
- [開発者向け](docs/develop/_index.md)

ぜひ併せてご利用ください。

## 開発
### 言語・フレームワーク
- Go ([バックエンド](https://github.com/yossy4411/ogsp-server))
- Rust (メイン処理部)
- Iced (GUI)

### 環境
```bash
$ rustc --version
rustc 1.84.0 (9fc6b4312 2025-01-07)
```
- RustRover (JetBrains IDE)
- Windows11

### ビルド
```bash
$ cargo build --release
```

### インストール
現時点でリリースはありません。ビルドするか、リリースをお待ち下さい。