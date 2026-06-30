---
name: youtube-transcribe-translate
argument-hint: "<input> --to ja|en [--from ja|en] [--format md|txt] [--output PATH]"
description: >-
  Translate a transcript or Markdown article between English and Japanese using Claude.
  Preserves Markdown structure, code, URLs, and technical terms. Output is a language-tagged
  file next to the source (e.g. talk.md + --to ja → talk.ja.md).
  Trigger whenever the user wants to translate a transcript or article — "translate this to
  Japanese", "translate to English", "EN->JA", "JA->EN", "日本語に訳して",
  "make a Japanese version of this article", "give me the English version" — and especially
  right after a transcript or article was produced by youtube-transcribe-transcribe-youtube
  and the user accepts a translation offer, even without saying the word "translate"
  (e.g. they reply "yes, Japanese" or "do the JP one").
  The --to flag is required; Claude prompts the user to specify it when omitted, rather than
  guessing — short and mixed-language texts mis-detect frequently.
context: fork
---

# youtube-transcribe-translate

Translate a transcript or article between English and Japanese. You (Claude) do the translation
in this forked context — no subprocess, no external MT tool. Read the input, translate
according to the rules below, write the output file, and reply with `wrote <path>`.

## Invocation

The user's message will look like:

```
translate <input> [--to ja|en] [--from ja|en] [--format md|txt] [--output PATH]
```

Parse the arguments. Defaults:

- `--to` — **required**. If omitted, do not guess: ask the user which target language they want.
  Short and mixed-language texts mis-detect; explicit direction is the safer default.
- `--from` — usually unnecessary; detect the source from the text.
- `--format` — follows the input format (`md` in → `md` out).
- Output path — language-tagged stem next to the source:
  `talk.md` + `--to ja` → `talk.ja.md`; `talk.en.md` + `--to ja` → `talk.en.ja.md`.
  Honor `--output` when supplied.

## Steps

1. **Read the input** with the Read tool. Transcripts can be large — read fully.
2. **Determine source and target languages** from flags or detection.
3. **Determine output path** (language-tagged default or `--output`).
   If the destination is sandbox-blocked, fall back to `$TMPDIR/<basename>` and
   announce the fallback: `wrote /tmp/... (couldn't write next to input: permission denied)`.
4. **Translate** using the rules below.
5. **Sanity check**: a faithful translation carries the same information density as the source.
   If the output is a small fraction of the input, something went wrong — stop and tell the user.
6. **Write** with the Write tool.
7. **Reply** with exactly `wrote <path>`. Do not paste the translation into the reply.

## Translation rules

Produce a translation the reader experiences as if the document had been written originally
in the target language by someone who fully understood it.

- **Faithful and complete.** Carry over every idea, example, and conclusion. No summarisation,
  dropped asides, or added explanations the source didn't have.
- **Natural in the target language.** Translate meaning, not surface syntax. Reorder sentences,
  split or join clauses, choose idiom — whatever reads fluently to a native reader.
- **Preserve document structure exactly.** Heading levels, list nesting, bold/italic, blockquotes,
  and tables carry meaning — translate only the text inside them.
- **Leave code and machine-readable tokens verbatim.** Fenced code blocks, inline `code`, URLs,
  file paths, command names, flags, and identifiers must not be translated. Natural-language
  comments inside code blocks may be translated.
- **Technical terms.** Use the term a practitioner in the target language actually uses:
  sometimes the borrowed English loanword, sometimes the established native equivalent.
  Accuracy for the domain reader beats dictionary literalness.
- **Names.** Product names (Anthropic, Claude, OpenAI) stay in their original Latin script in
  Japanese, or take the established katakana only where that is the conventional form.
  Person names stay as themselves; transliterate only when that is the clear convention.
- **Speaker labels.** Preserve `Alice:`, `Speaker 1:`, etc. — they carry attribution.

## Format: `--format txt`

When the input is plain text or `--format txt` is passed, produce plain prose with paragraph
breaks only. The translation rules still apply; only the Markdown markup is dropped.

## Examples

```
translate ~/Downloads/Talk Title [abc123].md --to ja
# wrote ~/Downloads/Talk Title [abc123].ja.md

translate ~/Downloads/keynote.md
# (source detected as Japanese) → wrote ~/Downloads/keynote.en.md

translate ~/Downloads/talk.srt --to ja --format txt --output ~/Downloads/talk-ja.txt
# wrote ~/Downloads/talk-ja.txt
```
