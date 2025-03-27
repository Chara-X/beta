#!/usr/bin/bash

go install package
go test -test.list regexp -test.run regexp -test.parallel n -test.timeout d -test.coverprofile file -test.v -c -args args
go build -race -buildmode mode
go mod init
go mod tidy
go tool cover -html file
