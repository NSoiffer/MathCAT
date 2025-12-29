"""
Data models for the audit tool.

Contains dataclasses for representing rules and comparison results.
"""

from dataclasses import dataclass, field
from typing import List, Tuple, Optional


@dataclass
class RuleInfo:
    """Information about a single rule"""
    name: Optional[str]  # None for unicode entries
    tag: Optional[str]   # None for unicode entries
    key: str             # For unicode entries, this is the character/range
    line_number: int
    raw_content: str
    has_untranslated_text: bool = False
    untranslated_keys: List[str] = field(default_factory=list)
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
    untranslated_text: List[Tuple[RuleInfo, List[str]]]  # Rules with lowercase t/ot/ct
    file_path: str
    english_rule_count: int
    translated_rule_count: int
    rule_differences: List[RuleDifference] = field(default_factory=list)  # Fine-grained diffs
