# 全体像

```mermaid
classDiagram
    ProviderContainer *-- ProviderPointerManager : 状態を管理
    ProviderContainer ..> Provider : read / build
    ProviderPointerManager *-- ProviderPointer : 状態へのポインタ
    ProviderPointer *-- ErasedElement : 型消去して保持
    ProviderElement ..|> ErasedElement : 実装
    ProviderElement --> Provider : 対応する状態
    Provider --> Ref : factory に渡す
    Ref ..> Provider : 依存先を読む
    Ref *-- BuildContext : build 情報
    BuildContext --> ProviderContainer : container を参照
    BuildContext --> BuildFrame : 現在の build frame
    BuildFrame --> BuildFrame : 親 frame
```

`Provider` が状態の作り方を定義し、`ProviderContainer` が状態を保持します。`ProviderPointer` は `ErasedElement` trait 経由で状態を保持し、実体の `ProviderElement` がその trait を実装します。Provider の生成中は `Ref` と `BuildContext` を使い、`BuildFrame` で依存のたどり方を管理します。
