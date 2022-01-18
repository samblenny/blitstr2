# blitstr2

multi-lingual string blitter for 1-bit monochrome (sequel to blitstr)


## Text Samples

![small_sampler](doc/small_sampler.png)

![regular_sampler](doc/regular_sampler.png)

![mono_sampler](doc/mono_sampler.png)

![bold_sampler](doc/bold_sampler.png)

![multi_sampler](doc/multi_sampler.png)


## What's New

Compared to the original version of blitstr, blitstr2:
- Supports more languages
- Uses 16x16 pixel size for CJK glyphs instead of 32x32
- Makes it easier for calling code to calculate size of rendered strings
- Ignores multi-codepoint grapheme clusters for modern emoji (instead, there
  is limited support for old-school monochrome bitmap emoji)
- Focuses on an API to look up glyphs from just one typeface at a time


## License

Source code for blitstr2 is dual licensed under the terms of [Apache 2.0](LICENSE-APACHE)
or [MIT](LICENSE-MIT), at your option.

Glyph bitmaps included with blitstr2 have their own copyrights and licenses
(OFL-1.1, public domain, Japanese equivalent of public domain).

See [LEGAL.md](LEGAL.md) for copyright and license details on embedded glyph
bitmaps.
