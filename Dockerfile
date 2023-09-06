# 声明代理变量
ARG HTTP_PROXY
ARG HTTPS_PROXY
# npm编译
FROM node:16.17.0 AS npm-build
WORKDIR /app
COPY terraria_web .
RUN npm config set registry http://mirrors.cloud.tencent.com/npm/ && npm install && npm run build
# Rust编译
FROM rust:1.71 AS rust-build
ENV http_proxy=$HTTP_PROXY \
    https_proxy=$HTTPS_PROXY
WORKDIR /usr/src/rust-app
COPY terraria_server .
RUN RUSTFLAGS='-C target-feature=+crt-static' cargo build --release --target=x86_64-unknown-linux-gnu
# 打包运行
FROM nginx:stable
WORKDIR /app
# 替换为自己的泰拉瑞亚服务
COPY server /app/server
# 拷贝必要数据
COPY default.conf /etc/nginx/conf.d/default.conf
COPY --from=npm-build /app/dist /usr/share/nginx/html/
COPY --from=rust-build /usr/src/rust-app/target/x86_64-unknown-linux-gnu/release/terriaira_server /app/
# 安装supervisord
RUN sed -i s@/deb.debian.org/@/mirrors.aliyun.com/@g /etc/apt/sources.list && sed -i s@/security.debian.org/@/mirrors.aliyun.com/@g /etc/apt/sources.list &&  \
    apt-get update && apt-get install -y supervisor --fix-missing
# 创建supervisor配置文件
RUN echo "[supervisord]\nnodaemon=true\n\n\
[program:terriaira]\ncommand=/app/terriaira_server\n\n\
[program:nginx]\ncommand=nginx -g 'daemon off;'\n" > /etc/supervisor/conf.d/supervisord.conf
EXPOSE 3000
# 设置启动命
CMD ["/usr/bin/supervisord"]


