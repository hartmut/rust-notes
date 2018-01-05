# what is it good for
Experimenting with [bindgen](https://crates.io/crates/bindgen) and [AtomSpace](https://github.com/opencog/atomspace) and other functions from the [OpenCog](https://github.com/opencog) project.

# current problems
* `time_t` from `/usr/include/time.h` does not work with bindgen
* bindgen has problems with the `namespace` concept
