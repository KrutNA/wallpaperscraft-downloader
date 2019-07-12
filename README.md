## Downloader from [wallpaperscraft](https://wallpaperscraft.ru/)
### Multithreaded downloader from site wallpaperscraft
First program on **Rust**, just for leaning language.
### Usage
```
Wallpaperscraft downloader 0.1.0
KrutNA <krutko_n_a@mail.ru>
Downloads images from wallpaperscraft.ru

USAGE:
    wallpaperscraft-downloader [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -b, --begin <begin>      Begin page number [default: 1]
    -e, --end <end>          End page ("0" if to end) [default: 0]
    -r, --s <resolution>     image resolution [default: 1366x768]
    -t, --tag <tag>          base tag for downloading [default: games]
    -c, --count <threads>    Threads count [default: 5]
```
### Examples
All images downloading to `./files/<tag>/<page>/<image>` directory.

For example `~/ $ ./programs/wallpaperscraft-downloader`.

Will download all images to `~/files/<tage>/<page>/<image>`.

For some reasons max lenght of `<image>` name wil be not more than *100*.

###### `$ wallpaperscraft-downloader -h`
Shows help message, instead `-h` you can use `--help` flag.

###### `$ wallpaperscraft-downloader`
Starts downloading all images by default parameters, which you can see from *help* message.

###### `$ wallpaperscraft-downloader -b 5 -e 30 --tag=animals -c 8`
Starts downloading all images from **5th** to **30th** pages with tag **animals** in **8** threads. 
