#!/bin/bash
cat ~/.config/programmini/buff/last_session.tmux | while read line;
do
	studio --end $line
done
studio --begin $1
echo "$1" > ~/.config/programmini/buff/last_session.tmux 
