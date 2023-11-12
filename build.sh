#!/bin/bash
version=2.22.10
docker build . --network=host -t webhook_gateway:$version
sleep 1
if [[ $? -ne 0 ]]; then
    echo "Build failed"
    exit 1
fi
if [[ -z $VIEW ]]; then
    kubectl set image  deployment/webhook-gateway webhook-gateway=webhook_gateway:$version
else
    $VIEW/kubectl set image  deployment/webhook-gateway webhook-gateway=webhook_gateway:$version
fi
