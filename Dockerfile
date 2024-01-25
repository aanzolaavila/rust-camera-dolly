FROM rust:latest

# Install dependencies for cross-compiling to Arduino (like avr-gcc)
RUN apt-get update && apt-get install -y \
	gcc-avr \
	avr-libc \
	avrdude \
	pkg-config \
	libudev-dev \
	build-essential \
	binutils-avr \
	&& apt-get clean \
	&& rm -rf /var/lib/apt/lists/*

# Set the working directory

WORKDIR /usr/src/myapp

RUN rustup override set nightly
RUN rustup component add rust-src --toolchain nightly-aarch64-unknown-linux-gnu

ENV ARDUINO_IDE_VERSION 1.8.5
RUN (wget -q -O- https://downloads.arduino.cc/arduino-${ARDUINO_IDE_VERSION}-linux64.tar.xz \
	| tar xJC /usr/local/share \
	&& ln -s /usr/local/share/arduino-${ARDUINO_IDE_VERSION} /usr/local/share/arduino \
	&& ln -s /usr/local/share/arduino-${ARDUINO_IDE_VERSION}/arduino /usr/local/bin/arduino)

# Build your application
CMD ["rustup", "run", "nightly", "cargo", "build", "--release"]
