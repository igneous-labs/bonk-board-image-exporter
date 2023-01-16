# bonk-board-image-exporter

Converts an image file to json pixel data for bonkbaord.gg.

```
Usage: bonk-board-image-exporter [OPTIONS] --input-file <INPUT_FILE> --output-file <OUTPUT_FILE>

Options:
  -i, --input-file <INPUT_FILE>
          Path to image file convert to bonk board user data
  -o, --output-file <OUTPUT_FILE>
          Path to json output file
  -p, --pixels-per-tx <PIXELS_PER_TX>
          Maximum number of pixels to pack into each transaction [default: 100]
  -t, --top-left-coord <TOP_LEFT_COORD>
          Coordinate of the top left corner to place the image at, in the format of "x,y" [default: 0,0]
  -c, --color-to-ignore <COLOR_TO_IGNORE>
          RGB 24bit value to be used to exclude pixels from the image, in the format of "r,g,b" [default: 0,0,0]
  -h, --help
          Print help
  -V, --version
          Print version
```


For example, export image `./daftbonk-final.bmp` to `./daftbonk-final.json`
excluding black pixels and position it at x = 375, y = 250:

```shell
./bonk-board-image-exporter -t "375,250" -c "0,0,0"  -i ./daftbonk-final.bmp -o ./daftbonk-final.json
INFO [bonk_board_image_exporter] Reading image file: ./daftbonk-final.bmp
INFO [bonk_board_image_exporter] Read total 2944 pixels (image: 64x46)
INFO [bonk_board_image_exporter] Excluding the given color Color { r: 0, g: 0, b: 0 }, prepared 1376 pixels to write
INFO [bonk_board_image_exporter] With the given maxium 100 pixels per tx, writing an array of length 14 to file
INFO [bonk_board_image_exporter] Done
```
