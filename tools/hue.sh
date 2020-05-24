#!/bin/bash

h=$1
s=$2
b=$3

echo Parsed_hue_1 h $h | zmqsend
echo Parsed_hue_1 s $s | zmqsend
echo Parsed_hue_1 b $b | zmqsend
