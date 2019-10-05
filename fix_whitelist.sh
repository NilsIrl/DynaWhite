#!/usr/bin/env sh

jq 'map(.name="MC-162683")' < whitelist.json | sponge whitelist.json
