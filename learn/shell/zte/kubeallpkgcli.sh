#!/usr/bin/env bash

kubeall-pkg-cli release -b branch -n chartName -v version -d fileDir
kubeall-pkg-cli get -b branch -n chartName -v version -d fileDir
