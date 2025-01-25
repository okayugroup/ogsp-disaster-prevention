# wgpuの魅力

ここでは、[wgpu](https://github.com/gfx-rs/wgpu)の特徴や使い方について解説します。

## wgpuとは

wgpuは、Pure RustのグラフィックスAPIです。このライブラリは、WebGPUの仕様に基づいて設計されています。

### WebGPUとは

WebGPUは、Webブラウザ向けのグラフィックスAPIで、WebGLの後継として開発されています。

低レベルなため、高速なレンダリングが可能となっています。


## wgpuの特徴

- Rustのメモリ安全性やパフォーマンスを活かした設計
- 高速なレンダリング
- WebGPUの仕様に基づいた設計
- 3Dのサポート
- GPU上で動作
- Web上での動作

## wgpuの使い方

wgpuを使うには、まず`wgpu`クレートをCargo.tomlに追加します。

```toml
[dependencies]
wgpu = "0.10"  # 最新バージョンを指定してね
```

次に、`wgpu`をインポートして使います。

```rust
use wgpu::*;
```

これで、wgpuを使う準備が整いました。

## まとめ

wgpuは、Rustの特徴である安全性やパフォーマンスを活かしたグラフィックスAPIです。

wgpuを使用することで、高速で安全なアプリやゲームを作ることが出来ます！

みんなも使ってみてね！