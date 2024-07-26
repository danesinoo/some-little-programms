#!/bin/bash

# Execute the command and store the result in a variable
result=$(yabai -m query --windows is-floating --window)

if [[ $result == *'false'* ]]; then
	yabai -m window --toggle float
	yabai -m window --grid 1:2:1:1:1:1
else
	yabai -m window --grid 1:2:1:1:1:1
fi
