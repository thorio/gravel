FROM mcr.microsoft.com/devcontainers/rust:1-bookworm

RUN apt-get update && \
	apt-get install libx11-dev libxext-dev libxft-dev libxinerama-dev libxcursor-dev libxrender-dev libxfixes-dev libpango1.0-dev libgl1-mesa-dev libglu1-mesa-dev mingw-w64 libgtk-3-bin xdg-utils xterm -y --no-install-recommends && \
	apt-get clean && \
	rm -rf /var/lib/apt/lists/*

RUN rustup target add x86_64-unknown-linux-gnu x86_64-pc-windows-gnu
RUN rustup component add rustfmt
