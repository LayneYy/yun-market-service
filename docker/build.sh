#!/bin/zsh

cd ..

cross build --target x86_64-unknown-linux-musl --release

cd - || exit

tag=$1

cp ../target/x86_64-unknown-linux-musl/release/yun-market-service .

docker build . -t registry.cn-hangzhou.aliyuncs.com/repayment/market:"$tag"

rm -f yun-market-service

docker push registry.cn-hangzhou.aliyuncs.com/repayment/market:"$tag"