# MathCAT Translation Audit Tool

This tool compares YAML rule files from a source language with translated versions to identify translation gaps and formatting issues. It assists translators in ensuring their translations are complete, consistent, and properly formatted.

### 🔍 Detection Capabilities

The tool analyzes rule files to detect the following issues:

* **Missing Rules:** Rules present in the source file but missing in the target translation.
* **Extra Rules:** Rules present in the target translation but absent in the source (flagged as potentially intentional language-specific additions).
* **Untranslated Text:** Detects text keys that still use **lowercase** formatting, indicating they haven't been verified or translated yet.
* **Rule Differences:** Structural changes (match expressions, conditions, variables, or test/replace layout) between the source and target translation.
* **Rule Order:** After the summary, reports the first differing position of shared named rules in each affected file. Missing, extra, and `# audit-ignore` rules do not shift the comparison. Include entries are not checked.
* **Definition Coverage:** Compares literal `definitions.yaml` entries by name and collection kind (`vector`, `set`, or `map`).

After a blank line, add `# audit-ignore` immediately before a rule to suppress
auditing that rule. Existing markers inside a rule block are also supported.

---

### 🔑 Understanding Text Keys & Translation Status

In MathCAT YAML rule files, text output is controlled by specific keys (`t`, `ot`, `ct`). The case of the key indicates the translation status.

**Key Definitions:**
* `t` / `T` (**text**): Simple text output spoken as-is.
* `ot` / `OT` (**open text**): Text spoken at the start of a construct (e.g., "start fraction").
* `ct` / `CT` (**close text**): Text spoken at the end of a construct (e.g., "end fraction").

**The Translation Convention:**
* **Lowercase (`t`, `ot`, `ct`):** Untranslated or unverified text (Needs Review).
* **Uppercase (`T`, `OT`, `CT`):** Translated and verified text.

**Example:**
```yaml
# English Source
- t: "square root"      # lowercase = original English

# Spanish Translation
- T: "raíz cuadrada"    # uppercase = verified translation
```

---

### 📂 File Type Handling

The tool automatically adjusts its matching logic based on the file type:

1.  **Standard Rule Files:**
    * Matches rules based on `name` or `tag` identifiers.
    * *Examples:* `ClearSpeak_Rules.yaml`, `SimpleSpeak_Rules.yaml`, `SharedRules/*.yaml`.
2.  **Unicode Files:**
    * Matches rules based on character/range keys.
    * *Examples:* `unicode.yaml`, `unicode-full.yaml` (keys like `a-z`, `!`, `0-9`).
3.  **Definition Files:**
    * `definitions.yaml` is audited by default and can be selected with `--file definitions.yaml`.
    * Definitions are matched by name. Missing definitions and collection-kind mismatches are issues; target-only definitions are informational.
    * Definition contents are not compared, includes are not resolved, and translation verification is not available for definitions.


---

### ⚙️ Usage & Commands

**Syntax:**
```bash
uv run audit-translations <language> [--file <specific_file>]
uv run audit-translations <language> --source <source-language>
uv run audit-translations --list
uv run audit-translations --rule-coverage

# If running from the repo root, point uv at the project:
uv run --project PythonScripts audit-translations <language>
uv run --project PythonScripts audit-translations <language> --source <source-language>
uv run --project PythonScripts audit-translations --list
uv run --project PythonScripts audit-translations --rule-coverage
```

**Convenience Features:**
* `--list`: Displays all available languages.
  * Region variants are shown as `lang-region` (e.g., `zz-aa`) based on subdirectories under `Rules/Languages/<lang>`.
* `--source`: Sets the source/reference language. Defaults to `en`.
* `--file`: Audits a single specific file instead of the whole directory.
* `--rules-dir`: Override the Rules/Languages directory path.
* `--only`: Filter issue types (comma-separated): `missing`, `untranslated`, `extra`, `diffs`, `all`.
* `--verbose`: Show detailed output including source/target snippets for rule differences.
* `--rule-coverage`: Run the full Rust test suite, generate file- and rule-level pattern coverage under `target/rule-coverage/`, and open the HTML report. File coverage includes files with active rules; empty and include-only files are not separate coverage targets. Expand a file to inspect matched and unmatched rules by name and tag, their hit counts, and hover over a rule to see the tests that matched it. Coverage fractions include percentages. This runs independently of a language audit.
* **Summary Stats:** Provides a statistical summary after every run.

**Examples:**

```bash
# List available languages
uv run audit-translations --list

# Same from repo root
uv run --project PythonScripts audit-translations --list

# Audit all Spanish translation files
uv run audit-translations es

# Audit German translations
uv run audit-translations de

# Compare Norwegian Bokmal against Swedish instead of English
uv run audit-translations nb --source sv

# Audit only a specific file
uv run audit-translations es --file SharedRules/default.yaml

# Audit a regional variant (merges Rules/Languages/de and Rules/Languages/de/CH)
uv run audit-translations de-CH

# Show detailed output with source/target snippets for rule differences
uv run audit-translations es --verbose
```

**Running from the repo root (without `cd PythonScripts`):**
```bash
uv run --project PythonScripts audit-translations es
uv run --project PythonScripts audit-translations nb --source sv
uv run --project PythonScripts audit-translations --list
```

### Testing

```uv run pytest```
