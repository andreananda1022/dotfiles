#!/bin/sh

if xrandr | grep -q "^HDMI-1 connected"; then
    xrandr \
        --output eDP-1 --auto --primary \
	--output HDMI-1 --mode 1920x1080 --rate 180 --right-of eDP-1
else
    xrandr --output eDP-1 --auto --promary
fi

picom &
feh --bg-fill --no-fehbg --randomize ~/Pictures &
exec dwm
