import asyncio
import os
from pydantic.networks import AnyUrl
from mcp.client.session import ClientSession
from mcp.client.stdio import StdioServerParameters, stdio_client


async def main():
    try:
        async with (
            stdio_client(
                StdioServerParameters(
                    command="python",
                    args=[os.path.join(os.path.dirname(__file__), "weather.py")],
                )
            ) as (read, write),
            ClientSession(read, write) as session,
        ):
            try:
                await session.initialize()

                # List available resources
                resources = await session.list_resources()
                print(resources)

                # Get a specific resource
                resource = await session.read_resource(AnyUrl("file:///北京.txt"))
                print(resource)

                # List available tools
                tools = await session.list_tools()
                print(tools)

                # Call the fetch tool
                result = await session.call_tool("get_city_id", {"input": "北京"})
                print(result)

                # Call the fetch tool
                result = await session.call_tool("get_weather", {"input": "101010100"})
                print(result)
            except Exception as e:
                print(f"客户端会话出错: {e}")
    except Exception as e:
        print(f"连接服务器失败: {e}")
        return 1
    return 0


asyncio.run(main())
