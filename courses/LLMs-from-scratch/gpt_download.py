from gpt import GPTModel
from pre_training import generate, text_to_token_ids, token_ids_to_text
import torch
import tiktoken
import os
import urllib.request

if __name__ == "__main__":
    GPT_CONFIG_124M = {
        "vocab_size": 50257,
        "context_length": 256,
        "emb_dim": 768,
        "n_heads": 12,
        "n_layers": 12,
        "drop_rate": 0.1,
        "qkv_bias": False,
    }

    tokenizer = tiktoken.get_encoding("gpt2")
    checkpoint = torch.load("model_and_optimizer.pth", weights_only=True)
    torch.manual_seed(123)
    model = GPTModel(GPT_CONFIG_124M)
    device = torch.device("cuda" if torch.cuda.is_available() else "cpu")
    model.to(device)
    model.load_state_dict(checkpoint["model_state_dict"])
    model.eval()

    optimizer = torch.optim.AdamW(model.parameters(), lr=0.0005, weight_decay=0.1)
    optimizer.load_state_dict(checkpoint["optimizer_state_dict"])
    model.train()
    token_ids = generate(
        model=model,
        idx=text_to_token_ids("Every effort moves you", tokenizer),
        max_new_tokens=15,
        context_size=GPT_CONFIG_124M["context_length"],
        top_k=25,
        temperature=1.4,
    )

    print("Output text:\n", token_ids_to_text(token_ids, tokenizer))

    file_name = "gpt2-small-124M.pth"
    # file_name = "gpt2-medium-355M.pth"
    # file_name = "gpt2-large-774M.pth"
    # file_name = "gpt2-xl-1558M.pth"

    url = f"https://huggingface.co/rasbt/gpt2-from-scratch-pytorch/resolve/main/{file_name}"

    if not os.path.exists(file_name):
        urllib.request.urlretrieve(url, file_name)
        print(f"Downloaded to {file_name}")
    GPT_CONFIG_BASE = {
        "vocab_size": 50257,  # Vocabulary size
        "context_length": 1024,  # Original context length
        "emb_dim": 768,  # Embedding dimension
        "n_heads": 12,  # Number of attention heads
        "n_layers": 12,  # Number of layers
        "drop_rate": 0.0,  # Dropout rate
        "qkv_bias": True,  # Query-key-value bias
    }
    model_configs = {
        "gpt2-small-124M.pth": {"emb_dim": 768, "n_layers": 12, "n_heads": 12},
        "gpt2-medium-355M.pth": {"emb_dim": 1024, "n_layers": 24, "n_heads": 16},
        "gpt2-large-774M.pth": {"emb_dim": 1280, "n_layers": 36, "n_heads": 20},
        "gpt2-xl-1558M.pth": {"emb_dim": 1600, "n_layers": 48, "n_heads": 25},
    }
    NEW_CONFIG = GPT_CONFIG_BASE.copy()
    NEW_CONFIG.update(model_configs[file_name])
    model = GPTModel(NEW_CONFIG)
    device = torch.device("cuda" if torch.cuda.is_available() else "cpu")
    model.to(device)
    model.load_state_dict(torch.load(file_name, weights_only=True))
    model.eval()
    torch.manual_seed(123)
    token_ids = generate(
        model=model,
        idx=text_to_token_ids("Every effort moves you", tokenizer),
        max_new_tokens=25,
        context_size=NEW_CONFIG["context_length"],
        top_k=50,
        temperature=1.5,
    )
    print("Output text:\n", token_ids_to_text(token_ids, tokenizer))
