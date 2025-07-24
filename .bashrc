[[ $- != *i* ]] && return

PS1='\[\033[0;35m\]┌\[\033[0;31m\]  \h  \[\033[0;32m\] \u  \[\033[0;33m\]  \w  \[\033[0;36m\] \t\[\033[0;35m\]\n└>\[\033[0m\] '

alias ls='ls --color=auto'
alias ll='ls -la --color=auto'
alias la='ls -A --color=auto'
alias grep='grep --color=auto'

export BUN_INSTALL="$HOME/.bun"
export PATH="$BUN_INSTALL/bin:$PATH"

export CHROME_EXECUTABLE=/usr/bin/chromium
