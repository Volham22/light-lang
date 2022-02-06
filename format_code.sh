#!/bin/sh

find -name '*.rs' -exec rustfmt {} \;
