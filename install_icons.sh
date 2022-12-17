#!/bin/sh

cd icons

for size in 16 32 64 128 256 512
do
    xdg-icon-resource install --size $size checked_$size.png workbreak-ready
    xdg-icon-resource install --size $size pomodoro_$size.png workbreak-pomodoro
    xdg-icon-resource install --size $size speedometer_$size.png workbreak-efficiency
    xdg-icon-resource install --size $size headache_$size.png workbreak-injury
    xdg-icon-resource install --size $size coffee-cup_$size.png workbreak-break
done
