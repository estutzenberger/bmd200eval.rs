#!/bin/bash

openocd -f openocd.cfg -c init -c "flashall target/thumbv6m-none-eabi/release/blink.hex 0" -c shutdown
