import shutil
import subprocess
import sys
from pathlib import Path
from hatchling.builders.hooks.plugin.interface import BuildHookInterface

class CustomBuildHook(BuildHookInterface):
    def initialize(self, version, build_data):
        build_data["pure_python"] = False
        build_data["infer_tag"] = True

        binary_name = "snake"
        exe = binary_name + (".exe" if sys.platform == "win32" else "")

        subprocess.run(
            ["cargo", "build", "--release", "--manifest-path", f"rust/{binary_name}/Cargo.toml"],
            check=True,
        )

        src = Path("target") / "release" / exe
        dst_dir = Path("src") / "lgtm" / "bin"
        dst_dir.mkdir(parents=True, exist_ok=True)

        dst = dst_dir / exe
        shutil.copy2(src, dst)
        dst.chmod(dst.stat().st_mode | 0o111)
