## Chauri
ChauriはTauri + Vue + Typescriptで作られたChatGPT用GUIクライアントです。

### 特徴
- プライバシー
  - あなたのデータはOpenAI以外のどこにも送信しません。全てのデータはあなたのストレージの中にあります。デバッグログを送信することもありません。

### 使い方
1. chauri用のデータディレクトリを作成してください。

```
$ mkdir $USER/.chauri
```

2. `$USER/.chauri/.env`ファイル内に以下の内容を記述してください。

```
OPENAI_API_KEY=[your OpenAI API Key]
DEFAULT_MODEL=gpt-3.5-turbo
```

3. Chauriを使い始めます！