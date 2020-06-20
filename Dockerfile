FROM httpd:alpine

# MIMETYPEにWASMを追加
COPY mime.types /usr/local/apache2/conf/
# 開発用にリバプロ設定を追加
COPY httpd.conf /usr/local/apache2/conf/

# デプロイ
COPY dist/ /usr/local/apache2/htdocs/