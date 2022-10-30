#!/bin/sh

rustfmt $(find src -name "*.rs") build.rs --edition 2021
