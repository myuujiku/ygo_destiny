#!/bin/sh

rustfmt $(find src -name "*.rs") --edition 2021
