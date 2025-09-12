import tiktoken
import torch

from torch.utils.data import Dataset, DataLoader


class GPTDataset(Dataset):
    def __init__(self, txt, tokenizer, max_length, stride):
        self.input_ids = []
        self.target_ids = []
        token_idx = tokenizer.encode(txt)
        for i in range(0, len(token_idx) - max_length, stride):
            input_chunk = token_idx[i : i + max_length]
            target_chunk = token_idx[i + 1 : i + max_length + 1]
            self.input_ids.append(torch.tensor(input_chunk))
            self.target_ids.append(torch.tensor(target_chunk))

    def __len__(self):
        return len(self.input_ids)

    def __getitem__(self, index):
        return self.input_ids[index], self.target_ids[index]


def create_dataloader(
    txt,
    batch_size=4,
    max_length=256,
    stride=128,
    shuffle=True,
    drop_last=True,
    num_workers=0,
):
    tokenizer = tiktoken.get_encoding("gpt2")
    dataset = GPTDataset(txt, tokenizer, max_length, stride)
    dataloader = DataLoader(
        dataset,
        batch_size=batch_size,
        shuffle=shuffle,
        drop_last=drop_last,
        num_workers=num_workers,
    )
    return dataloader


if __name__ == "__main__":
    with open("the-verdict.txt") as f:
        raw_text = f.read()

    tokenizer = tiktoken.get_encoding("gpt2")
    enc_text = tokenizer.encode(raw_text)
    print(len(enc_text))

    context_size = 10
    for i in range(1, context_size + 1):
        context = enc_text[:i]
        desired = enc_text[i]
        print(tokenizer.decode(context), "---->", tokenizer.decode([desired]))

    dataloader = create_dataloader(
        raw_text, batch_size=1, max_length=4, stride=1, shuffle=False
    )
    data_iter = iter(dataloader)
    first_batch = next(data_iter)
    print(first_batch)

    vocab_size = tokenizer.n_vocab
    output_dim = 256
    token_embedding_layer = torch.nn.Embedding(vocab_size, output_dim)
    dataloader = create_dataloader(
        raw_text, batch_size=8, max_length=4, stride=4, shuffle=False
    )

    data_iter = iter(dataloader)
    inputs, targets = next(data_iter)
    print("Token IDs:\n", inputs)
    print("\nInputs shape:\n", inputs.shape)
    token_embeddings = token_embedding_layer(inputs)
    print(token_embeddings.shape)

    pos_embedding_layer = torch.nn.Embedding(4, output_dim)
    pos_embeddings = pos_embedding_layer(torch.arange(4))
    print(pos_embeddings.shape)

    input_embeddings = token_embeddings + pos_embeddings
    print(input_embeddings.shape)
