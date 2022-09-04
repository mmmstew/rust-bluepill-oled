# rust-bluepill-oled
Flashes "Hello World!" every 100ms, for 100ms using the ubiquitous STM32F103C8-based "Bluepill" connected to the equally ubiquitous SSD1306-based 0.96" OLED available via Aliexpress etc.

##Connections

SDA -> PB9

SCL -> PB8

## Pre-requisites
**Update rust**

    rustup update

**Install target**

    rustup target install thumbv7m-none-eabi

**Install flash tool (For ST-Link V2 and clones)**

    cargo install cargo-flash

**Get the code**

    git clone https://github.com/mmmstew/rust-bluepill-oled.git

**Jump into the folder**

    cd rust-bluepill-oled


## Usage
**Build the project**

    cargo build --release

**Flash to target**

    cargo flash --chip STM32F103C8 --connect-under-reset --release

**If updating linker script or things Cargo might not notice, it can be helpful to follow up with**

    cargo clean
