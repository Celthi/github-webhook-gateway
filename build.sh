#!/bin/bash
version=2.22.0
cargo check && docker build . --network=host -t webhook_gateway:$version
sleep 1
kubectl set image  deployment/webhook-gateway webhook-gateway=webhook_gateway:$version
