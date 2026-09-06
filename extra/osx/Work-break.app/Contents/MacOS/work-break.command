#!/bin/sh

# Get the absolute directory path of where this script is located
FILEPATH=$(cd "$(dirname "$0")" && pwd)

# Run the executable in the background, handling spaces in paths safely
"$FILEPATH/work-break" &

exit 0
