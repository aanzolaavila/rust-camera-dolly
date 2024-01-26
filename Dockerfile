FROM --platform=linux/amd64 rust:1.75.0-bookworm

# Install dependencies for cross-compiling to Arduino (like avr-gcc)
RUN apt-get update && apt-get install -y \
	gcc-avr \
	avr-libc \
	avrdude \
	pkg-config \
	libudev-dev \
	build-essential \
	binutils-avr \
	clang \
	libclang-dev \
	libc6 \
	&& apt-get clean \
	&& rm -rf /var/lib/apt/lists/*

# Set the working directory

WORKDIR /usr/src/myapp

RUN cargo install ravedude

RUN rustup override set nightly
RUN rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu

RUN wget -qO arduino-cli.tar.gz https://downloads.arduino.cc/arduino-cli/arduino-cli_latest_Linux_64bit.tar.gz && tar xf arduino-cli.tar.gz -C /usr/local/bin arduino-cli

RUN arduino-cli config init
RUN arduino-cli core update-index; \
	arduino-cli lib update-index
RUN arduino-cli core install arduino:avr
# RUN arduino-cli lib install "LiquidCrystal"

RUN mkdir -p $HOME/Arduino/libraries ; wget https://github.com/johnrickman/LiquidCrystal_I2C/archive/refs/tags/1.1.3.tar.gz -O liquidcrystal.tar.gz \
	&& tar xf liquidcrystal.tar.gz -C $HOME/Arduino/libraries \
	&& rm liquidcrystal.tar.gz
RUN cd ${HOME}/Arduino/libraries && mv LiquidCrystal_I2C-1.1.3 LiquidCrystal_I2C

# Build your application
CMD ["rustup", "run", "nightly", "cargo", "build", "--release"]
