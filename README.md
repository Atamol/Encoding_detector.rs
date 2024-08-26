![License](https://img.shields.io/badge/license-MIT-green)<br>
![Python](https://img.shields.io/badge/python-3.x-blue)

# Encoding Detector

このスクリプトは，[`chardet`](https://pypi.org/project/chardet/)というライブラリを使用し，指定されたファイルのエンコーディングを検出することができます．<br>
Shift_JISやUTF-8が混合されてしまった場合や，単にエンコーディングが分からなくなってしまった場合に便利です．<br>
さまざまなケースに対応しており，明確なフィードバックを行うことができます．

## 機能・特徴

- **エンコーディング検出**: [`chardet`](https://pypi.org/project/chardet/)または`UniversalDetector`を使用してテキストファイルのエンコーディングを検出します．
- **信頼度表示**: クラスによって検出されたエンコーディングの信頼度を表示します．
- **言語検出**: 可能であればテキストの言語も検出します．
- **手軽で使いやすい**: 無効なファイルパスやその他の問題に対して，ユーザーフレンドリーなエラーメッセージを提供します。

## インストール

1. **`main.py`をダウンロード，もしくはリポジトリをクローン:**

2. **依存関係のインストール:**

    [`chardet`](https://pypi.org/project/chardet/)をインストールします．

    ```bash
    pip install chardet
    ```

## 使用方法

1. **スクリプトを実行:**

    ```bash
    python main.py
    ```

2. **ファイルパスの入力:**

    エンコーディングを検出したいファイルのパスを入力します．該当するファイルをターミナルにドラッグ＆ドロップして，パスを自動入力することができます．<br>
    無効なパスが入力された場合，もう一度パスの入力を求められます．

   ```bash
   File path?
   ```

4. **結果の確認:**

    検出されたエンコーディング，信頼度，言語を次の形式で表示します．

    ```text
    Using chardet.detect() for file size: 102400 bytes
    {Encoding: SHIFT_JIS, Confidence: 0.99, Language: Japanese}
    ```

    検出できなかった場合には次のように表示されます．

    ```text
    Using UniversalDetector() for file size: 1048576 bytes
    {Encoding: None, Confidence: 0.0, Language: N/A}
    ```

## `chardet`と`UniversalDetector`の説明

- **`chardet.detect()`:**
  - [`chardet`](https://pypi.org/project/chardet/)はファイルの全体を読み込み，そのエンコーディングを検出するライブラリです．小さなファイルに対しては迅速に処理が行われ，検出精度が高いのが特徴です．
  
- **`UniversalDetector`:**
  - `UniversalDetector`は，ファイルを逐次的に読み込みながらエンコーディングを検出し，ある程度の確証が持てた時点で処理を終了します．そのため大きなファイルに対しては効率的に処理を行えるため，[`chardet`](https://pypi.org/project/chardet/)よりも適していることがあります．

### `chardet`と`UniversalDetector`の切り替えについて

このスクリプトでは，ファイルサイズによって`chardet`と`UniversalDetector`を自動的に切り替えます．

  - **500KB未満のファイル:** `chardet.detect()` を使用． 
  - **500KB以上のファイル:** `UniversalDetector` を使用．

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
