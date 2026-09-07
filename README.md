# i3 instant layout

A small utility to switch layout instantly. Rust-based implementation of Python-based [i3-instant-layout](https://github.com/TyberiusPrime/i3-instant-layout).

Even though the Python version works fine, I wanted to build it in Rust. I have two broken monitors with usable areas,
but i3 do not have a way to define fixed layout easily. So this tool exists! I will add a way to define more layouts
easily someday. But it does what I need it to do, so additional updates are most likely not going to happen.

Just run with any of the following arguments. I didn't add all the layout options that are implemented in Python version
But, it is pretty easy to do

`i3-instant-layout splitv`

```
splitv
splith
splitv3
splith3
splitv2
splith2
4k
2k
```
