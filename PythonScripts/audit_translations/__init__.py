"""
MathCAT Translation Audit Tool

Compares English YAML rule files with translated versions to identify translation
gaps and issues. This tool helps translators ensure their translations are complete
and properly formatted.

Read README.md for more details.
"""
import sys
sys.stdout.reconfigure(encoding='utf-8')
from .cli import main

__all__ = [
    'main',
]
