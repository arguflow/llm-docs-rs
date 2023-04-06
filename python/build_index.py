"""Build index for ratchet_core docs"""
import os
from dotenv import load_dotenv
from langchain.agents import Tool, initialize_agent
from langchain.chains.conversation.memory import ConversationBufferMemory
from langchain.chat_models import ChatOpenAI
from llama_index import GPTSimpleVectorIndex, download_loader


def get_env():
    """Load environment variables from .env file"""
    load_dotenv()
    os.getenv("OPENAI_API_KEY")


def build_and_save_index(crate_name: str):
    """Build and save index for ratchet_core docs"""
    simple_directory_loader = download_loader("SimpleDirectoryReader")

    documents = simple_directory_loader(
        input_dir=f"./embedding-docs/{crate_name}",
        recursive=True,
        file_extractor={
            ".html": "UnstructuredReader",
        },
    ).load_data()

    index = GPTSimpleVectorIndex.from_documents(documents)
    index.save_to_disk(f"./embedding-indices/{crate_name}_index.json")


def load_index(crate_name: str):
    """Load index for ratchet_core docs"""
    index = GPTSimpleVectorIndex.load_from_disk(
        f"./embedding-indices/{crate_name}_index.json"
    )
    return index


def build_chat_agent(crate_name: str):
    """Build chat agent for ratchet_core docs"""
    index = load_index(crate_name)
    memory = ConversationBufferMemory(memory_key=f"{crate_name}_memory")
    llm = ChatOpenAI(temperature=0.3)
    tools = [
        Tool(
            name=f"{crate_name} docs",
            func=lambda q: str(index.query(q)),
            description=f"Useful for when you need to answer questions about {crate_name} - the rust crate in context -  docs. The input to this tool should be a complete english sentence.\n",
            return_direct=True,
        )
    ]
    agent_chain = initialize_agent(
        tools, llm, agent="conversational-react-description", memory=memory
    )
    return agent_chain
