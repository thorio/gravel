FROM mcr.microsoft.com/devcontainers/rust:0-1-bullseye

RUN apt-get update && \
	apt-get install libx11-dev libxext-dev libxft-dev libxinerama-dev libxcursor-dev libxrender-dev libxfixes-dev libpango1.0-dev libgl1-mesa-dev libglu1-mesa-dev -y --no-install-recommends && \
	apt-get clean && \
	rm -rf /var/lib/apt/lists/*
