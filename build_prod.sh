#!/bin/bash

echo "build aries for production..."

docker build -f ./prod.Dockerfile -t aries_prod . --no-cache
