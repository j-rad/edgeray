#!/bin/bash
# EdgeRay Asset Downloader - Improved Naming
# Consolidates all CDN dependencies for offline use

VENDOR_DIR="edgeray-app/assets/vendor"
rm -rf "$VENDOR_DIR"
mkdir -p "$VENDOR_DIR"

echo "Downloading Tailwind CSS CDN..."
curl -s -L "https://cdn.jsdelivr.net/npm/@tailwindcss/browser@4" -o "$VENDOR_DIR/tailwind.js"

# Function to download font family
download_font() {
    local family=$1
    local name=$2
    local output_css="$VENDOR_DIR/${name}.css"
    echo "Processing font: $family as $name"
    
    local css_content=$(curl -s -H "User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36" \
        "https://fonts.googleapis.com/css2?family=$family&display=swap")
    
    local i=0
    while read -r url; do
        if [ -z "$url" ]; then continue; fi
        i=$((i+1))
        local filename="${name}_$i.woff2"
        echo "  Downloading $filename..."
        curl -s -L "$url" -o "$VENDOR_DIR/$filename"
        css_content=$(echo "$css_content" | sed "s|$url|./vendor/$filename|g")
    done <<EOF
$(echo "$css_content" | grep -o 'https://fonts.gstatic.com[^)]*')
EOF

    echo "$css_content" > "$output_css"
}

download_font "Inter:wght@400;500;600;700" "inter"
download_font "JetBrains+Mono:wght@400;500" "jetbrains-mono"
download_font "Material+Symbols+Outlined:opsz,wght,FILL,GRAD@20..48,100..700,0..1,-50..200" "material-symbols"

echo "Assets consolidated in $VENDOR_DIR"
