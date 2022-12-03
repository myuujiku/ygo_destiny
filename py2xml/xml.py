"""
YGO Destiny – A Yu-Gi-Oh! sealed draft simulator written in rust.
Copyright (C) 2022  myujiku

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License version 3 as
published by the Free Software Foundation.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
"""


from pathlib import Path


class XmlElement:
    __slots__ = ()

    def eval(self) -> str:
        return ""


class XmlTag(XmlElement):
    __slots__ = (
        "content",
        "name",
        "options",
    )

    content: tuple[XmlElement]
    name: str
    options: dict

    def __init__(
        self, name: str, content: tuple[XmlElement] = None, options: dict = None
    ) -> None:
        self.content = content or ()
        self.name = name
        self.options = options or {}

    def eval(self) -> str:
        if isinstance(self.content, XmlElement):
            content = self.content.eval()
        elif isinstance(self.content, str):
            content = self.content
        else:
            content: str = "".join((e.eval() for e in self.content))

        options: str = "".join(
            [f' {option[0]}="{option[1]}"' for option in self.options.items()]
        )

        if content:
            return f"<{self.name}{options}>{content}</{self.name}>"
        else:
            return f"<{self.name}{options} />"


def save_to_file(path: str, content: XmlElement, header: str = "") -> None:
    do_changes: bool = True
    new_content: str = header + content.eval()

    if Path(path).is_file():
        with open(path, "r") as file:
            old_content = file.read()

        do_changes = old_content != new_content

    if do_changes:
        with open(path, "w") as file:
            file.write(new_content)

        print(f"Compiled {path}")
    else:
        print(f"Skipped {path}")
