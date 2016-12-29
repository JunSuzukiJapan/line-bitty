# line-botty

Framework of LINE Messaging API Application with AWS Lambda and TypeScript.

# Requirements

- make
- nodejs
- gulp

# Install

## Rust & Cargo のインストール

Linux & Mac

```
$ curl https://sh.rustup.rs -sSf | sh
```

## line_botty のインストール

```
$ cargo install line_botty
```

# Usage

## プロジェクトの作成

```
$ line-botty create --name your_project_name
```

LINE Messaging API のChannel Access Tokenを訊かれますので、入力してください。
(--tokenオプションで指定することも可能。)

## プロジェクトのビルド

```
$ line-botty build
```

## npmパッケージの追加＆削除

line_bottyでnpmパッケージの管理が可能です。

npmパッケージをアプリで使用するには、src/ディレクトリで```npm install```を実行する必要があるのですが、line_bottyで管理することにより、ディレクトリの移動をすることなく管理できます。

### 追加

```
$ line_botty npm install パッケージ名
```

### 削除

```
$ line_botty npm uninstall パッケージ名
```

## serverless コマンドの実行

```
$ line_botty sls コマンド引数
```

slsコマンドにコマンド引数以下を渡して実行します。


# License

[MIT](License)