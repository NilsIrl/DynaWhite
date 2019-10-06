
# DynaWhite

A bukkit/spigot/paper plugin that allows to whitelist people based on identity
verification written in Rust

## Setup

Run `build.sh` to generate the appropriate files.

`build.rs` expects `spigot.jar` to be present in `PWD`.

`fix_whitelist.sh` MAY be run called from `start.sh`. It ensures the whitelist
has a correct format and is loaded when the server restarts.

The `LD_LIBRARY_PATH` environment variable should be set to the path of the
directory containing `libhttp_server.so`. `libhttp_server.so` is found in
`http-server/target/(debug|release)/libhttp_server.so`

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

`DynaWhite.jar` should be put inside the `plugins/` directory of your server.

### Optional

* You probably want to set `white-list` to `true` in `server.properties`.
* And set a custom whitelist message in `spigot.yml`.

## Dependencies

* `jq`
* rust nightly
* java
