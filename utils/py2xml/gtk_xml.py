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

from xml import XmlElement, XmlString, XmlTag


class GtkObject(XmlTag):
    def __init__(
        self, class_name: str, content: tuple[XmlElement] = None, id: str = None
    ) -> None:
        options: dict = {"class": class_name}
        if id:
            options["id"] = id

        super().__init__("object", content, options)


class GtkChildObject(XmlTag):
    def __init__(
        self,
        class_name: str,
        content: tuple[XmlElement] = None,
        options: dict = None,
        id: str = None,
    ) -> None:
        super().__init__("child", GtkObject(class_name, content, id), options)


class GtkProperty(XmlTag):
    def __init__(self, prop_name: str, content: tuple[XmlElement] = None) -> None:
        super().__init__("property", content, {"name": prop_name})


def GtkMargins(s: int, e: int, t: int, b: int) -> list[GtkProperty]:
    args: tuple = (s, e, t, b)
    directions: tuple = ("start", "end", "top", "bottom")
    return [
        GtkProperty(f"margin-{directions[i]}", XmlString(str(args[i])))
        for i in range(4)
    ]


GtkTrue = XmlString("true")
GtkFalse = XmlString("false")
