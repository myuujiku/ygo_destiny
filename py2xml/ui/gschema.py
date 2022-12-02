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
from xml import XmlElement, XmlTag

schema_opts = {
    "id": project.app_id,
    "path": project.app_path,
}


def Key(name: str, t: str, content: tuple[XmlElement]) -> XmlTag:
    opts = {
        "name": name,
        "type": t,
    }

    return XmlTag("key", options=opts, content=content)


content = (
    Key(
        "use-big-images",
        "b",
        (
            XmlTag("default", GtkTrue),
            XmlTag(
                "summary",
                "Whether or not to download bigger card images instead of the smaller ones",
            ),
        ),
    ),
)

schema = XmlTag("schema", options=schema_opts, content=content)

xml_content = XmlTag("schemalist", schema)


xml.save_to_file(
    f"{project.directory.parent}/{__name__.split('.')[-1]}.xml",
    xml_content,
    project.header,
)
