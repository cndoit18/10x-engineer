from torch.utils.data import Dataset, DataLoader
import tiktoken


class GPTDataset(Dataset):
    def __init__(self, txt, tokenizer, max_length, stride):
        self._chunks = []
        token_ids = tokenizer.encode(txt)
        for i in range(0, len(token_ids) - max_length, stride):
            input_chunk = tuple(token_ids[i : i + max_length])
            output_chunk = tuple(token_ids[i + 1 : i + max_length + 1])
            self._chunks.append((input_chunk, output_chunk))

    def __len__(self):
        return len(self._chunks)

    def __getitem__(self, index):
        return self._chunks[index]


def create_dataloader(
    text,
    batch_size=4,
    max_length=256,
    stride=128,
    shuffle=True,
    drop_last=True,
    num_workers=0,
):
    return DataLoader(
        GPTDataset(
            text, tiktoken.get_encoding("gpt2"), max_length=max_length, stride=stride
        ),
        batch_size=batch_size,
        shuffle=shuffle,
        drop_last=drop_last,
        num_workers=num_workers,
    )
