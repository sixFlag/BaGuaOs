#!/bin/bash

/home/ubuntu/blog_os/rust-gdb/rust-os-gdb/bin/rust-gdb \
	-ex 'file target/x86_64-blog_os/debug/six_os' \
	-ex 'target remote 127.0.0.1:1234'