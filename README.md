# Console Simple Image Viewer

## Using

```
Usage: csiv [OPTIONS] --image <image>

Options:
  -i, --image <image>  Image filepath
  -s, --scale <scale>  Image scale [default: 1]
  -h, --help           Print help
  -V, --version        Print version
```

## Example

```
./csiv -i "/opt/app/test.jpg"
```

<img src="example_scale_1.png" alt="CSIV example, scale 1">

```
./csiv -i "/opt/app/test.jpg" -s 0.5
```

<img src="example_scale_0.5.png" alt="CSIV example, scale 0.5">
