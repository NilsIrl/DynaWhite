
# DynaWhite

A
[Bukkit](https://www.spigotmc.org/wiki/what-is-spigot-craftbukkit-bukkit-vanilla-forg/)
plugin that allows to whitelist people based on identity verification written
in Rust.

## Setup

1. Run `mvn package` to generate the `.jar` file for the plugin.
2. Run `cargo build --release` to generate the dynamic library for the plugin.

Both files will be found in `/target/`.

`fix_whitelist.sh` MAY be run called from `start.sh`. It ensures the whitelist
has a correct format and is loaded when the server restarts.

The `LD_LIBRARY_PATH` or `java.library.path` should be set to the path of the
directory containing the dynamic library.

Environment variables for the http server:

* `SUPPORT_EMAIL_ADDRESS` - The address that a staff should send an email to in
  order to be whitelisted. (The program will not send emails to staff members.)
* `WEBSITE_URL`
* `MC_SERVER_ADDR`
* `SMTP_ADDR`
* `SMTP_PORT`
* `SMTP_USERNAME`
* `SMTP_PASSWORD`
* `VALID_DOMAIN` - The domain that email should belong to in order to be
  whitelisted

The `.jar` should be put inside the `plugins/` directory of your server.

### Optional

* You probably want to set `white-list` to `true` in `server.properties`.
* And set a custom whitelist message in `spigot.yml`.

## Dependencies

* `jq`
* rust nightly
* java
