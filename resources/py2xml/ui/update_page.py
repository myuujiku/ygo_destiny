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

class_name = "YGOUpdatePage"
parent_class = "AdwLeafletPage"

progress_bar = GtkChildObject(
    "GtkProgressBar",
    (
        GtkProperty("show-text", GtkTrue),
        GtkProperty("fraction", XmlString("0.25")),
    ),
)


container = GtkProperty(
    "child",
    GtkObject(
        "AdwClamp",
        (
            GtkChildObject(
                "GtkBox",
                (
                    *GtkMargins(12, 12, 12, 12),
                    GtkProperty("orientation", XmlString("vertical")),
                    GtkProperty("vexpand", GtkTrue),
                    GtkProperty("hexpand", GtkTrue),
                    GtkProperty("valign", XmlString("center")),
                    progress_bar,
                    GtkChildObject(
                        "GtkLabel",
                        (
                            GtkProperty("label", XmlString("this is a progress bar")),
                            # GtkProperty("halign", XmlString("start")),
                        ),
                    ),
                ),
            ),
        ),
    ),
)


gtk_template = XmlTag(
    "template",
    options={
        "class": class_name,
        "parent": parent_class,
    },
    content=container,
)

xml_content = XmlTag("interface", gtk_template)

xml.save_to_file(
    f"{project.directory}/{__name__.split('.')[-1]}.ui", xml_content, project.header
)
