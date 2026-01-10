"""
Data models for the audit tool.

Contains dataclasses for representing rules and comparison results.
"""

from dataclasses import dataclass, field
from typing import Any, List, Tuple, Optional, Dict


@dataclass
class RuleInfo:
    """
    Information about a single rule parsed from a YAML file.

    Attributes
    ----------
    name : Optional[str]
        Rule name for standard rule files; None for unicode entries.
    tag : Optional[str]
        Rule tag (normalized string); None for unicode entries.
    key : str
        Stable identifier used for matching; for unicode entries this is the character or range key.
    line_number : int
        1-based line number where the rule starts in the source file.
    raw_content : str
        Raw YAML block for this rule (used for reporting/snippets).
    data : Optional[Any]
        Parsed YAML node for the rule; used for structural diffs.
    has_untranslated_text : bool
        True if the rule contains lowercase t/ot/ct/etc. values.
    untranslated_keys : List[str]
        List of untranslated text values (used for summary counts).
    untranslated_entries : List[Tuple[str, str, Optional[int]]]
        List of (key, text, line) entries extracted from lowercase translation keys.
        This drives per-issue JSONL output so each untranslated string can report
        the specific YAML line number where it appears.
    line_map : Dict[str, List[int]]
        Mapping of element type to line numbers for rule components like match,
        conditions, variables, and structural tokens. This is used to point
        structural diffs at a precise line rather than the top of the rule.
    audit_ignore : bool
        True if the raw content contains an audit-ignore marker.
    """
    name: Optional[str]  # None for unicode entries
    tag: Optional[str]   # None for unicode entries
    key: str             # For unicode entries, this is the character/range
    line_number: int
    raw_content: str
    data: Optional[Any] = None
    has_untranslated_text: bool = False
    untranslated_keys: List[str] = field(default_factory=list)
    untranslated_entries: List[Tuple[str, str, Optional[int]]] = field(default_factory=list)  # (key, text, line) for JSONL output
    line_map: Dict[str, List[int]] = field(default_factory=dict)  # Element-type -> line numbers for precise diff locations
    audit_ignore: bool = False


@dataclass
class RuleDifference:
    """Fine-grained difference between English and translated rule"""
    english_rule: RuleInfo
    translated_rule: RuleInfo
    diff_type: str  # 'match', 'condition', 'structure', 'variables'
    description: str
    english_snippet: str
    translated_snippet: str


@dataclass
class ComparisonResult:
    """Results from comparing English and translated files"""
    missing_rules: List[RuleInfo]           # Rules in English but not in translation
    extra_rules: List[RuleInfo]             # Rules in translation but not in English
    untranslated_text: List[Tuple[RuleInfo, List[Tuple[str, str, Optional[int]]]]]  # Rules with lowercase t/ot/ct
    file_path: str
    english_rule_count: int
    translated_rule_count: int
    rule_differences: List[RuleDifference] = field(default_factory=list)  # Fine-grained diffs
