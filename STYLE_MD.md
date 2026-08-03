<!-- SPDX-License-Identifier: CC-BY-3.0 -->

# Google Markdown Style Guide

Source: <https://google.github.io/styleguide/docguide/style.html>

© Google. This document is reproduced from the
[Google Style Guide](https://google.github.io/styleguide/) and is licensed
under [Creative Commons Attribution 3.0 Unported (CC BY 3.0)](https://creativecommons.org/licenses/by/3.0/),
not the CC0-1.0 dedication that covers the rest of this repository.

## Overview

This guide emphasizes three core goals:

1. Source text is readable and portable.
2. The Markdown corpus is maintainable over time and across teams.
3. The syntax is simple and easy to remember.

## Minimum Viable Documentation

Prioritize a small set of fresh and accurate docs over sprawling collections
in various states of disrepair. Focus on what truly matters — release
documentation, API references, testing guidelines — and remove outdated
material regularly.

## Better Is Better Than Best

Documentation reviews differ from code reviews. Authors must stay productive
when implementing improvements. Reviewers should request enhancements but
recognize the "Better/Best Rule," allowing quick iterations rather than
demanding perfection.

### Reviewer Guidance

- Approve promptly when reasonable, trusting follow-up corrections.
- Suggest alternatives rather than vague criticism.
- Submit separate CLs for substantial changes.
- Only block submissions when documentation becomes worse.

### Author Guidance

- Avoid trivial disputes; concede points and progress.
- Reference the Better/Best Rule as needed.

## Capitalization

Maintain original product, tool, and binary names with their proper
capitalization. Example: capitalize `Markdown` when referencing the
platform, not "markdown."

## Document Layout

Standard structure includes:

```
# Document Title

Short introduction.

[TOC]

## Topic

Content.

## See also

* https://link-to-more-info
```

Each component serves a purpose:

- **Title (H1)**: Should match the filename; becomes the page title tag.
- **Introduction**: 1-3 sentences explaining the topic's basics.
- **[TOC]**: Table of contents directive (if supported by hosting).
- **Headings (H2+)**: Content sections.
- **See also**: References for deeper exploration.

## Table of Contents

Place `[TOC]` after the introduction and before the first H2 heading. This
placement matters for screen readers, which process the DOM sequentially
rather than visually.

## Character Line Limit

Follow an 80-character line limit, consistent with coding standards. This
supports tooling integration and maintains quality through familiar
engineering habits.

### Exceptions

Lines containing links, tables, headings, and code blocks may exceed 80
characters.

## Trailing Whitespace

Avoid trailing whitespace entirely. Use a backslash for line breaks instead:

```
For some reason I just really want a break here,\
though it's probably not necessary.
```

Two trailing spaces per CommonMark create `<br />` tags, but most systems
filter this via presubmit checks.

## Headings

### ATX-Style Format

Use hash symbols for all headings:

```
# Heading 1
## Heading 2
```

Avoid underline styles (`=` or `-`); they're difficult to maintain and
create ambiguity.

### Unique, Complete Names

Each heading should be descriptive and distinct, even sub-sections. This
ensures automatically-generated anchor links are intuitive:

**Avoid:**

```
## Foo
### Summary
### Example
## Bar
### Summary
### Example
```

**Prefer:**

```
## Foo
### Foo summary
### Foo example
## Bar
### Bar summary
### Bar example
```

### Spacing Standards

Include a space after `#` and blank lines before/after headings:

```
...text before.

## Heading 2

Text after...
```

### Single H1 Heading

Use exactly one H1 as your document's main title. Subsequent headings begin
at H2.

### Title Capitalization

Follow the Google Developer Documentation Style Guide's "capitalization in
titles and headings" rules.

## Lists

### Lazy Numbering

For long, changeable lists, use "lazy" numbering (repeating "1." for all
items):

```
1.  Foo.
1.  Bar.
    1.  Foofoo.
    1.  Barbar.
1.  Baz.
```

For short, static lists, use sequential numbering for readability in
source.

### Nested List Spacing

Use 4-space indentation for nested content in both numbered and bulleted
lists:

```
1.  Use 2 spaces after the item number.
    Use a 4-space indent for wrapped text.
2.  Next item.

*   Use 3 spaces after a bullet.
    Use a 4-space indent for wrapped text.
    1.  Nested numbered list.
        Wrapped text needs 8-space indent.
    2.  Next nested item.
*   Back to bullets.
```

For single-line, non-nested lists, one space suffices.

## Code

### Inline Code

Use backticks for inline code references, field names, and file types:

```
Run `really_cool_script.sh arg`.
Check the `foo_bar_whammy` field.
Update your `README.md`!
```

### Code Span for Escaping

Wrap text in backticks to prevent Markdown processing (fake paths, example
URLs):

```
Example shortlink: `Markdown/foo/Markdown/bar.md`
Example query: `https://www.google.com/search?q=$TERM`
```

### Fenced Code Blocks

Use triple backticks with a language declaration:

````
```python
def Foo(self, bar):
  self.bar = bar
```
````

**Language declaration**: Always specify the language for syntax
highlighting and clarity.

**Avoid indented code blocks**: Four-space indenting works but prevents
language specification, creates ambiguous block boundaries, and complicates
searching.

**Escape newlines**: Use backslashes to break long command-line snippets
for copy-paste readiness:

````
```shell
$ bazel run :target -- --flag --foo=longlonglonglonglongvalue \
  --bar=anotherlonglonglonglonglonglonglonglonglonglongvalue
```
````

**Nested code blocks in lists**: Indent code blocks to maintain list
structure:

```
*   Bullet.

    ```c++
    int foo;
    ```

*   Next bullet.
```

Or use 4-space indentation (8 total from document edge):

```
*   Bullet.

        int foo;

*   Next bullet.
```

## Links

Long links make source Markdown difficult to read and break the 80
character wrapping. Shorten links wherever possible.

### Explicit Paths

Use explicit paths for internal Markdown links:

```
[...](/path/to/other/markdown/page.md)
```

Avoid full URLs when internal paths work.

### Avoid Relative Paths with `../`

Relative paths within the same directory are safe. Avoid paths traversing
multiple directories:

**Avoid:**

```
[...](../../bad/path/to/another/dir/other-page.md)
```

**Prefer:**

```
[...](/path/to/another/dir/other-page.md)
```

### Informative Link Titles

Write naturally, then wrap the most relevant phrase with the link:

**Avoid:**

```
See the guide for info: [link](markdown.md), or [here](/style.html).
Check [https://example.com/foo/bar](https://example.com/foo/bar).
```

**Prefer:**

```
See the [Markdown guide](markdown.md) for more info, or check the
[style guide](/style.html).
Check out a [typical test result](https://example.com/foo/bar).
```

### Reference Links

#### Use for Long Links

Reference links suit lengthy URLs that disrupt text flow:

```
The [style guide] says not to use reference links unless necessary.

[style guide]: https://docs.google.com/document/d/13HQBxfhCwx8lVRuN2Wf6poqvAfVeEXmFVcawP5I6B3c/edit
```

Short links should remain inline.

#### Reduce Duplication

Reference the same link multiple times using consistent labels:

```
First mention of [example][ref].
Later mention of [example][ref].

[ref]: https://example.com
```

#### Define After First Use

Place reference link definitions just before the next heading (at section
end). Exception: links used across multiple sections go at the document's
end.

**Poor placement:**

```
# Header

Some text with a [link][link_def].

(lots of intervening content)

[link_def]: http://reallyreallyreallylonglink.com
```

**Better placement:**

```
# Header

Some text with a [link][link_def].

[link_def]: http://reallyreallyreallylonglink.com

## Next Header

(content continues)
```

## Images

Use images sparingly; plain text gets users down to the business of
communication faster.

**When to use images:**

- Showing is easier than describing (UI navigation, visual concepts).
- Always provide descriptive alt text for accessibility.

**Avoid:**

- Excessive images causing reader distraction.
- Missing alt text that excludes sighted-impaired readers.

## Tables

Use tables for tabular data requiring quick scanning. Avoid tables when
lists work better.

**Poor table example:** Many empty cells, unbalanced dimensions, rambling
prose.

**When tables excel:**

- Relatively uniform data across two dimensions.
- Many parallel items with distinct attributes.
- Compact presentation improves readability.

**Example — good table:**

```
Transport        | Favored by     | Advantages
---------------- | -------------- | -----------------------------------------------
Swallow          | Coconuts       | [Fast when unladen][airspeed]
Bicycle          | Miss Gulch     | [Weatherproof][tornado_proofing]
X-34 landspeeder | Whiny farmboys | [Cheap][tosche_station] since the XP-38 came out

[airspeed]: http://google3/airspeed.h
[tornado_proofing]: http://google3/kansas/
[tosche_station]: http://google3/power_converter.h
```

Use reference links in tables to keep cells manageable.

## Markdown vs. HTML

Strongly prefer Markdown to standard syntax wherever possible and avoid
HTML hacks. HTML reduces readability and portability. Markdown handles
nearly all needs without HTML workarounds.
