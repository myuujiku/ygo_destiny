#!/bin/sh

schema_dir="$HOME/.local/share/glib-2.0/schemas"

mkdir -p $schema_dir
cp "./resources/gschema.xml" "$schema_dir/com.myujiku.ygo_destiny.gschema.xml"
glib-compile-schemas $schema_dir
