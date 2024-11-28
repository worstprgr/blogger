#!/bin/bash

# You'll need `inotify-tools`

# Extreme lazy and hacky approach. Without proper logging and signaling.
# This should only give you a rough idea how it could work.
#
# Note: I'm using `rm` twice on different places. 
#       This might not be your use case.


watch_file="/<path to git repo>/foobar-blog.git/refs/heads"

inotifywait -m -e modify "$watch_file" | while read -r directory event filename; do
    echo "refs/heads modified!"
    cd /<blog dir> && \
        echo "Purging entries (local) ..." && \
        rm -rf /<blog dir>/page/www/posts/*.html && \
        echo "Fetching remotes ..." && \
        git fetch && \
        echo "Pulling latest commit ..." && \
        git reset --hard origin/main && \
        echo "Updating blog ..." && \
        blogger -u && \
        echo "Purging entries (nginx)..." && \
        rm -rf /usr/share/nginx/html/blog/posts/*.html && \
        echo "Copying contents to nginx/www ..." && \
        cp -r page/www/* /usr/share/nginx/html/blog && \
        echo "Blog updated"
done