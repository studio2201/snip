# Threat model — snip

## 1. Adversary
An attacker who can craft an LLM-generated diff that bypasses static rules
(regex, secret-scan, PII-pattern match) while still exfiltrating data via a
novel encoding. The attacker controls the prompt or the model output that
produces the diff, and is betting that the static scanner will not
recognize the encoding they chose.

## 2. Trust boundaries
We trust: the structural diff (added/removed/context lines), the
known-shape pattern library (regexes, secret formats, PII patterns). We
do not trust: the *semantics* of what the diff does — Snip is a static
checker, not an interpreter. A diff that exfiltrates data via a novel
encoding is by definition out of the static checker's reach.

## 3. Out of scope
Snip does not defend against: runtime-exfiltration via the program the
diff modifies (Snip does not run the program), exfiltration via
out-of-band channels (DNS tunneling, HTTP request smuggling in
unrelated files), and exfiltration that requires *combining* two diffs
across two separate commits (Snip checks one diff at a time).

## 4. Residual risk
Snip's pattern library is finite; novel encodings are infinite. An
attacker who is willing to invest in a custom encoding (e.g., emoji
substitution, homoglyph swap, multi-layer Base64) will defeat Snip's
static checks until the pattern library is updated. The buyer is
accepting "best-effort static detection, not a guarantee against
LLM-targeted exfiltration."
