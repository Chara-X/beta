#!/usr/bin/bash

kubectl apply -f filename
kubectl edit type name -n namespace
kubectl delete type name -n namespace -l selector        # DELETE /api/{group}/{version}/namespaces/{namespace}/{type}/{name}?labelSelector={selector}
kubectl get type name -n namespace -l selector -Awo yaml # GET /api/{group}/{version}/namespaces/{namespace}/{type}/{name}?labelSelector={selector}&watch={watch}
kubectl logs name -n namespace -f                        # GET /api/v1/namespaces/{namespace}/pods/{name}/log?follow={follow}
kubectl exec name -n namespace -it -- command args       # GET /api/v1/namespaces/{namespace}/pods/{name}/exec?command={command}&stdin=true&stdout=true&stderr=true
kubectl cp file-spec-src file-spec-dest -An namespace
kubectl proxy -p port
kubectl -v 9
