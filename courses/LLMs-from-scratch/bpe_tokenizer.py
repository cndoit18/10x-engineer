import tiktoken

tokenizer = tiktoken.get_encoding("gpt2")
text1 = "Hello, do you likke tea"
text2 = """"It's the last he painted, you know, "
Mrs. Gisburn said with pardonable pride."""
ids = tokenizer.encode(
    " <|endoftext|> ".join((text1, text2)), allowed_special={"<|endoftext|>"}
)
print(ids)
text = tokenizer.decode(ids)
print(text)
