from aiogram import Bot, Dispatcher
from aiogram.types import InlineQuery, InlineQueryResultArticle, InputTextMessageContent
from requests import get
import os
import asyncio
import hashlib

bot = Bot(token=os.environ["BOT_TOKEN"])
dp = Dispatcher()
domain_statuses = {
    "gfbrowser.com/youtube": "available",
    "downexample.com/youtube": "down",
}


async def update_domain_statuses() -> None:
    global domain_statuses
    domain_statuses = get("http://127.0.0.1:7777").json()


async def original_link_to_alt_domain(alt_domain, original_link) -> str:
    return f"https://{alt_domain}/{original_link.split('/')[-1]}"


async def generate_result_article(
    alt_domain, original_link
) -> InlineQueryResultArticle:
    try:
        alt_link = await original_link_to_alt_domain(alt_domain, original_link)

        return InlineQueryResultArticle(
            id=hashlib.md5(f"{alt_domain}:{original_link}".encode()).hexdigest(),
            title=alt_link,
            url=alt_link,
            input_message_content=InputTextMessageContent(message_text=alt_link),
        )

    except Exception as e:
        print(e)
        pass


@dp.inline_query()
async def inline_query_handler(query: InlineQuery) -> None:
    if query:
        query_result_array: list[InlineQueryResultArticle] = [
            await generate_result_article(alt_domain, query.query)
            for alt_domain in domain_statuses.keys()
            if domain_statuses[alt_domain] == "available"
        ]
        print(query_result_array)

        await query.answer(results=query_result_array)


async def main() -> None:
    await dp.start_polling(bot)


if __name__ == "__main__":
    asyncio.run(main())