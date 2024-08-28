![License](https://img.shields.io/badge/license-MIT-green)<br>
![Rust](https://img.shields.io/badge/rust-1.80.1-orange)

# Encoding Detector

このスクリプトは，[`chardetng`](https://docs.rs/chardetng/latest/chardetng/)というライブラリを使用し，指定されたファイルのエンコーディングを検出することができます．<br>
Shift_JISやUTF-8が混合されてしまった場合や，単にエンコーディングが分からなくなってしまった場合に便利です．<br>
さまざまなケースに対応しており，明確なフィードバックを行うことができます．

## 機能・特徴

- **エンコーディング検出**: [`chardetng`](https://docs.rs/chardetng/latest/chardetng/)または独自の`逐次的処理法`を使用してテキストファイルのエンコーディングを検出します．
- **言語検出**: 可能であればテキストの言語も検出します．
- **見やすいデザイン**: デザインが少し工夫されており，一目で視認しやすいです．
- **手軽で使いやすい**: 簡単なステップで実行を完了でき，無効なファイルパスやその他の問題に対してはユーザーフレンドリーなエラーメッセージを提供します．

## インストールおよび実行

   このスクリプトは追加の依存関係を必要としません．<br>
   リポジトリをクローンし，`cargo`を使用してビルドおよび実行するだけです．

1. **リポジトリをクローン:**

   ```bash
   git clone https://github.com/Atamol/Encoding_detector.rs.git
   ```

2. **ファイルパスの入力:**

   エンコーディングを検出したいファイルのパスを入力します．該当するファイルをターミナルにドラッグ＆ドロップして，パスを自動入力することができます．<br>
   無効なパスが入力された場合，もう一度パスの入力を求められます．

   ```bash
   File path?
   ```

3. **結果の確認:**

    検出されたエンコーディングなどは次のような形式で表示されます．

    ```text
    Using chardetng for file size: 102400 bytes
    {Encoding: "SHIFT_JIS", File size: "1000 KB"}
    ```

## `chardetng`と逐次的処理法の説明

- **`chardetng()`:**
  - [`chardetng`](https://docs.rs/chardetng/latest/chardetng/)はファイルの全体を読み込み，そのエンコーディングを検出するライブラリです．小さなファイルに対しては迅速に処理が行われ，検出精度が高いのが特徴です．
  
- **`逐次的処理法`:**
  - もう一方の`逐次的処理法`は，ファイルを逐次的に読み込みながらエンコーディングを検出し，ある程度の確証が持てた時点で処理を終了します．そのため大きなファイルに対しては効率的に処理を行うことができ，[`chardetng`](https://docs.rs/chardetng/latest/chardetng/)よりも適しています．

### ファイルのサイズによる処理方法の切り替え

このスクリプトでは，指定されたファイルのサイズによって処理の方法を自動で変更します．

  - **5MB未満のファイル:** ファイルのすべてを読み込む． 
  - **5MB以上のファイル:** 逐次的な読み込みにより処理を最適化．

これにより，小さなファイルに対しては迅速で高精度な検出が行われ，大きなファイルに対しては効率的な処理が行われるようになっています．

## エラーハンドリング

- **無効なファイルパス**: ファイルパスが無効な場合，スクリプトは有効なパスを再入力するよう促します．

  ```bash
  File path? hogehoge.com
  Error: Invalid file path or access issue. Please try again.
  File path?
  ```

- **ユーザーの中断**: `Ctrl + C`を使用してスクリプトを中断すると，どのタイミングでもメッセージを表示して適切に終了することができます．
  
  ```bash
  File path? # Ctrl + C を入力
  Process interrupted by user. Exiting...
  C:\Users\Alice>
  ```
