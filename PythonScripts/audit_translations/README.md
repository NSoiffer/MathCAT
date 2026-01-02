# MathCAT Translation Audit Tool

This tool compares English YAML rule files with translated versions to identify translation gaps and formatting issues. It assists translators in ensuring their translations are complete, consistent, and properly formatted.

### 🔍 Detection Capabilities

The tool analyzes rule files to detect the following issues:

* **Missing Rules:** Rules present in the master English file but missing in the target translation.
* **Extra Rules:** Rules present in the translation but absent in English (flagged as potentially intentional language-specific additions).
* **Untranslated Text:** Detects text keys that still use **lowercase** formatting, indicating they haven't been verified or translated yet.

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

---

### ⚙️ Usage & Commands

**Syntax:**
```bash
python -m audit_translations <language> [--file <specific_file>]
python -m audit_translations --list
```

**Convenience Features:**
* `--list`: Displays all available languages.
* `--file`: Audits a single specific file instead of the whole directory.
* `--format`: Output format (`rich`, `jsonl`).
* `--output`: Write output to a file instead of stdout.
* `--rules-dir`: Override the Rules/Languages directory path.
* `--only`: Filter issue types (comma-separated): `missing`, `untranslated`, `extra`, `diffs`, `all`.
* **Summary Stats:** Provides a statistical summary after every run.

**Examples:**

```bash
# List available languages
python -m audit_translations --list

# Audit all Spanish translation files
python -m audit_translations es

# Audit German translations
python -m audit_translations de

# Audit only a specific file
python -m audit_translations es --file SharedRules/default.yaml

# Produce JSONL output for automation or AI workflows
python -m audit_translations es --format jsonl --output es-issues.jsonl

```
