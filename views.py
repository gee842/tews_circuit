from discord import ui 

class UserSelect(ui.View):
    user = None

    @ui.select(cls=ui.UserSelect)
    async def challenge_user(self, interaction, select_item):
        self.answer1 = select_item.values
