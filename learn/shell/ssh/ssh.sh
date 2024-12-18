#!/usr/bin/bash

ssh destination -p port -L port:host:hostport -R port:host:hostport -vvv
