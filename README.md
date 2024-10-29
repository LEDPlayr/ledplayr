# LEDPlayr

![LEDPlayr Logo](web/static/logo.png)

LEDPlayr is a lightweight player and scheduler for xLights FSEQ files

## What is supported?
* xLights integration
	* LEDPlayr will appear as a v6 FPP instance allowing model, outputs, and sequence uploads straight from xLights
	* LEDPlayr supports the FPP discovery protocol so should be automatically found on your network
* FSEQ files - this the rendered file from xLights
* Playlists with multiple sequences
* Scheduling of sequences
	* By date, day of the week, and time period
* Display of logs in the WebUI
* Dark mode
* Ability to test models
    * A combination of static and moving patterns can be used to test models

## What isn't supported?
* Testing models
* Essentially any form of error handling if you mess up your config
	* This includes uploading a sequence with the wrong number of nodes
* Playing of media
	* Uploads from xLights will be accepted but there is no facility to play media

## But FPP exists!
It does and it has a LOT more features. LEDPlayr was born out of my frustration downloading a near 800MB docker image onto a storage and memory constrained RaspberryPi 3B and also my desire to learn rust. Those may not be particularly good reasons but we are where we are and I learnt a lot along the way 😀 Hopefully someone else find use in my efforts.

## (Somewhat) Planned Features
Most of what I need is included. Below is a list of what I may include if I find time / have the need for it.

* Docker images
* Webcam streaming to the UI to make testing LED placement easier
* Ability to view a sequence in the Web browser on the 3D Display
	* I have no real need for this but it sounds fun and cool

## Unplanned Features
* Built-in sequences and patterns
	* I use xLights for sequencing or WLED does a decent job of timed pre-planned sequencing
* Playing of media

## Known Issues
* Uploading via xLights can result in a sequence that has one less node than intended
	* This only happens when uploading sequences at the same time as outputs
	* It is advisable to untick all sequences and select "All" in the "UDP Out" column to upload outputs
    * After initial output upload select "None" in the "UDP Out" column and tick sequences
    * This issue is being actively investigated

## How to run
Create the required configuration files and directories. None of the directories have fixed values but below are recommended paths and a sample `systemd` unit file.

### Download
Download the pre-built binary from Github releases. Currently available builds for x86_64 and aarch64. Save the file to `/usr/local/bin/ledplayr` and make it executable.

### Configuration
`/usr/local/etc/ledplayr.toml`
```toml
database_url = "/usr/local/share/ledplayr/db.sqlite" # Required
storage = "/usr/local/share/ledplayr/storage" # Required
multicast = true # Optional, defaults to true

[scheduler]
auto_start = true # Optional, defaults to true

[web]
bind = "0.0.0.0" # Optional, defaults to "0.0.0.0"
port = 80 # Optional, defaults to 3000

[log] # Optional, defaults to no file logging
directory = "/usr/local/share/ledplayr/storage" # Required
prefix = "ledplay." # Optionsl, defaults to ""
period = "hour" # Optional, log rotate period - defaults to never
                # options are "minute", "hour", "day", "never"
max_files = 3 # Optional, default is unlimited
```

### Systemd
`/etc/default/ledplayr`
```env
CONFIG=/usr/local/etc/ledplayr.toml
RUST_LOG=info
```

`/usr/lib/systemd/system/ledplayr.service`
```systemd
[Unit]
Description=LEDplayr - Christmas light sequencer
After=network.target

[Service]
EnvironmentFile=-/etc/default/ledplayr
ExecStart=/usr/local/bin/ledplayr
KillMode=process
Restart=on-failure
RestartPreventExitStatus=255

[Install]
WantedBy=multi-user.target
```

You can then start ledplayr

```bash
sudo systemctl daemon-reload
sudo systemctl enable ledplayr
sudo systemctl start ledplayr
```

## How to build
LEDPlayr uses Rust for the backend and Svelte/Typescript for the frontend. Both apps are included in this repo and can be built as follows. Ensure that you have up to date Rust and Typescript toolchains available to continue.

```bash
cd web
pnpm install
pnpm run build
cd ..
cargo build
```

The Rust build bundles the built web app into a single binary so it is essential to build the web interface ahead of time.

## Development
 When developing it is possible to use the vite proxy and two sessions.

```bash
# Terminal 1
CONFIG=config.toml cargo run --bin ledplayr
```

```bash
# Terminal 2
cd web
PROXY=http://127.0.0.1:3000 pnpm run dev
```

### OpenAPI
The rust application is capable of generating an `openapi.json` file which can then be used by Typescript to generate a client for the frontend. If you update any of the `axum` endpoints then it is essential to rebuild the OpenAPI client.

```bash
cargo run --bin gen_openapi
cd web
pnpm run openapi
cd ..
```
