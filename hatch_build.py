import os
import shutil
import subprocess
import sys
from hatchling.builders.hooks.plugin.interface import BuildHookInterface

class CustomBuildHook(BuildHookInterface):
    def initialize(self, version, build_data):
        binary_name = "snake"
        result = subprocess.run(
            ["cargo", "build", "--release", "--manifest-path", f"rust/{binary_name}/Cargo.toml"],
            check=False,
        )
        if result.returncode != 0:
            sys.exit("cargo build failed")

        src = os.path.join("target", "release", binary_name)
        dst_dir = os.path.join("src", "lgtm", "bin")
        dst = os.path.join(dst_dir, binary_name)

        os.makedirs(dst_dir, exist_ok=True)
        shutil.copy2(src, dst)

        # make it executable
        os.chmod(dst, os.stat(dst).st_mode | 0o111)
