# VTracer_Py

[VTracer](https://www.visioncortex.org/vtracer/) is an excellent project from [visioncortex](https://www.visioncortex.org/) that converts binary and color raster images to vector SVGs. It's written in Rust, so this binding aims to make it accessible from Python.

### Installation

```shell
pip install vtracer
```

### Usage

```python
import vtracer

input_path = "/path/to/some_file.jpg"
output_path = "/path/to/some_file.vtracer.jpg"

# Minimal example: use all default values, generate a multicolor SVG
vtracer.convert_image_to_svg_py(inp, out)

# Single-color example. Good for line art, and much faster
# than color runs:
vtracer.convert_image_to_svg_py(inp, out, colormode='binary')

# All the bells & whistles
vtracer.convert_image_to_svg_py(inp,
                                out,
                                colormode = 'color',        # ["color"] or "binary"
                                hierarchical = 'stacked',   # ["stacked"] or "cutout"
                                mode = '',                  # ["spline"]"polygon", "none"
                                filter_speckle = 4,         # default: 4
                                color_precision = 6,        # default: 6
                                layer_difference = 16,      # default: 16
                                corner_threshold = 60,      # default: 60
                                length_threshold = 4.0,     # in [3.5, 10] default: 4.0
                                max_iterations = 10,        # default: 10
                                splice_threshold = 45,      # default: 45
                                path_precision = 3          # default: 8
                                )

```

### Note: File-to-file tracing only

Note that the only image conversion method we expose (`vtracer.convert_image_to_svg_py()`) accepts string paths to read
and write images to/from disk. [VTracer](https://github.com/visioncortex/vtracer) doesn't expose in-memory image transforms, so we don't do that in Python either. The computation time to trace an image is so much greater than the time to read/write from disk that this doesn't seem like a great loss.

### Caveats

Windows, man. Probably doable, but I haven't gotten there yet.

### Contact

Please file an [issue](https://github.com/etjones/vtracer_py/issues) on Github, or send mail to [me](mailto:evan_t_jones@mac.com) with comments.
