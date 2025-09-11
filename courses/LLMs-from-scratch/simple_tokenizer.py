import re


class SimpleTokenizer:
    def __init__(self, vocab):
        self.str_to_int = vocab
        self.int_to_str = {i: s for s, i in vocab.items()}

    def encode(self, text):
        preprocessed = re.split(r'([,.:;?_!"()\']|--|\s)', text)
        preprocessed = [item.strip() for item in preprocessed if item.strip()]
        return [
            self.str_to_int.get(s, self.str_to_int["<|unk|>"]) for s in preprocessed
        ]

    def decode(self, ids):
        text = " ".join([self.int_to_str[i] for i in ids])
        text = re.sub(r'\s+([,.:;?_!"()\'])', r"\1", text)
        return text


if __name__ == "__main__":
    with open("the-verdict.txt", "r", encoding="utf-8") as f:
        raw_text = f.read()
    print("Total number of character:", len(raw_text))

    preprocessed = re.split(r'([,.:;?_!"()\']|--|\s)', raw_text)
    preprocessed = [item.strip() for item in preprocessed if item.strip()]
    print(preprocessed[:99])

    all_tokens = sorted(set(preprocessed))
    all_tokens.extend(["<|endoftext|>", "<|unk|>"])

    vocab_size = len(all_tokens)
    print(vocab_size)
    vocab = {token: integer for integer, token in enumerate(all_tokens)}

    print(list(vocab.items())[:50])

    tokenizer = SimpleTokenizer(vocab)
    text1 = "Hello, do you likke tea"
    text2 = """"It's the last he painted, you know, "
    Mrs. Gisburn said with pardonable pride."""
    ids = tokenizer.encode(" <|endoftext|> ".join((text1, text2)))
    print(ids)
    text = tokenizer.decode(ids)
    print(text)
