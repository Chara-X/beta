#!/usr/bin/bash

go install package
go test -run func -coverprofile file -v
go build -race -buildmode mode
go mod init
go mod tidy
go tool cover -html file
