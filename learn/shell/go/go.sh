#!/usr/bin/bash

go install package
go test -c -- -test.list regexp -test.run regexp -test.v -test.timeout d
go build -race -buildmode mode
go mod init
go mod tidy
