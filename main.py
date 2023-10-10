# Views only buttons and selects. 
# Modal only text input, yes

import asyncio
import logging
import os

from cogs.utils import Utils
from cogs.challenge import Challenge

import discord
from discord.ext import commands
from discord.utils import setup_logging

intents = discord.Intents.default()
intents.message_content = True

bot = commands.Bot(command_prefix='!', intents=intents)

class Tews(commands.Cog):
    def __init__(self, bot):
        self.bot = bot
        self._last_member = None

    @commands.Cog.listener()
    async def on_ready(self):
        print(f'We have logged in as {self.bot.user}')

    @commands.Cog.listener()
    async def on_message(self, message):
        if message.author == bot.user:
            return

async def main():
    token = os.environ["DISCORD_TOKEN"]
    setup_logging(level=logging.INFO, root=False)
    async with bot:
        await bot.add_cog(Tews(bot))
        await bot.add_cog(Utils(bot))
        await bot.add_cog(Challenge(bot))

        await bot.start(token)

if __name__ == "__main__":
    asyncio.run(main())
