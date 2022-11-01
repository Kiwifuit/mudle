# The Mµdle Project
A simple API for [my school's LMS](https://basic-ed.cit.edu)

This project is a rewrite of [the original repository](https://github.com/Kiwifuit/MoodleTUI), only that its more portable than ever

- [The Mµdle Project](#the-mµdle-project)
  - [What is this project about?](#what-is-this-project-about)
  - [To Investigate](#to-investigate)
  - [Comparison between this and the original Moodle TUI Project](#comparison-between-this-and-the-original-moodle-tui-project)
  - [Usage](#usage)
    - [Docker Image](#docker-image)
    - [Native Executable](#native-executable)
  - [Compilation](#compilation)
    - [Docker Image](#docker-image-1)
    - [Native Executable](#native-executable-1)

## What is this project about?
This is a project aimed at creating a library that interacts with the LMS (and possibly automating the process)

Why?

Because the LMS burns my eyes with light mode.

I eventually plan to have a working web server, which adds better UI and stuff.

For now, I just write the library as a proof of concept that I can do it.

> *Fun fact: This is the 3rd Iteration of the project. It was under the name MoodleTUI before this*

## To Investigate
- [x] Session Key
  - Used extensively in the site fsr
  - A hidden `input` on all pages
- [x] Login System (`/login/index.php`)
  - [x] Obtain `MoodleSession`
   - `anchor` is always empty
   - `logintoken` can be scraped from a hidden `input` element
   - `username` and `password` should be user input (obviously)
- [ ] File Storage System (`/repository/draftfiles_ajax.php`)
  - [ ] Upload files
  - [ ] Download files
  - [x] List files (`?action=list`)
- [ ] Calendar System
- [ ] Notification System
- [ ] Myserious `service.php` (`/lib/ajax/service.php`)
  - Used in obtaining calendar and notification data
  - Returns JSON
  - Dunno why the client calls it, but exposed data entrypoint for me 😋


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

mv target/release/mtui ..

cd ..
rm -vrf mtui-rs
./mtui
```

*(Optional) Run tests before building*

```
cargo test
```
