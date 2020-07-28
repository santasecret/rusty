#!/bin/sh

touch sample.txt

if [ -z "$S3_ENDPOINT_URL" ]; then
  aws s3 cp /cache/x86_64-unknown-linux-musl/release/rusty s3://rusty/builds/
else
  aws --endpoint-url "$S3_ENDPOINT_URL" s3 cp /cache/x86_64-unknown-linux-musl/release/rusty s3://rusty/builds/
fi

