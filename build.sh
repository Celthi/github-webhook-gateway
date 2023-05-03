#!/bin/bash
version=2.20.1
cargo check && docker build . -t webhook_gateway:$version
sleep 1
kubectl set image  deployment/webhook-gateway webhook-gateway=webhook_gateway:$version
