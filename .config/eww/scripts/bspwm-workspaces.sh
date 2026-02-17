#!/bin/bash

bspc subscribe report | while read -r line; do
IFS=':' read -ra ADDR <<< "${line#?}"

output="["
first=true

for i in "${ADDR[@]}"; do
  case $i in
    [fFoOuU]*)
      if [ "$first" = false ]; then output+=","; fi
      status="${i:0:1}"
      id="${i:1}"
      output+="{\"id\":\"$id\",\"status\":\"$status\"}"
      first=false
      ;;
  esac
done

output+="]"
echo "$output"
done
