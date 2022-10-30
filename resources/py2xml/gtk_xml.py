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
