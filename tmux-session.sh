#!/bin/zsh

SESSIONNAME="SMD-rustBackend"
tmux has-session -t $SESSIONNAME &> /dev/null

if [ $? != 0 ]
 then
    tmux new-session -s $SESSIONNAME -n nvim -d
    tmux send-keys -t $SESSIONNAME "nvim ." C-m
    tmux new-window -t $SESSIONNAME -n lazygit -- lazygit
    tmux new-window -t $SESSIONNAME -n cargo
    tmux new-window -t $SESSIONNAME -n docker
fi

tmux attach -t $SESSIONNAME

