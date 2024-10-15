# Simple Raspberry Status Page in the Rust

## Snippets

### Dev commands
```sh
cargo update
cargo run
```

### Release commands
- Compile your Rust project into a binary:
```bash
cargo build --release
```

- Move the binary to a directory like `/usr/local/bin`:
```bash
sudo cp target/release/status-page /usr/local/bin/
```

- Create a `systemd` service:
```bash
sudo vim /etc/systemd/system/status-page.service
```

With the following content:
```ini
[Unit]
Description=Raspberry Pi Status Page Service
After=network.target

[Service]
ExecStart=/usr/local/bin/status-page
Restart=always
User=pi

[Install]
WantedBy=multi-user.target
```

Enable and start the service:
```bash
sudo systemctl enable status-page
sudo systemctl start status-page
```
