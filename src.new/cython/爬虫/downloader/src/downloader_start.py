import sys

from downloader import main

if len(sys.argv) == 2:
    main(sys.argv[1])
else:
    print("Please use: python downloader_start.py <url>")
    sys.exit(-1)

