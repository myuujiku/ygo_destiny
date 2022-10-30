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

class_name = "YGOWindow"
parent_class = "AdwApplicationWindow"

test_page = GtkProperty(
    "child", GtkObject("AdwStatusPage", GtkProperty("title", XmlString("test")))
)

test_page2 = GtkProperty(
    "child", GtkObject("AdwStatusPage", GtkProperty("title", XmlString("test2")))
)

header_bar = GtkChildObject("AdwHeaderBar", ())

view_switcher = GtkChildObject(
    "AdwViewSwitcher",
    (
        GtkProperty("policy", XmlString("narrow")),
        GtkProperty("stack", XmlString("stack")),
    ),
)

view_stack = GtkChildObject(
    "AdwViewStack",
    id="stack",
    content=(
        GtkProperty("vexpand", GtkTrue),
        GtkChildObject(
            "AdwViewStackPage",
            (
                GtkProperty("name", XmlString("1")),
                GtkProperty("title", XmlString("1")),
                test_page,
            ),
        ),
        GtkChildObject(
            "AdwViewStackPage",
            (
                GtkProperty("name", XmlString("2")),
                GtkProperty("title", XmlString("2")),
                test_page2,
            ),
        ),
    ),
)

main_box = GtkProperty(
    "child",
    GtkObject(
        "GtkBox",
        (
            GtkProperty("orientation", XmlString("vertical")),
            GtkProperty("vexpand", GtkTrue),
            GtkProperty("hexpand", GtkTrue),
            header_bar,
            view_switcher,
            view_stack,
        ),
    ),
)

toast_overlay = GtkObject(
    "AdwToastOverlay",
    id="toast_overlay",
    content=(
        GtkChildObject(
            "AdwLeaflet",
            id="leaflet",
            content=(
                GtkProperty("can-navigate-back", GtkTrue),
                GtkProperty("can-unfold", GtkFalse),
                GtkProperty("transition-type", XmlString("slide")),
                GtkChildObject("AdwLeafletPage", main_box),
            ),
        )
    ),
)

gtk_template = XmlTag(
    "template",
    options={
        "class": class_name,
        "parent": parent_class,
    },
    content=(
        GtkProperty(
            "content",
            toast_overlay,
        ),
    ),
)

xml_content = XmlTag("interface", gtk_template)

xml.save_to_file(
    f"{project.directory}/{__name__.split('.')[-1]}.ui", xml_content, project.header
)
