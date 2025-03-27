#!/usr/bin/bash

argo submit file -n namespace               # kubectl apply -f file -n namespace
argo delete workflow -n namespace           # kubectl delete wf workflow -n namespace
argo list -n namespace -l selector -Ao yaml # kubectl get wf -n namespace -l selector -Ao yaml
argo get workflow -n namespace              # kubectl get wf workflow -n namespace
argo logs workflow -n namespace             # kubectl logs workflow -n namespace
