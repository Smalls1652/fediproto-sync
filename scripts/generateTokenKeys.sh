#!/bin/sh

PRIVATE_KEY_PATH="${TMPDIR}fediproto-private_key.pem"
PUBLIC_KEY_PATH="${TMPDIR}fediproto-public_key.pem"

openssl genrsa -out $PRIVATE_KEY_PATH 4096 > /dev/null 2>&1
BASE64_PRIVATE_KEY=$(cat $PRIVATE_KEY_PATH | base64 -w 0)

openssl rsa -in $PRIVATE_KEY_PATH -pubout -out $PUBLIC_KEY_PATH > /dev/null 2>&1
BASE64_PUBLIC_KEY=$(cat $PUBLIC_KEY_PATH | base64 -w 0)

rm -f $PRIVATE_KEY_PATH
rm -f $PUBLIC_KEY_PATH

echo "TOKEN_ENCRYPTION_PRIVATE_KEY=\"${BASE64_PRIVATE_KEY}\""
echo ""
echo "TOKEN_ENCRYPTION_PUBLIC_KEY=\"${BASE64_PUBLIC_KEY}\""
