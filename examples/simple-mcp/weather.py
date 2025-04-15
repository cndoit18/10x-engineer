from mcp.server.lowlevel import Server
import asyncio
import anyio
import mcp.types as types
import json
from pydantic import FileUrl, AnyUrl
from urllib.parse import unquote
import os
import sys
import httpx

BASE_API = "http://t.weather.sojson.com/"
with open(os.path.join(os.path.dirname(__file__), "cities.json"), "r") as f:
    CITIES = json.load(f)


def main() -> int:
    app = Server("weather")

    @app.list_resources()
    async def list_resources() -> list[types.Resource]:
        return [
            types.Resource(
                uri=FileUrl(f"file:///{name}.txt"),
                name=name,
                mimeType="text/plain",
            )
            for name in CITIES
        ]

    @app.read_resource()
    async def read_resource(uri: AnyUrl) -> str | bytes:
        name = unquote(uri.path.replace(".txt", "").lstrip("/"), "utf-8")

        if name not in CITIES:
            raise ValueError(f"Unknown resource: {uri}")

        return str(CITIES[name])

    async def fetch_website(
        id: str,
    ) -> list[types.TextContent | types.ImageContent | types.EmbeddedResource]:
        headers = {"User-Agent": "MCP Weather"}
        async with httpx.AsyncClient(follow_redirects=True, headers=headers) as client:
            response = await client.get(f"{BASE_API}/api/weather/city/{id}")
            response.raise_for_status()
            return [types.TextContent(type="text", text=response.text)]

    @app.call_tool()
    async def fetch_tool(
        name: str, arguments: dict
    ) -> list[types.TextContent | types.ImageContent | types.EmbeddedResource]:
        tools = {
            "get_city_id": lambda city: [
                types.TextContent(
                    type="text",
                    text=str(CITIES.get(city, "未找到该城市"))
                    if city in CITIES
                    else "未找到该城市",
                )
            ],
            "get_weather": lambda city_id: fetch_website(city_id),
        }
        if name not in tools:
            raise ValueError(f"Unknown tool: {name}")
        result = tools[name](arguments["input"])
        return result if not asyncio.iscoroutine(result) else await result

    @app.list_tools()
    async def list_tools() -> list[types.Tool]:
        return [
            types.Tool(
                name="get_weather",
                description="获取城市的天气，只支持中国地区",
                inputSchema={
                    "type": "object",
                    "required": ["input"],
                    "properties": {
                        "input": {
                            "type": "string",
                            "description": """只能输入城市的id，如北京为101010100""",
                        }
                    },
                },
            ),
            types.Tool(
                name="get_city_id",
                description="获取城市的id，只支持中国地区",
                inputSchema={
                    "type": "object",
                    "required": ["input"],
                    "properties": {
                        "input": {
                            "type": "string",
                            "description": """城市的名字，只支持中国地区。
                    不要添加行政区后缀，如北京市，只需要输入北京。""",
                        }
                    },
                },
            ),
        ]

    from mcp.server.stdio import stdio_server

    async def arun():
        async with stdio_server() as streams:
            await app.run(streams[0], streams[1], app.create_initialization_options())

    anyio.run(arun)
    return 0


if __name__ == "__main__":
    sys.exit(main())
