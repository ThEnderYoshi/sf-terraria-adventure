"""update_readme v0.2.0

    By ThEnderYoshi, 2026
    Made with Python 3.12
    Under the MIT License

    Auto-updates some parts of the README file using the scan data from
    RPack Toolbox.
"""


from dataclasses import dataclass
from datetime import datetime, timezone
from typing import Any

import json
import math
import re


MILESTONE_OFFSET = 6
PACK_INFO_PATH = "pack.json"
README_PATH = "README.md"
REGION_IMG_COUNT = "img_count"
REGION_TABLE = "progress"
REGION_TXT_COUNT = "txt_count"
SCAN_DUMP_PATH = "Tools/rpack_scan_dump.json"

TRACKED_ASSET_KINDS = {
    "Images": None,
    "English Text": "Text Entries",
    "Sounds": None,
    "Music": None,
    "Fonts": None,
}


@dataclass(init=False, slots=True)
class ScanData:
    """Represents the parsed scan data."""

    kind: str
    replaced: int
    counts: str
    percent: str

    def __init__(self, kind: str, replaced: int, total: int) -> None:
        self.kind = kind
        self.replaced = replaced
        self.counts = f"{replaced:,} / {total:,}"
        self.percent = serialize_progress_bar((replaced / total) * 100.0)


def info(*args, **kwargs) -> None:
    print("[update_readme]", *args, **kwargs)


def get_dump_data() -> dict[str, Any]:
    """Reads the dumped scan data from the dump file."""

    info("Reading dump file...")

    with open(SCAN_DUMP_PATH) as f:
        return json.load(f)


def get_pack_version() -> tuple[int, int]:
    """Returns the pack's version as the tuple `(major, minor)`."""

    info(f"Reading `{PACK_INFO_PATH}`...")

    with open(PACK_INFO_PATH) as f:
        data: dict[str, Any] = json.load(f)
        version: dict[str, int] = data["Version"]
        return (version["major"], version["minor"])


def process_dump_data(raw: dict[str, Any]) -> list[ScanData]:
    """Processes the scan data into table rows."""

    result: list[ScanData] = []
    total_replaced = 0
    total_total = 0 # Hrm yes

    for kind in TRACKED_ASSET_KINDS:
        raw_data: dict[str, int] = raw[kind]
        replaced = raw_data["replaced"]
        total = raw_data["total"]
        total_replaced += replaced
        total_total += total

        name = TRACKED_ASSET_KINDS[kind]
        name = kind if name == None else name

        data = ScanData(name, replaced, total)
        result.append(data)

    total_data = ScanData("Total", total_replaced, total_total)
    result.append(total_data)
    return result


def serialize_progress_bar(percent: float) -> str:
    """Creates a progress bar."""

    percent = math.floor(percent)
    return f"![{percent}%](https://geps.dev/progress/{percent})"


def serialize_progress(
    scan_data: list[ScanData],
    version: tuple[int, int],
) -> str:
    """Creates the Markdown table to be injected."""

    info("Serializing status section...")

    # Status information

    date = datetime.now(timezone.utc).date().isoformat()

    # NOTE: My 2023 dumbass shipped 1.0 with 1006 images instead of 1000
    # so now we have to live with the consequences forever. YAY! YAY!
    milestone = version[0] * 1000 + MILESTONE_OFFSET
    replaced_imgs = scan_data[0].replaced

    milestone_pct = (
            100 if replaced_imgs >= milestone
            else 1 if replaced_imgs <= milestone - 1000
            else max(1, (replaced_imgs % 1000) // 10)
    )

    result = (
            f"\n> **Last updated:** {date} UTC \\"
            f"\n> **Update progress:** {serialize_progress_bar(milestone_pct)}"
            "\n\n"
    )

    # Start making the table

    table = (
            [["Kind", "Replaced / Total", "% Replaced"]]
            + [[r.kind, r.counts, r.percent] for r in scan_data]
    )

    # Get column sizes

    col_sizes = [3, 3, 3]

    for row in table:
        for i in range(len(col_sizes)):
            col_sizes[i] = max(len(row[i]), col_sizes[i])

    # Serialize table

    in_header = True

    for row in table:
        for i, cell in enumerate(row):
            result += "| {cell: <{size}} ".format(cell=cell, size=col_sizes[i])

        result += "|\n"

        if in_header:
            for size in col_sizes:
                result += f"| :{"-" * (size - 2)}: "

            result += "|\n"
            in_header = False

    return result


def get_readme_file() -> str:
    """Returns the contents of the README file."""

    info(f"Reading `{README_PATH}`...")

    with open(README_PATH) as f:
        return "".join(f.readlines())


def inject_region(source: str, payload: str, region_name: str) -> str:
    """Injects a string into a source string's region."""

    info(f"Injecting `{region_name}` into the file...")

    return re.sub(
        fr"<!--#region {region_name}-->[\s\S]*?<!--#endregion-->",
        f"<!--#region {region_name}-->{payload}<!--#endregion-->",
        source,
    )


def write_readme_file(content: str) -> None:
    """Writes the provided string to the README file."""

    info(f"Writing to `{README_PATH}`...")

    with open(README_PATH, "w") as f:
        f.write(content)


def main() -> None:
    """The main logic of the script."""

    raw = get_dump_data()
    data = process_dump_data(raw)
    version = get_pack_version()
    table = serialize_progress(data, version)
    img_count = f"{raw["Images"]["total"]:,}"
    txt_count = f"{raw["English Text"]["total"]:,}"

    readme = get_readme_file()
    readme = inject_region(readme, table, REGION_TABLE)
    readme = inject_region(readme, img_count, REGION_IMG_COUNT)
    readme = inject_region(readme, txt_count, REGION_TXT_COUNT)
    write_readme_file(readme)

    info("All done!")


if __name__ == "__main__":
    main()
