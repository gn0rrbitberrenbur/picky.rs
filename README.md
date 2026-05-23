# picky.rs
picky.rs is an application that lets you swipe through your images. Just select an folder containing .jpg/.raw filepairs and start swiping!

## Run
### Install releases
Head over to the [releases](https://github.com/gn0rrbitberrenbur/picky.rs/releases) and download the release for you platform.

### Build yourself
To run build picky.rs follow those steps:

#### 1. Install the dependencies
- Install Tauri dependencies([see here](https://v2.tauri.app/start/prerequisites/))
- Install Tauri ([see here]())

#### 2. Test your development environment
Then clone the repo and run the following:
```
cd picky.rs
npm run tauri dev
```

#### 3. Build
To build picky.rs run the following:
```
npm run tauri build
```

You can find the built file in `../src-tauri/target/release/bundle/..`