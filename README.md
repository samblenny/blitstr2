# blitstr2

multi-lingual string blitter for 1-bit monochrome (sequel to blitstr)


## What's New

Compared to the original version of blitstr, blitstr2:
- Supports more languages
- Uses 16x16 pixel size for CJK glyphs instead of 32x32
- Makes it easier for calling code to calculate size of rendered strings
- Ignores multi-codepoint grapheme clusters for modern emoji (instead, there
  is limited support for old-school monochrome bitmap emoji)
- Focuses on an API to look up glyphs from just one typeface at a time


## License

Dual licensed under the terms of [Apache 2.0](LICENSE-APACHE) or
[MIT](LICENSE-MIT), at your option.
