#!/usr/bin/env bash

helm repo index dir --merge index
helm repo add repo url
helm repo update repo
helm repo remove repo
helm repo list
helm search repo keyword
helm install release chart -n namespace -f values --dry-run
helm upgrade release chart -n namespace -f values --dry-run
helm uninstall release -n namespace --dry-run
helm list -n namespace -A
helm env
