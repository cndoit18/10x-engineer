import torch
from torch import nn


class SelfAttention(nn.Module):
    def __init__(self, d_in, d_out, qkv_bias=False):
        super().__init__()
        self.w_query = nn.Linear(d_in, d_out, bias=qkv_bias)
        self.w_key = nn.Linear(d_in, d_out, bias=qkv_bias)
        self.w_value = nn.Linear(d_in, d_out, bias=qkv_bias)

    def forward(self, x):
        keys = self.w_key(x)
        queries = self.w_query(x)
        values = self.w_value(x)
        attn_scores = queries @ keys.T
        attn_weights = torch.softmax(attn_scores / keys.shape[-1] ** 0.5, dim=-1)
        context_vec = attn_weights @ values
        return context_vec


if __name__ == "__main__":
    inputs = torch.tensor(
        [
            [0.43, 0.15, 0.89],  # Your
            [0.55, 0.87, 0.66],  # journey
            [0.57, 0.85, 0.64],  # starts
            [0.22, 0.58, 0.33],  # with
            [0.77, 0.25, 0.10],  # one
            [0.05, 0.80, 0.55],  # step
        ]
    )

    query = inputs[1]
    attn_scores_2 = torch.empty(inputs.shape[0])
    for i, x_i in enumerate(inputs):
        attn_scores_2[i] = torch.dot(x_i, query)

    print(attn_scores_2)

    def softmax_native(x):
        return torch.exp(x) / torch.exp(x).sum(dim=0)

    attn_weights_2_native = softmax_native(attn_scores_2)

    print("Attention weights:", attn_weights_2_native)
    print("Sum:", attn_weights_2_native.sum())

    context_vec_2 = torch.zeros(query.shape)
    for i, x_i in enumerate(inputs):
        context_vec_2 += attn_weights_2_native[i] * x_i
    print(context_vec_2)

    attn_scores = torch.empty(inputs.shape[0], inputs.shape[0])
    for i, x_i in enumerate(inputs):
        for j, x_j in enumerate(inputs):
            attn_scores[i][j] = torch.dot(x_j, x_i)
    # attn_scores = inputs @ inputs.T
    print(attn_scores)
    attn_weights = torch.softmax(attn_scores, dim=-1)
    print(attn_weights)
    all_context_vecs = attn_weights @ inputs
    print(all_context_vecs)

    x_2 = inputs[1]
    d_in = inputs.shape[1]
    d_out = 2

    torch.manual_seed(123)
    w_query = torch.nn.Parameter(torch.rand(d_in, d_out), requires_grad=False)
    w_key = torch.nn.Parameter(torch.rand(d_in, d_out), requires_grad=False)
    w_value = torch.nn.Parameter(torch.rand(d_in, d_out), requires_grad=False)

    query_2 = x_2 @ w_query
    key_2 = x_2 @ w_key
    value_2 = x_2 @ w_value
    print(query_2)
    attn_score_22 = query_2.dot(key_2)
    print(attn_score_22)

    keys = inputs @ w_key
    values = inputs @ w_value
    print("keys.shape:", keys.shape)
    print("values.shape:", values.shape)

    attn_scores_2 = query_2 @ keys.T
    print(attn_scores_2)
    d_k = keys.shape[-1]
    attn_weights_2 = torch.softmax(attn_scores_2 / d_k**0.5, dim=-1)
    print(attn_weights_2)

    context_vec_2 = attn_weights_2 @ values
    print(context_vec_2)

    torch.manual_seed(789)
    sa = SelfAttention(d_in, d_out)
    print(sa(inputs))
