#!/usr/bin/bash

oapi-codegen -package string -o string -generate client,server,models openapi.json
