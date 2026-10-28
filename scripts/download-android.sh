#!/bin/bash
# Download Android x86_64 image for LinuxStacks
#
# Usage: ./scripts/download-android.sh [url]
#
# Examples:
#   ./scripts/download-android.sh
#   ./scripts/download-android.sh "https://dl.google.com/android/repository/sys-img/android/x86_64/android-x86_64_r01.zip"

set -e

ANDROID_DIR="$HOME/LinuxStacks/images"
mkdir -p "$ANDROID_DIR"

# Default URL - Android x86_64 system image (Google APIs)
IMAGE_URL="${1:-https://dl.google.com/android/repository/sys-img/android/x86_64/android-x86_64_r01.zip}"
IMAGE_PATH="$ANDROID_DIR/android-x86_64.zip"

echo "Baixando imagem Android x86_64..."
echo "URL: $IMAGE_URL"
echo "Destino: $IMAGE_PATH"

if [ -f "$IMAGE_PATH" ]; then
    echo "Imagem já existe. Pulando download."
    exit 0
fi

wget --show-progress -O "$IMAGE_PATH" "$IMAGE_URL"

if [ -f "$IMAGE_PATH" ]; then
    SIZE=$(du -h "$IMAGE_PATH" | cut -f1)
    echo "Download concluído! Tamanho: $SIZE"
    echo ""
    echo "Próximos passos:"
    echo "  1. Extraia a imagem:"
    echo "     unzip -o $IMAGE_PATH -d $ANDROID_DIR/"
    echo "  2. No LinuxStacks, configure a URL da imagem para:"
    echo "     $ANDROID_DIR/system-images/android-*/x86_64/*.img"
else
    echo "Falha no download!"
    exit 1
fi
