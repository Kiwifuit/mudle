<!--
 Copyright (c) 2022 Misery <mahkiwi123@gmail.com>

 This software is released under the MIT License.
 https://opensource.org/licenses/MIT
-->

# MoodleTUI.rs

This project is a rewrite of the original Moodle TUI repository, only that its more portable than ever

- [MoodleTUI.rs](#moodletuirs)
  - [Comparison between this and the original Moodle TUI Project](#comparison-between-this-and-the-original-moodle-tui-project)
  - [Usage](#usage)
    - [Docker Image](#docker-image)
    - [Native Executable](#native-executable)
  - [Compilation](#compilation)
    - [Docker Image](#docker-image-1)
    - [Native Executable](#native-executable-1)

## Comparison between this and the original Moodle TUI Project
- [x] Does not require Python
- [x] Standalone
- [x] More performant (now that its being rewritten in rust)
- [x] Less bugs (with the built-in testing that the language comes with)
- [x] Docker Image (w/ Docker Compose to manage the program's data easily)

## Usage
### Docker Image
***With Docker Compose***
- Clone the repository
- Run `docker compose up`

***Without Docker Compose***
- Pull the latest image from the packages tab
- Make a volume called `mtui-data`
  ```
  docker volume create mtui-data
  ```
  *`docker volume ls` should return something like this if you don't have other volumes:*
  ```
  DRIVER    VOLUME NAME
  local     mtui-data
  ```
- Run `docker run -it -v mtui-data:/app/res:rw mtui:latest`

### Native Executable
- Go to the releases tab
- Get the latest version of the program
- Execute

## Compilation
***I assume you know what these commands do so not a lot of text instructions from here on out. Just copy-and-paste commands***

### Docker Image
***Requirements***:
- Docker
- Git

```
git clone https://github.com/Kiwifuit/mtui-rs
cd mtui-rs
docker build -t mtui:local .
docker run -it -v mtui-data:/app/res:rw mtui:latest
```

### Native Executable
***Requirements***:
- Git
- Rustup (Rust installer)

***On windows, I recommend running these commands on PowerShell instead of the command prompt***

```
git clone https://github.com/Kiwifuit/mtui-rs
cd mtui-rs
cargo build --release
```
On linux:
```
mv target/release/mtui ..
```

On windows:
```
mv target/release/mtui.exe ..
```

```
cd ..
rm -vrf mtui-rs
./mtui
```

*(Optional) Run tests before building*

```
cargo test
```