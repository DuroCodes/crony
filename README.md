# Crony

<div align="center">
  <img src="./logo.png" width="400" />
</div>

Crony is a simple and powerful command line cron job manager and scheduler written in Rust. It allows you to easily manage your scheduled tasks with human-readable schedule syntax.

## Installation

You can install Crony via cargo with `cargo install crony-cli` to install the CLI into your system's PATH. (Make sure you have Rust and Cargo installed on your system.)

## Usage

To use Crony, you can run `crony` in your terminal. This will start the CLI, where you can add, remove, and list scheduled tasks. You can also use `crony run` to start the scheduler, which will execute your tasks based on their defined schedules.

> [!NOTE]
> Tasks that produce output will print it to the terminal output of the `crony run` command.
> If you want to retrieve the output of a task, you should redirect it to a file.

## Configuration

You can also edit tasks via the `tasks.toml` file under `~/.config/crony/` (Unix) or `%APPDATA%\crony\` (Windows). This file contains all your scheduled tasks, in a format like this:

```toml
[tasks.hello]
name = "hello"
schedule = "every 1 min"
command = "echo 'hi!'"
```

## Running Crony as a Service

You can add `crony run` to your system's service manager to run it as a background service. This way, your tasks will be executed automatically based on their schedules.

### Linux (systemd)

You can create a systemd service file for Crony. Create a file named `crony.service` in `/etc/systemd/system/` with the following content:

```ini
[Unit]
Description=Crony Scheduler Service
After=network.target
[Service]
ExecStart=/usr/local/bin/crony run
Restart=always
[Install]
WantedBy=multi-user.target
```

After creating the service file, you can enable and start the service with the following commands:

```bash
sudo systemctl enable crony.service
sudo systemctl start crony.service
```

### macOS (launchd)

You can create a launchd plist file for Crony. Create a file named `me.durocodes.crony.plist` in `~/Library/LaunchAgents/` with the following content:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>me.durocodes.crony</string>
    <key>ProgramArguments</key>
    <array>
        <string>/usr/local/bin/crony</string>
        <string>run</string>
    </array>
    <key>RunAtLoad</key>
    <true/>
    <key>KeepAlive</key>
    <true/>
</dict>
</plist>
```

After creating the plist file, you can load it with the following command:

```bash
launchctl load ~/Library/LaunchAgents/me.durocodes.crony.plist
```

## Windows (NSSM)

> [!NOTE]
> You can also use the Windows Task Scheduler, but it requires more setup and is less straightforward than just using NSSM.

You can use NSSM (Non-Sucking Service Manager) to run Crony as a service on Windows. First, download and install NSSM from [nssm.cc](https://nssm.cc/).

Then, you can create a service for Crony with the following command:

```bash
nssm install Crony "C:\path\to\crony.exe" run
```

After creating the service, you can start it with the following command:

```bash
nssm start Crony
```


