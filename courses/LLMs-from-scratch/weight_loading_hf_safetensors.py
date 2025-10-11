import tiktoken
from gpt import ModelArgs, GPTModel
from dataclasses import replace
import os
import requests
from safetensors.torch import load_file
import torch
from gpt_train import (
    text_to_token_ids,
    token_ids_to_text,
    generate,
)


def assign(left, right):
    assert left.shape == right.shape, (
        f"Shape mismatch. Left: {left.shape}, Right: {right.shape}"
    )
    return torch.nn.Parameter(right.detach())


def load_weights_into_gpt(gpt, params):
    gpt.emb.pos_emb.weight = assign(gpt.emb.pos_emb.weight, params["wpe.weight"])
    gpt.emb.tok_emb.weight = assign(gpt.emb.tok_emb.weight, params["wte.weight"])

    for b in range(len(gpt.trf_blocks)):
        q_w, k_w, v_w = torch.chunk(params[f"h.{b}.attn.c_attn.weight"], 3, dim=-1)
        gpt.trf_blocks[b].attn.W_query.weight = assign(
            gpt.trf_blocks[b].attn.W_query.weight, q_w.T
        )
        gpt.trf_blocks[b].attn.W_key.weight = assign(
            gpt.trf_blocks[b].attn.W_key.weight, k_w.T
        )
        gpt.trf_blocks[b].attn.W_value.weight = assign(
            gpt.trf_blocks[b].attn.W_value.weight, v_w.T
        )

        q_b, k_b, v_b = torch.chunk(params[f"h.{b}.attn.c_attn.bias"], 3, dim=-1)
        gpt.trf_blocks[b].attn.W_query.bias = assign(
            gpt.trf_blocks[b].attn.W_query.bias, q_b
        )
        gpt.trf_blocks[b].attn.W_key.bias = assign(
            gpt.trf_blocks[b].attn.W_key.bias, k_b
        )
        gpt.trf_blocks[b].attn.W_value.bias = assign(
            gpt.trf_blocks[b].attn.W_value.bias, v_b
        )

        gpt.trf_blocks[b].attn.out_proj.weight = assign(
            gpt.trf_blocks[b].attn.out_proj.weight,
            params[f"h.{b}.attn.c_proj.weight"].T,
        )
        gpt.trf_blocks[b].attn.out_proj.bias = assign(
            gpt.trf_blocks[b].attn.out_proj.bias, params[f"h.{b}.attn.c_proj.bias"]
        )

        gpt.trf_blocks[b].ff.layers[0].weight = assign(
            gpt.trf_blocks[b].ff.layers[0].weight, params[f"h.{b}.mlp.c_fc.weight"].T
        )
        gpt.trf_blocks[b].ff.layers[0].bias = assign(
            gpt.trf_blocks[b].ff.layers[0].bias, params[f"h.{b}.mlp.c_fc.bias"]
        )
        gpt.trf_blocks[b].ff.layers[2].weight = assign(
            gpt.trf_blocks[b].ff.layers[2].weight, params[f"h.{b}.mlp.c_proj.weight"].T
        )
        gpt.trf_blocks[b].ff.layers[2].bias = assign(
            gpt.trf_blocks[b].ff.layers[2].bias, params[f"h.{b}.mlp.c_proj.bias"]
        )

        gpt.trf_blocks[b].norm1.scale = assign(
            gpt.trf_blocks[b].norm1.scale, params[f"h.{b}.ln_1.weight"]
        )
        gpt.trf_blocks[b].norm1.shift = assign(
            gpt.trf_blocks[b].norm1.shift, params[f"h.{b}.ln_1.bias"]
        )
        gpt.trf_blocks[b].norm2.scale = assign(
            gpt.trf_blocks[b].norm2.scale, params[f"h.{b}.ln_2.weight"]
        )
        gpt.trf_blocks[b].norm2.shift = assign(
            gpt.trf_blocks[b].norm2.shift, params[f"h.{b}.ln_2.bias"]
        )

    gpt.final_norm.scale = assign(gpt.final_norm.scale, params["ln_f.weight"])
    gpt.final_norm.shift = assign(gpt.final_norm.shift, params["ln_f.bias"])
    gpt.out_head.weight = assign(gpt.out_head.weight, params["wte.weight"])


if __name__ == "__main__":
    BASE_CONFIG = ModelArgs(
        vocab_size=50257,
        context_length=1024,
        drop_rate=0.0,
        qkv_bias=True,
    )

    model_configs = {
        "gpt2": {"emb_dim": 768, "n_layers": 12, "n_heads": 12, "inter_dim": 3072},
        "gpt2-large": {
            "emb_dim": 1280,
            "n_layers": 36,
            "n_heads": 20,
            "inter_dim": 5120,
        },
        "gpt2-xl": {"emb_dim": 1600, "n_layers": 48, "n_heads": 25, "inter_dim": 6400},
    }

    CHOOSE_MODEL = "gpt2-xl"
    MODEL_CONFIG = replace(BASE_CONFIG, **model_configs[CHOOSE_MODEL])

    url = f"https://huggingface.co/openai-community/{CHOOSE_MODEL}/resolve/main/model.safetensors"
    output_file = f"model-{CHOOSE_MODEL}.safetensors"

    if not os.path.exists(output_file):
        response = requests.get(url, timeout=30)
        response.raise_for_status()
        with open(output_file, "wb") as f:
            f.write(response.content)

    state_dict = load_file(output_file)

    tokenizer = tiktoken.get_encoding("gpt2")
    gpt = GPTModel(MODEL_CONFIG)

    device = torch.device("cuda" if torch.cuda.is_available() else "cpu")
    load_weights_into_gpt(gpt, state_dict)
    gpt.to(device)

    token_ids = generate(
        model=gpt,
        idx=text_to_token_ids("How do you", tokenizer, device=device),
        max_new_tokens=100,
        context_size=MODEL_CONFIG.context_length,
        top_k=15,
        temperature=1.0,
    )
    print("Output text:\n", token_ids_to_text(token_ids, tokenizer))
