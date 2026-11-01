#!/usr/bin/env python3
"""Render the EscapeTypo product trailer as a GIF with FFmpeg."""

from __future__ import annotations

import shutil
import subprocess
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
OUTPUT = ROOT / "assets" / "demo.gif"
FFMPEG = shutil.which("ffmpeg") or "ffmpeg"
FONT = "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf"
MONO = "/usr/share/fonts/truetype/dejavu/DejaVuSansMono.ttf"


def esc(text: str) -> str:
    return text.replace("\\", "\\\\").replace(":", "\\:").replace("'", "\\'")


def text(text: str, x: str, y: str, size: int, start: float, end: float,
         font: str = FONT, color: str = "0xF8FAFC") -> str:
    return (
        f"drawtext=fontfile='{font}':text='{esc(text)}':x={x}:y={y}:"
        f"fontsize={size}:fontcolor={color}:enable='between(t,{start},{end})'"
    )


def build_filter() -> str:
    # Six short beats: problem, promise, product, privacy, learning,
    # and the final call to action. All timing is deterministic and loopable.
    parts = [
        "drawbox=x=0:y=0:w=iw:h=ih:color=0x08111f:t=fill",
        "drawbox=x=0:y=0:w=iw:h=8:color=0x22d3ee:t=fill",
        "drawbox=x=0:y=ih-8:w=iw:h=8:color=0x1d4ed8:t=fill",
        # 0–4: the noisy feed
        text("THE FEED NEVER STOPS.", "(w-text_w)/2", "330", 76, 0, 4),
        text("YOUR FOCUS DOESN'T HAVE TO.", "(w-text_w)/2", "445", 42, 0.4, 4),
        text("EscapeTypo", "84", "82", 30, 0, 4, MONO, "0x67e8f9"),
        # 4–8: the product reveal
        text("ESCAPE", "(w-text_w)/2", "290", 104, 4, 8, MONO, "0x67e8f9"),
        text("THE SLOP.", "(w-text_w)/2", "425", 104, 4.35, 8),
        text("A focused typing practice for your PDFs.", "(w-text_w)/2", "590", 34, 5, 8),
        # 8–12: show the terminal-like experience
        text("READ  /  TYPE  /  IMPROVE", "150", "190", 38, 8, 12, MONO, "0x67e8f9"),
        text(">  The quiet mind learns faster.", "150", "340", 46, 8.5, 12, MONO),
        text("WPM  72       ACCURACY  98%", "150", "500", 42, 9.2, 12, MONO, "0xa7f3d0"),
        "drawbox=x=150:y=580:w=1620:h=4:color=0x1e3a5f:t=fill",
        "drawbox=x=150:y=580:w='min(1620\\,(t-9)*700)':h=4:color=0x22d3ee:t=fill",
        # 12–16: no compromise list
        text("NO AI FEED.", "220", "230", 60, 12, 16, MONO, "0x67e8f9"),
        text("NO ADS.", "220", "335", 60, 12.35, 16, MONO, "0x67e8f9"),
        text("NO MONTHLY FEE.", "220", "440", 60, 12.7, 16, MONO, "0x67e8f9"),
        text("NO DATA LEAKING.", "220", "545", 60, 13.05, 16, MONO, "0x67e8f9"),
        # 16–20: privacy and local practice
        text("YOUR PDFs STAY YOURS.", "(w-text_w)/2", "280", 76, 16, 20, color="0xF8FAFC"),
        text("ESCAPETYPO", "(w-text_w)/2", "430", 52, 16.5, 20, MONO, "0x67e8f9"),
        text("No account. No upload. Just practice.", "(w-text_w)/2", "560", 34, 17.1, 20),
        # 20–24: close
        text("TYPE. LEARN. IMPROVE.", "(w-text_w)/2", "285", 84, 20, 24, MONO, "0x67e8f9"),
        text("READ. TYPE. REPEAT.", "(w-text_w)/2", "430", 60, 20.5, 24),
        text("Enter focus.", "(w-text_w)/2", "650", 38, 21.3, 24, MONO, "0xa7f3d0"),
        "drawbox=x='(t-20)*860':y=760:w=220:h=8:color=0x22d3ee:t=fill",
    ]
    return ",".join(parts)


def main() -> None:
    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    cmd = [
        FFMPEG, "-y", "-hide_banner", "-loglevel", "error",
        "-f", "lavfi", "-i",
        "color=c=0x08111f:s=1920x1080:r=30:d=24",
        "-filter_complex", f"[0:v]{build_filter()},fps=12,split[s0][s1];"
        "[s0]palettegen=stats_mode=diff[p];[s1][p]paletteuse=dither=sierra2_4a[v]",
        "-map", "[v]", "-t", "24", "-loop", "0", str(OUTPUT),
    ]
    subprocess.run(cmd, check=True)
    print(f"wrote {OUTPUT} ({OUTPUT.stat().st_size} bytes)")


if __name__ == "__main__":
    main()
