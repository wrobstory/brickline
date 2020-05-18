```
| ◯ ◯ ● |
| ◯ ◯ ● |
  | ◯ ● ◯ |
  | ◯ ● ◯ | 
    | ● ◯ ◯ |
    | ● ◯ ◯ |
```

# brickline

Brickline is a set of command line utilities for manipulating, modifying, and analyzing BrickLink LEGO wanted lists. 

This README contains information on how to
[install `brickline`](https://github.com/wrobstory/brickline#installation) as well as a guide to the available commands. 

## Commands

### Join

Join two Bricklink Wanted List on ItemID and Color, summing the minimum quantity (MinQty) values of the two lists; it will keep the remaining metadata from the lefthand list. This is something you can't do on Bricklink right now: if you try to copy a wanted list to another wanted list with duplicate ItemID/Color combinations you will get a "Warning: Item color combination already exists".

Example: 
```
$ ./target/release/brickline join -l ./resources/test/test_wanted_list_3.xml \
                                  -r ./resources/test/test_wanted_list_4.xml \
                                  -o /tmp/joined_wanted_list.xml
Left Bricklink Wanted List: ./resources/test/test_wanted_list_3.xml
Right Bricklink Wanted List: ./resources/test/test_wanted_list_4.xml
Merging wanted lists...
Writing joined wanted list to /tmp/joined_wanted_list.xml
```


## Installation 

You can compile from source by [installing Cargo](https://crates.io/install), ([Rust's](https://www.rust-lang.org/) package manager)


```bash
git clone git://github.com/wrobstory/brickline
cd brickline
cargo build --release
```

The resulting binary will be at `target/release/brickline`. 

I'm hoping to have Github binaries and a homebrew package available soon.
