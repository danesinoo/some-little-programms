#!/bin/bash
studio --end $1
rm ~/.config/programmini/buff/last_session.tmux
touch ~/.config/programmini/buff/last_session.tmux
