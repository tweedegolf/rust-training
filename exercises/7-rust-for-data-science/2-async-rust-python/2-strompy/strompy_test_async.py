import asyncio
import aiofiles
import strompy
import random

"""
Open file `op.json`, and feed it to a Strompy writer in small chunks. Returns
when all bytes have been fed.
"""
async def feed(writer):
    async with aiofiles.open('op.json', mode='rb', buffering=1000) as file:
        while True:
            chunk = await file.read(random.randint(0, 128))
            if len(chunk) == 0:
                break
            await strompy.feed_bytes(writer, chunk)
        print('Done reading!')

"""
Poll the Strompy reader for execution results, returning once
Strompy yields `None`
"""
async def poll(reader):
    while True:
        res = await reader.next()
        if res is None:
            break
        print(f'Result: {res}')

async def main():
    # Set up a channel
    writer, reader = strompy.channel()
    # Spawn feed and poll_next tasks
    write = asyncio.create_task(feed(writer))
    read = asyncio.create_task(poll(reader))

    # Await both tasks
    await asyncio.gather(write, read)

asyncio.run(main())
