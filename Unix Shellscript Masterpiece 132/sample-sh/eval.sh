#!/bin/sh

filename="myapp.log"
eval `sed -n "s/<code>\(.*\)<\/code>/\1/p" command.htm`
