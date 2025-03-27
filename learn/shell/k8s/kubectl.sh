#!/usr/bin/bash

kubectl -v 9
kubectl version
kubectl api-versions
kubectl api-resources --cached -o output
kubectl create -f filename
kubectl patch type name -p patch
kubectl delete type name -Awl selector
kubectl get type name -Awl selector -o output
kubectl logs pod -f
kubectl attach pod -it
kubectl exec pod -it -- command
kubectl port-forward pod local-port:remote-port
kubectl cp file-spec-src file-spec-dest
kubectl proxy -p port
