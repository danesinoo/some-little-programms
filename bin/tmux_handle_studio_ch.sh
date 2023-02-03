#!/bin/bash
cat ~/.config/programmini/buff/last_session.tmux | while read line
do
	studio $line
done
studio $1_begin
echo "$1_end" > ~/.config/programmini/buff/last_session.tmux 
