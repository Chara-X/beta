#!/usr/bin/env bash

minikube start
minikube ssh
minikube stop
minikube delete
minikube addons enable addon-name
minikube addons disable addon-name
minikube addons list
