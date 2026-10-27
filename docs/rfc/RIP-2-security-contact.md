# RIP-2: Security contact for this fork

Status: draft

## Summary

Name where a vulnerability in this repository's code or prerelease binaries
should be reported. Stop sending those reports to the upstream author.

## Motivation

`README.md` tells readers to contact Andrew Gallant. He maintains
BurntSushi/ripgrep. This fork builds and publishes its own `rg` binaries from
`.github/workflows/release.yml`. A report about those binaries, or about a
change that exists only here, does not belong in his inbox. There is no
replacement address in the tree.

## Design

Turn on private vulnerability reporting for `laputa-systems/rg` and point
the README's vulnerability section at
`https://github.com/laputa-systems/rg/security/advisories/new`. Delete the
upstream contact from that section.

## Alternatives considered

- Leave the upstream contact. Reports for this fork go to someone who does
  not ship it.
- Accept reports as public GitHub issues. The report is public before a fix
  exists.
- Publish a personal email address in the README. That works, and it rots
  when the person changes. Prefer the GitHub advisory form unless the owners
  already have an address they will read.

## Open questions

- Who is allowed to see advisories on `laputa-systems/rg`?
- Is private vulnerability reporting already enabled on the repository?

## Decision

Unset. The recommendation is GitHub private advisories for this repository.
