#!/bin/sh

ffmpeg \
  -i green.png \
  -f v4l2 -framerate 25 -video_size 640x480 \
  -i /dev/video0 \
  -filter_complex \
  "
  " \
  -shortest \
  -vcodec rawvideo -pix_fmt yuv420p \
  -f v4l2 -f matroska - | ffplay -fflags nobuffer -

