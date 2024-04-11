import hello_py
import datetime
import asyncio

async def main():
   await hello_py.print_sleep(datetime.timedelta(seconds=3))

asyncio.run(main())