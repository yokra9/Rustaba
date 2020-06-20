FROM httpd:alpine

# MIMETYPEにWASMを追加
COPY mime.types /usr/local/apache2/conf/

# デプロイ
COPY dist/ /usr/local/apache2/htdocs/