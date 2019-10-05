#!/usr/bin/env sh

javac -classpath spigot.jar re/nilsand/opgsmc/whitelist/App.java &&
jar cf DynaWhite.jar re/nilsand/opgsmc/whitelist/App.class plugin.yml &&
(cd http-server/; cargo build --release)

