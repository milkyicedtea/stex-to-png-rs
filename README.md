# stex-to-png-rs
Rust port of the [python script](https://github.com/ClementineAccount/godot-stex-to-png) made by [@ClementineAccount](https://github.com/ClementineAccount)
used to convert `.stex` files into readable `.png` files.

## Requirements
- [Rust](https://rust-lang.org/)

## Usage
```bash
stex-to-png-rs file.stex # converts one file to png
```

>[!TIP]
> This port can also convert multiple files at once, something that the original couldn't do without modification.
>
> ```bash 
> stex-to-png-rs *.stex # converts all .stex files in the folder to png
> ```

## More information
- [Godot Source Code where .stex files are read](https://github.com/godotengine/godot/blob/master/editor/import/resource_importer_image.cpp#L73)
- [Reddit post discussing .stex to .png conversions](https://www.reddit.com/r/godot/comments/n178h2/convert_stex_back_to_png/)