import argparse
import os
import aiohttp
import asyncio
from aiohttp import web
from retry import retry
from fake_useragent import UserAgent
from urllib import parse
from rich.console import Console
from rich.progress import (BarColumn, DownloadColumn, Progress, TextColumn,
                           TimeRemainingColumn, TransferSpeedColumn)
import PyTaskbar
console = Console()
prog = PyTaskbar.Progress()
prog.init()

async def total(url, session, header):
    async with session.head(url, headers=header, timeout=60, verify_ssl=False) as req:
        return int(req.headers.get('content-length', 0))

async def main(url, retry_nums):
    main_dir = os.path.split(os.path.abspath(__file__))[0]
    download_dir = os.path.join(os.path.dirname(main_dir), "Downloaded files")
    download_file = os.path.normpath(os.path.basename(parse.urlparse(url).path))
    user_agent = UserAgent().random
    async with aiohttp.ClientSession() as session:
        _total = await total(url, session, {'User-Agent': user_agent})
        print(_total)
    f = open(download_dir+download_file, "wb")
    @retry(tries=retry_nums)
    async def start_download():
        progress.start_task(task_id)
        prog.setState('normal')
        _headers = {'User-Agent': user_agent}
        async with aiohttp.ClientSession() as session:
            async with session.get(url, headers=_headers, verify_ssl=False, timeout=60) as req:
                response = web.StreamResponse()
                response.content_type = 'text/plain'
                await response.prepare(req)
                chunk_size = 2048
                chunks = []
                for chunk in req.iter_content(chunk_size=chunk_size):
                    chunks.append(chunk)
                    progress.update(task_id, advance=len(chunk))
                    prog.setProgress(len(chunk), total)
                f.seek(start)
                for chunk in chunks:
                    await f.write(chunk)
                del chunks
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
) as progress:
        task_id = progress.add_task("Download file:", filename=download_file, start=False)
        progress.update(task_id, total=_total)
        await start_download()
        f.close

if __name__ == '__main__':
    parser = argparse.ArgumentParser(description="A downloader.")
    parser.add_argument('-U', '--url', type=str, required=True)
    parser.add_argument('--retry-nums', type=int, required=False, default=3)
    arg = parser.parse_args()
    asyncio.run(main(arg.url, arg.retry_nums))
