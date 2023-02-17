# ⚒️ handyman

handyman is a *blazingly fast* health check and automation tool for your infrastructure. Built as an (optional) systemd service, It allows you to configure health checks for your infrastructure and automate actions based on the results of those health checks.

For instance, you can configure handyman to check if a service is running, and if it's not, handyman can automatically restart it for you. This was built for my personal use to automatically restart docker containers when a HTTP check fails.

handyman is written in Rust and is designed to be as fast as possible. It's also designed to be as simple as possible, so you can easily configure it to do what you want.

## Configuration

handyman is configured using `.conf` files, with a TOML syntax. You can find an example configuration file in the `example` directory.

Example of a configuration file:

```toml
# Automically restart the docker container if the HTTP check fails
name = "example"
interval = 5

[[http]]
name = "web server"
url = "http://localhost:8080"
timeout = 5

[[handlers]]
name = "restart web server docker container"
state = "failed"
command = "docker restart example"
```

Simply chuck this into `/etc/handyman.d/example.conf` and you're good to go!

handyman will automatically pick up the configuration file and start running the health check.

## Why not a cron job?

handyman has many advantages over cron jobs:

- Handyman is written in Rust, which is blazingly fast. Handyman is multithreaded and can run hundreds of health checks in a second.
- Handyman is designed to be as simple as possible, so you can easily configure it to do what you want.
- Handyman handles the common instrastrucure of running health checks
- Handyman is designed to be run as a systemd service, so it's easy to manage and monitor.
- Handyman defines your healthchecks in a configuration file(s), so you can easily see what's being checked and what's being done when a health check fails.
- Handyman provides logging about health checks and their commands, so you can easily see what's going on.