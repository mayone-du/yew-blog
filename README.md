# Yew Blog

Rust で Web フロントを書いたブログです。  
SSG などはせず、GitHub の別リポジトリのマークダウンを読んで CSR してます。

### Note

1. Tailwind CSS のビルド

```
npm run build:talwindcss
```

2. ローカルサーバーの起動

```
trunk serve
```

3. GitHub でトークンを発行し、.env に `GITHUB_API_TOKEN` というキーで保存

https://github.com/settings/tokens

<!-- GraphQLを使う場合↓ -->
<!-- download the schema.docs.graphql from github.
https://docs.github.com/ja/graphql/overview/public-schema -->
