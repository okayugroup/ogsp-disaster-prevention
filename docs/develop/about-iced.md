# Icedとは

このプロジェクト「OGSP Disaster Prevention」では、GUIフレームワークとして[Iced](https://github.com/hecrj/iced)を使用しています。

Icedは、Rustで書かれたGUIライブラリです。このライブラリは、モダンで高速なアプリケーションを構築するために設計されています。

## バージョンについて
このプロジェクトでは、Icedのバージョン0.13.1を使用しています。

Icedは、頻繁にアップデートされるため、最新のバージョンを使用することをお勧めします。

ただし、Icedでは破壊的変更がよく行われるため、バージョンアップが困難な場合があります。

たとえば、0.8から0.10へアップデートされたときに、Applicationが廃止されたことがあります。

とにかく、Icedのバージョンアップは慎重に行う必要があります。

また、QiitaなどのコミュニティにもIcedに関する情報が多くありますが、バージョンの関係上互換性がない場合があるので、注意が必要です。

## Icedの特徴

Icedは、以下のような特徴を持っています。

- モダンなデザイン
- [wgpu](https://github.com/gfx-rs/wgpu)を用いた高速なレンダリング
- マルチプラットフォーム対応 (Windows, macOS, Linux, WASM)
- テーマのカスタマイズ
- 宣言型のUI構築
- マルチスレッド対応

また、Rustの特徴である安全性やパフォーマンスもそのまま活かされています。

## wgpuについて

Icedは、[wgpu](https://github.com/gfx-rs/wgpu)を用いて高速なレンダリングを実現しています。

wgpuは、Rustで書かれたPure RustのグラフィックスAPIです。このライブラリは、WebGPUの仕様に基づいて設計されています。

しかも、2Dに加えて3Dもサポートしているため、Icedは、高速な2D/3Dアプリケーションを構築するための最適なライブラリと言えるでしょう。

「[wgpuの魅力](wgpu.md)」には、wgpuの特徴や使い方について詳しく解説していますので、ぜひご覧ください。

## サンプル

Icedのサンプルコードは、[iced](https://github.com/hecrj/iced)のリポジトリに多数用意されています。それらを見るのが一番いいでしょう。

ここでは、README.mdに記載されているサンプルコードを一部紹介します。

```rust
use iced::widget::{button, column, text, Column};

#[derive(Default)]
struct Counter {
    value: i32,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Increment,
    Decrement,
}

impl Counter {
    pub fn view(&self) -> Column<Message> {
        // We use a column: a simple vertical layout
        column![
            // The increment button. We tell it to produce an
            // `Increment` message when pressed
            button("+").on_press(Message::Increment),

            // We show the value of the counter here
            text(self.value).size(50),

            // The decrement button. We tell it to produce a
            // `Decrement` message when pressed
            button("-").on_press(Message::Decrement),
        ]
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Increment => {
                self.value += 1;
            }
            Message::Decrement => {
                self.value -= 1;
            }
        }
    }
}

fn main() -> iced::Result {
    iced::run("A cool counter", Counter::update, Counter::view)
}
```

※Icedの仕様変更により、最新バージョンでは動作しない可能性があります。

このコードは、カウンターアプリを作成するサンプルです。ボタンを押すとカウンターの値が増減します。

MVU(Model-View-Update)アーキテクチャを採用しているため、状態とビューが分離されています。FlutterやReactにも似ていますね！

## よくある問題

Icedは、まだ開発中のため、ドキュメントも整っておらず、いくつかの問題が知られています。

- [日本語フォントの問題(解決策あり)](./iced-custom-font.md)