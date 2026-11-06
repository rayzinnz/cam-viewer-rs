```sh
cd /home/ray/MEGA/Rays/Programming/python/camera/security-cams/cam-viewer-rs
aider --chat-mode ask
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

In this app, the part that connects to the camera streams here: `Video::new` is blocking the UI. How can I idiomatically prevent UI freezing while camera connecting in iced v0.14?
