from pathlib import Path

directory: str = Path(__file__).absolute().parent.parent / "templates"
header: str = """<?xml version="1.0" encoding="UTF-8"?>"""

if __name__ == "__main__":
    from ui import *
