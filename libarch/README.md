# libarch
The implementation of the architecture.

Here you'll find implementations for interrupts, register abstractions, segmentations and everything else that is arch dependent. 

The holy scriptures for this one are the [intel manual](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html), the [osdev wiki](https://wiki.osdev.org) and [this repo](https://github.com/rust-osdev/x86_64). 


## TODOS
- [X] figure out the idt and register the interrupt table to that.
- [X] implement abstractions for segments.
- [ ] implement abstractions for registers
 