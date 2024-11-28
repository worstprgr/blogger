# Update Blog Script
This script is kinda a workaround. At the first iteration, 
I wanted to use git-hooks. But since I'm running my Git server inside a 
container, it is kinda pain in the bum to set this up.  

So I hacked something with `inotifywait`.  

## Workflow
```text

Client ----------------------------------
              +----------+
              | git push |
              +----------+
                    |
                    v
Server ----------------------------------
     +----------------------------+
     | Event: modify @ refs/heads |
     +----------------------------+
                    v
        +-----------------------+
        | Purge (local) entries |
        +-----------------------+
                    v
    +-------------------------------+
    | Git fetch & pull (hard reset) |
    +-------------------------------+
                    v
         +--------------------+
         | Calling Blogger -u |
         +--------------------+
                    v
      +---------------------------+
      | Purge (webserver) entries |
      +---------------------------+
                    v
       +------------------------+
       | Copy blog to webserver |
       +------------------------+

```

## Crontab & Initial run
Note: This is also a very lazy approach. Starting this script via `nohup` 
creates a file `nohup.out` which contains data from stdout/err.  
So keep in mind, this file will grow over time.

Starting this script in the background:  
`nohup ./update_blog.sh &`  

Crontab:  
`crontab -e` -> `@reboot /<path to update script>/update_blog.sh`
