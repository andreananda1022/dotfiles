[[ $- != *i* ]] && return

alias ls='ls --color=auto'
alias grep='grep --color=auto'

if [[ "$TERM" == "linux" ]]; then
  PS1='[\u@\h \W]\$ '
else
  PS1='\[\033[0;31m\]┌  \h  \[\033[0;32m\] \u  \[\033[0;33m\]  \w  \[\033[0;36m\] \t\[\033[0;35m\]\n\[\033[0;31m\]└>\[\033[0m\] '
fi
