#cython: language_level=3
#distutils: language = c++
"""
@ Filename: downloader.pyx
@ Author: Yang Zhenxun
"""
import os

import fake_useragent
import requests
from rich.console import Console
from rich.progress import (BarColumn, DownloadColumn, Progress, TextColumn,
                           TimeRemainingColumn, TransferSpeedColumn)
console = Console()

cdef void _main(str url):
    """A main function."""
    try:
        filepath = "..\\Downloaded files\\"
        filename = os.path.normpath(os.path.basename(url))
        user_agent = fake_useragent.UserAgent(\
        path = \
            "D:\\my-study--github\\"\
                    "src.new\\python\\downloader\\"
                        "v0.0.1\\src\\user_agent.json"\
                )
        ses = requests.session()
        ses.keep_alive = False
        req = ses.get(url, headers = {'User-Agent' : user_agent.random},\
            stream = True, timeout = 60, verify = False)
        total = int(req.headers.get('content-length', 0))
        with Progress(
                TextColumn("[bold blue]{task.fields[filename]}", justify="right"),
                BarColumn(bar_width=None),
                "[progress.percentage]{task.percentage:>3.1f}%",
                " ",
                DownloadColumn(),
                " ",
                TransferSpeedColumn(),
                " eta ",
                TimeRemainingColumn(),
            ) as progress_bar:
            task_id = progress_bar.add_task("", filename=filename, start=False)
            progress_bar.update(task_id, total = total)
            with open(filepath + filename, "wb") as file:
                progress_bar.start_task(task_id)
                for chunk in req.iter_content(1024):
                    size = file.write(chunk)
                    progress_bar.update(task_id, advance = size)
    except Exception:
        console.print_exception(extra_lines=8, show_locals=True)

def main(str url):
    try:
        _main(url)
    except Exception:
        console.print_exception(extra_lines=8, show_locals=True)
