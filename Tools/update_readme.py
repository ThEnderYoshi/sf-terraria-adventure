"""update_readme v0.1.0

    By ThEnderYoshi, 2025
    Made with Python 3.10

    Auto-updates some parts of the README file.
"""


from math import floor
from typing import Any

import datetime
import json
import re
import sys


REGION_TABLE: str = "progress"
REGION_IMAGE_COUNT: str = "img_count"
REGION_TEXT_COUNT: str = "txt_count"
DUMP_FILE_PATH: str = "Tools/tpack_scan_dump.json"
README_PATH: str = "README.md"


def eprint(*args, **kwargs) -> None:
    print(*args, **kwargs, file=sys.stderr)


def get_dump_data() -> dict[str, Any]:
    """Gets the data from the dump file."""

    eprint("Reading dump file...")

    with open(DUMP_FILE_PATH) as f:
        return json.load(f)


def create_progress_bar(percent: float) -> str:
    """Creates a Markdown progress bar."""

    percent: int = floor(percent)
    return f"![{percent}%](https://geps.dev/progress/{percent})"


def create_count(asset_data: dict[str, Any]) -> str:
    """Creates a Markdown asset count."""

    found: int = asset_data["found"]
    total: int = asset_data["total"]
    return f"{found:,} / {total:,}"


def create_md_table(data: dict[str, Any]) -> str:
    """Creates the Markdown table to be injected."""

    eprint("Creating Markdown table...")

    # Header

    table: list[list[str]] = [["Progress", ""]]

    # Progress bars

    images: dict[str, Any] = data["images"]
    table.append(["Total", create_progress_bar(images["total_percent"])])
    table.append(["Update", create_progress_bar(images["milestone_percent"])])

    # Asset counts

    text_entries: dict[str, Any] = data["text_entries"]["English"]
    table.append(["Images", create_count(images)])
    table.append(["Text Entries", create_count(text_entries)])
    table.append(["Songs", create_count(data["songs"])])
    table.append(["Sounds", create_count(data["sounds"])])

    # Date

    date: str = datetime.datetime.now().date().isoformat()
    result: str = f"\n> [!INFO]\n> Last updated: _{date} UTC_\n\n"

    # Get column sizes

    col_1_size: int = 0
    col_2_size: int = 0

    for row in table:
        col_1_size = max(len(row[0]), col_1_size)
        col_2_size = max(len(row[1]), col_2_size)

    # Serialize table

    in_header: bool = True

    for row in table:
        result += "| {name: <{c1_size}} | {value: <{c2_size}} |\n".format(
            name=row[0],
            value=row[1],
            c1_size=col_1_size,
            c2_size=col_2_size,
        )

        if in_header:
            result += f"|:{'-' * col_1_size}:|:{'-' * col_2_size}:|\n"
            in_header = False

    return result


def get_readme_file() -> str:
    """Returns the contents of the README file."""

    eprint(f"Reading `{README_PATH}`...")

    with open(README_PATH, mode="r") as f:
        return "".join(f.readlines())


def inject_region(source: str, payload: str, region_name: str) -> str:
    """Injects a string into a source string's region."""

    eprint(f"Injecting `{region_name}` into README file...")
    payload = f"<!--#region {region_name}-->{payload}<!--#endregion-->"

    return re.sub(
        fr"<!--#region {region_name}-->[\s\S]*?<!--#endregion-->",
        payload,
        source,
    )


def write_readme_file(content: str) -> None:
    """Writes the provided string to the README file."""

    eprint(f"Writing to `{README_PATH}`...")

    with open(README_PATH, mode="w") as f:
        f.write(content)


def main() -> None:
    """The main logic of the script."""

    data: dict[str, Any] = get_dump_data()
    table: str = create_md_table(data)
    image_count: int = f"{data['images']['total']:,}"
    text_count: int = f"{data['text_entries']['English']['total']:,}"

    readme: str = get_readme_file()
    readme = inject_region(readme, table, REGION_TABLE)
    readme = inject_region(readme, image_count, REGION_IMAGE_COUNT)
    readme = inject_region(readme, text_count, REGION_TEXT_COUNT)
    write_readme_file(readme)

    eprint("All done!")


if __name__ == "__main__":
    main()
