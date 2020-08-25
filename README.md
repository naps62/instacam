# Instacam

**Filters for your webcam**

This is a pet project motivated mostly by my desire to learn both [Rust](https://www.rust-lang.org/) and [FFmpeg](https://ffmpeg.org/)

It grabs an input video feed (most likely your original webcam), and outputs
a transformed video feed.


## Prerequisites

Currently only Linux is supported.

Setup [v4l2loopback](https://github.com/umlaeute/v4l2loopback), and create
a virtual webcam, such as:

```sh
sudo modprobe v4l2loopback devices=1 video_nr=2 card_label="Instacam" exclusive_caps=1
```

On my machine, this creates a `/dev/video2` device. Depending on how many you
already have, yours might be named differently.

## Setup

1. Clone the repo

```sh
# clone the repo
git clone https://github.com/naps62/instacam.git
cd instacam
```

2. Run the Rust daemon:

```sh
cd daemon
cargo run -- -i /dev/video0 -o /dev/video2 --width 1280 --height 720 --fps 30
```

3. Run the optional Web UI (for real time configuration edits)

```sh
cd ui
npm install
npm start
```
