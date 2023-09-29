# Custom Platform Template for OneTagger

This is a template project for creating custom platforms for OneTagger AutoTag feature.

## Getting started
Replace `my-platform` with your platform's name.

### 1. Clone the repository
```
git clone https://github.com/Marekkon5/onetagger-platform-template.git my-platform
```

### 2. Edit `Cargo.toml`

Change `my-platform` to your platform name
```
[package]
name = "my-platform"
```

Add dependencies:

```
[dependencies]
log = "0.4"
anyhow = "1.0"
serde_json = "1.0"

onetagger-tagger = { git = "https://github.com/Marekkon5/onetagger.git" }
```

**NOTE:** It is recommended to use a specific commit of `onetagger-tagger`

### 3. Implement
In `src/lib.rs` implement an `AutotaggerSourceBuilder`, `AutotaggerSource` and call `create_plugin!` with your `AutotaggerSourceBuilder` implementation.

The existing code includes barebone example implementation that doesn't match any track.

To see docs run: `cargo doc --open`

### 4. Replace the `icon.png` with your own.
 (Should be 1:1 aspect ratio)

### 5. Compile
```
cargo build --release
```
(Or you can push to Github and wait for the Github Actions CI to build your plugin to all platforms)

### 6. Install:

From `target/release/` copy:
- on Linux: `.so` file
- on Window: `.dll` file
- on MacOS: `.dylib` file

to: 
- on Linux: `~/.config/onetagger/platforms`
- on Window: `%appdata%\OneTagger\OneTagger\platforms`
- on MacOS: `/Users/your-user-account/Library/Preferences/com.OneTagger.OneTagger/platforms`


### 7. Restart OneTagger and use.

### 8. (Optional) Publish

You can create or edit the `info.json` file with metadata about the platform, and then create a Pull Request to include this platform within the `platforms` folder in https://github.com/Marekkon5/onetagger-platforms

By doing this other will be able to download your platform directly from 1T.