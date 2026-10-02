#!/usr/bin/env python3
"""Runs a command with the HAL simulator GUI extension loaded.

Picks the vendored halsim build for the given `arch` (matching a directory
name under halsims/), points HAL_LoadExtensions() at its libhalsim_gui via
HALSIM_EXTENSIONS, and makes sure everything it depends on (ntcore, wpimath,
wpinet alongside it, plus libwpiHal/libwpiutil from rphal-sys) is on the
runtime library search path. Then it execs the given command in that
environment.
"""

import argparse
import os
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent
HALSIMS_DIR = REPO_ROOT / "halsims"

# rphal-sys vendors libwpiHal/libwpiutil under WPILib's platform naming,
# which doesn't match the halsims/ directory names.
RPHAL_PLATFORM = {
    "linux": "linuxx86-64",
    "osx": "osxuniversal",
}

LIBRARY_PATH_VAR = {
    "linux": "LD_LIBRARY_PATH",
    "osx": "DYLD_LIBRARY_PATH",
}


def available_archs() -> list[str]:
    return sorted(p.name for p in HALSIMS_DIR.iterdir() if p.is_dir())


def find_gui_extension(arch_dir: Path) -> Path:
    matches = sorted(arch_dir.glob("libhalsim_gui.*"))
    if not matches:
        raise FileNotFoundError(f"no libhalsim_gui.* found in {arch_dir}")
    return matches[0]


def main() -> int:
    parser = argparse.ArgumentParser(
        prog="runner",
        description="Runs a command with the HAL simulator GUI extension loaded",
    )
    parser.add_argument("arch", choices=available_archs(), help="vendored halsim build to load (a halsims/ subdirectory)")
    parser.add_argument(
        "command",
        nargs=argparse.REMAINDER,
        help="command to run, e.g. -- cargo run -p rphal-sys --example blink",
    )
    args = parser.parse_args()

    command = args.command
    if command and command[0] == "--":
        command = command[1:]
    if not command:
        parser.error("no command given to run (pass it after the arch, e.g. `runner.py linux -- cargo run ...`)")

    arch_dir = HALSIMS_DIR / args.arch
    gui_extension = find_gui_extension(arch_dir)
    rphal_lib_dir = REPO_ROOT / "rphal-sys" / "headers" / "lib" / RPHAL_PLATFORM[args.arch]

    env = os.environ.copy()
    env["HALSIM_EXTENSIONS"] = str(gui_extension)

    lib_path_var = LIBRARY_PATH_VAR[args.arch]
    search_dirs = [str(arch_dir), str(rphal_lib_dir)]
    existing = env.get(lib_path_var)
    if existing:
        search_dirs.append(existing)
    env[lib_path_var] = os.pathsep.join(search_dirs)

    os.execvpe(command[0], command, env)


if __name__ == "__main__":
    sys.exit(main())
