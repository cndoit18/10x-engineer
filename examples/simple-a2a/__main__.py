from common.types import AgentSkill
import click
from common.types import AgentSkill, AgentCapabilities, AgentCard
from common.server import A2AServer
import logging

from task_manager import AgentTaskManager

logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)


@click.command()
@click.option("--host", default="localhost")
@click.option("--port", default=10002)
def main(host, port):
    skill = AgentSkill(
        id="my-project-echo-skill",
        name="Echo Tool",
        description="Echos the input given",
        tags=["echo", "repeater"],
        examples=["I will see this echoed back to me"],
        inputModes=["text"],
        outputModes=["text"],
    )
    logging.info(skill)

    capabilities = AgentCapabilities(streaming=True)
    agent_card = AgentCard(
        name="Echo Agent",
        description="This agent echos the input given",
        url=f"http://{host}:{port}/",
        version="0.1.0",
        defaultInputModes=["text"],
        defaultOutputModes=["text"],
        capabilities=capabilities,
        skills=[skill],
    )
    logging.info(agent_card)

    task_manager = AgentTaskManager()
    server = A2AServer(
        agent_card=agent_card,
        task_manager=task_manager,
        host=host,
        port=port,
    )
    server.start()


if __name__ == "__main__":
    main()
