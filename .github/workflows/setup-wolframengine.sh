#!/bin/bash

wget https://account.wolfram.com/download/public/wolfram-engine/desktop/LINUX
sudo bash LINUX -- -auto -verbose
rm LINUX

/usr/bin/wolframscript -authenticate $WOLFRAM_ID $WOLFRAM_PW
/usr/bin/wolframscript -activate
