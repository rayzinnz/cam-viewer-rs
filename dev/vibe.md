```sh
cd /home/ray/MEGA/Rays/Programming/python/camera/security-cams/cam-viewer-rs
aider --chat-mode ask --edit-format diff
aider --edit-format diff
aider --edit-format diff --no-verify-ssl

--yes-always


/add Cargo.toml src/main.rs
/read-only dev/iced_v0.14_repo_map.md dev/iced_v0.14_tour_main.rs

/run cargo check --message-format=short
/run cargo build
```

**vibe**
What's wrong? Fix. 
You're own knowledge of iced v0.14 is out of date. Please only use the included repo map read-only file "iced_v0.14_repo_map.md" and example code in "iced_v0.14_tour_main.rs" as the single source of truth regarding iced v0.14 structure.

Make a rust app that streams an rstp camera to screen using iced v0.14.
Here is the url for the stream: `rtsp://admin:your_password@192.168.1.34:554/user=admin_password=v4HXkn4qbU_channel=1_stream=0a&protocol=unicast.sdp?real_stream`
