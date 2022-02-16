# Helper script to convert the png font to binary

from PIL import Image

# Load the font image
font = Image.open("./assets/cozette.png")

raw = []

# Loop through each pixel. Each byte is 8 pixels, where 1 is white and 0 is black.
for y in range(font.size[1]):
    for x in range(font.size[0]):
        pixel = font.getpixel((x, y))
        if pixel == (255, 255, 255, 255):
            raw.append(1)
        else:
            raw.append(0)

    # Pad the row with zeros to a multiple of 8
    while len(raw) % 8 != 0:
        raw.append(0)

# Convert the raw data to bits
bits = []
byte = 0
for i in range(len(raw)):
    byte = byte << 1
    byte = byte | raw[i]
    if (i + 1) % 8 == 0:
        bits.append(byte)
        byte = 0

with open("./assets/cozette.bin", "wb") as f:
    f.write(bytes(bits))
