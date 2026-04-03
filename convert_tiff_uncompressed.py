#!/usr/bin/env python3
"""Convert a TIFF into an uncompressed TIFF.

Usage:
  python convert_tiff_uncompressed.py input.tif output.tif

Dependency:
  pip install tifffile
"""

from __future__ import annotations

import argparse
from pathlib import Path

import tifffile as tiff


def convert_tiff_to_uncompressed(input_path: Path, output_path: Path) -> None:
    with tiff.TiffFile(input_path) as src:
        page = src.pages[0]
        data = page.asarray()

        # Keep relevant layout info where possible.
        photometric = page.photometric
        planarconfig = page.planarconfig

    tiff.imwrite(
        output_path,
        data,
        compression=None,
        photometric=photometric,
        planarconfig=planarconfig,
    )


def main() -> None:
    parser = argparse.ArgumentParser(description="Convert TIFF to uncompressed TIFF")
    parser.add_argument("input", type=Path, help="Input TIFF file")
    parser.add_argument("output", type=Path, help="Output uncompressed TIFF file")
    args = parser.parse_args()

    if not args.input.exists():
        raise SystemExit(f"Input file not found: {args.input}")

    convert_tiff_to_uncompressed(args.input, args.output)
    print(f"Written uncompressed TIFF: {args.output}")


if __name__ == "__main__":
    main()
