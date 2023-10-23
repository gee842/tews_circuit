import os
import asyncio
import logging
import threading

from cogs.utils import Utils
from cogs.challenge import Challenge
from cogs.booking import Booking

import discord
from discord.ext import commands
from discord.utils import setup_logging

from database import verify_database
from database.challenge import disqualifications

async def process_disqualifications():
    while True:
        await asyncio.sleep(60 * 5)
        await disqualifications()

def start_loop(loop):
    asyncio.set_event_loop(loop)
    loop.run_until_complete(process_disqualifications())

intents = discord.Intents.default()
intents.message_content = True

bot = commands.Bot(command_prefix="!", intents=intents)


class Tews(commands.Cog):
    def __init__(self, bot):
        self.bot = bot
        self._last_member = None

    @commands.Cog.listener()
    async def on_ready(self):
        print(f"{self.bot.user} is active.")

    @commands.Cog.listener()
    async def on_message(self, message):
        if message.author == bot.user:
            return


async def main():
    await verify_database()

    token = os.environ["DISCORD_TOKEN"]
    setup_logging(level=logging.INFO, root=False)

    async with bot:
        await bot.add_cog(Tews(bot))
        await bot.add_cog(Utils(bot))
        await bot.add_cog(Challenge(bot))
        await bot.add_cog(Booking(bot))

        loop = asyncio.new_event_loop()
        thread = threading.Thread(target=start_loop, args=(loop,))
        thread.start()

        await bot.start(token)


if __name__ == "__main__":
    asyncio.run(main())
