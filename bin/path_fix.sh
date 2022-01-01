#!/bin/bash
sed -e 's/"//".//g' -e "s/'//'.//g" ../dist/index.html > ../dist/index.html