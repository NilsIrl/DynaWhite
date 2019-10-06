#!/usr/bin/env sh

WHITELIST="whitelist.json"
out=$(jq 'map(.name="MC-162683")' < "$WHITELIST") && echo "$out" > $WHITELIST
