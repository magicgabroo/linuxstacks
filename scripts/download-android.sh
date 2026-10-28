#!/bin/bash
# Download Android x86_64 image for LinuxStacks

set -e

ANDROID_DIR="$HOME/LinuxStacks/images"
mkdir -p "$ANDROID_DIR"

IMAGE_URL="https://dl.google.com/android/repository/android-x86_64.img"
IMAGE_PATH="$ANDROID_DIR/android.img"

echo "Downloading Android x86_64 image..."
echo "URL: $IMAGE_URL"
echo "Destination: $IMAGE_PATH"

if [ -f "$IMAGE_PATH" ]; then
    echo "Image already exists. Skipping download."
    exit 0
fi

wget --show-progress -O "$IMAGE_PATH" "$IMAGE_URL"

if [ -f "$IMAGE_PATH" ]; then
    SIZE=$(du -h "$IMAGE_PATH" | cut -f1)
    echo "Download complete! Size: $SIZE"
else
    echo "Download failed!"
    exit 1
fi