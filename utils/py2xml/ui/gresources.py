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

import project, xml
from gtk_xml import (
    GtkChildObject,
    GtkMargins,
    GtkObject,
    GtkProperty,
    GtkTrue,
    GtkFalse,
)
from xml import XmlString, XmlTag

template_files = (
    "templates/update_page.ui",
    "templates/window.ui",
)


def gresource(prefix: str, content) -> XmlTag:
    return XmlTag("gresource", options={"prefix": prefix}, content=content)


def template(path: str) -> XmlTag:
    return XmlTag("file", options={"compressed": "true"}, content=XmlString(path))


resources = (
    gresource("/com/myujiku/ygo_destiny", tuple(template(p) for p in template_files)),
)

xml_content = XmlTag("gresources", *resources)


xml.save_to_file(
    f"{project.directory.parent}/{__name__.split('.')[-1]}.xml",
    xml_content,
    project.header,
)
