~/MEGA/Rays/Programming/python/camera/security-cams/cam-viewer-rs/target/release/cam-viewer-rs

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

How do I fix this error:
```
error: lifetime may not live long enough
   --> src/main.rs:205:55
    |
199 |       fn view(&self) -> Element<'_, Message> {
    |               - let's call the lifetime of this reference `'1`
...
205 |                   let player = lazy(self.version, |_| { VideoPlayer::new(video)
    |  _______________________________________________________^
206 | |                     .on_new_frame(Message::OnNewFrame(Instant::now(), video_info.serial_no.clone()))
207 | |                     .on_error(|e| Message::OnCamError(video_info.serial_no.clone(), e.to_string()))
    | |___________________________________________________________________________________________________^ returning this value requires that `'1` must outlive `'static`

```
