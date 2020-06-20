# Rustaba

🦀 Rust + 🕸 Wasm + 🌱 Futaba = 🦞 Rustaba

## インストール

```sh
npm install
```

## デバッグする

```sh
# プロジェクトをデバッグモードでビルドし、新しいタブで開く（ホットリロード）。
npm run dev

# Wasmのみリリースモードでビルドして、残りのコードをデバッグします。
# デバッグモードは実行速度が非常に低速なので、通常はこちらを実行します。
npm start
```

## ビルドする

```sh
# プロジェクトをリリースモードでビルドして `dist` ディレクトリに配置する。
# また、テスト用のDokcerイメージをビルドする。
npm run build
```

## 各ファイルの説明

* `Cargo.toml` には Rust メタデータが含まれています。ここには Rust の依存関係を記述します。

* `package.json` には npm のメタデータが含まれています。JavaScript の依存関係をここに記述します。

* `webpack.config.js` には Webpack の設定が含まれています。

* `js` フォルダには JavaScript のコードが含まれています (`index.js` は Webpack にすべてを接続するために使用されているので、変更する必要はありません)。

* `src` フォルダには Rust のコードが格納されます。

* `static` フォルダには、最終ビルドにそのままコピーしたいファイルが入っています。また、`index.js` ファイルをロードする `index.html` ファイルが含まれています。
