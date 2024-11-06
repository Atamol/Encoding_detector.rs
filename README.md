![License](https://img.shields.io/badge/license-MIT-green)<br>
![Rust](https://img.shields.io/badge/Rust-1.80.1-orange?style=flat-square&logo=rust)

# Encoding Detector

    ※2014年前後に作成したものの更新版保存用です

[`chardetng`](https://docs.rs/chardetng/latest/chardetng/)を使用し，指定されたファイルのエンコーディングを検出する．  

## インストールおよび実行

追加の依存関係を必要としないため，リポジトリをクローンし，`cargo`を使用してビルドおよび実行するだけで良い．

1. **リポジトリをクローン:**

   ```bash
   git clone https://github.com/Atamol/Encoding_detector.rs.git
   ```

2. **ファイルパスの入力:**

   エンコーディングを検出したいファイルのパスを入力します．該当するファイルをターミナルにドラッグ＆ドロップして，パスを自動入力することができる．  
   無効なパスが入力された場合，もう一度パスの入力を求められる．

   ```bash
   File path?
   ```

3. **結果の確認:**

    検出されたエンコーディングなどは次のような形式で表示される．

    ```text
    Using chardetng for file size: 102400 bytes
    {Encoding: "SHIFT_JIS", File size: "1000 KB"}
    ```

## `chardetng`と逐次的処理法の説明

- **`chardetng()`:**
  - [`chardetng`](https://docs.rs/chardetng/latest/chardetng/)はファイルの全体を読み込み，そのエンコーディングを検出するライブラリ．小さなファイルに対しては迅速に処理が行われ，検出精度が高い．
  - 5MB未満のファイルに対して自動でこの方法が選択される．
  
- **`逐次的処理法`:**
  - もう一方の`逐次的処理法`は，ファイルを逐次的に読み込みながらエンコーディングを検出しある程度の確証が持てた時点で処理を終了ため，大きなファイルに対して効率的に処理を行うことができ，[`chardetng`](https://docs.rs/chardetng/latest/chardetng/)よりも適している場合がある．
  - 5MB以上のファイルに対して自動でこの方法が選択される．

## エラーハンドリング

- **無効なファイルパス**: ファイルパスが無効な場合，スクリプトは有効なパスを再入力するよう促す．

  ```bash
  File path? hogehoge.com
  Error: Invalid file path or access issue. Please try again.
  File path?
  ```

- **ユーザーの中断**: `Ctrl + C`を使用してスクリプトを中断すると，どのタイミングでもメッセージを表示して適切に終了することができる．
  
  ```bash
  File path? # Ctrl + C を入力
  Process interrupted by user. Exiting...
  C:\Users\Alice>
  ```
