hmglib
===

## create original raw data

```bash
# edit files/original_chars.txt
editer files/original_chars.txt

# to codepoint text
cat files/original_chars.txt | files/enc_unicode.py > files/original_char_codes.txt
```


## download raw data

```bash
curl 'https://raw.githubusercontent.com/codebox/homoglyph/master/raw_data/char_codes.txt' -o ./files/char_codes.txt
curl 'https://www.unicode.org/Public/security/latest/confusables.txt' -o ./files/confusables.txt
```

## generate data

```bash
files/generate_homoglyph.py
```

## TODO

- [ ] check add homoglyph
  - https://ja.wikipedia.org/wiki/%E3%82%B2%E3%82%A8%E3%82%BA%E6%96%87%E5%AD%97
