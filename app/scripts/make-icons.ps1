# Generates the placeholder application icon set into src-tauri/icons.
# Run from the app/ folder: powershell -File scripts/make-icons.ps1
# For a designed icon use `npm run tauri icon path/to/icon.png` instead.

Add-Type -AssemblyName System.Drawing

$out = Join-Path $PSScriptRoot "..\src-tauri\icons"
New-Item -ItemType Directory -Force $out | Out-Null

function New-IconPng([int]$size) {
    $bmp = New-Object System.Drawing.Bitmap $size, $size, ([System.Drawing.Imaging.PixelFormat]::Format32bppArgb)
    $g = [System.Drawing.Graphics]::FromImage($bmp)
    $g.SmoothingMode = [System.Drawing.Drawing2D.SmoothingMode]::AntiAlias
    $g.TextRenderingHint = [System.Drawing.Text.TextRenderingHint]::AntiAliasGridFit
    $g.Clear([System.Drawing.Color]::Transparent)

    $r = [single]($size * 0.22)
    $m = [single]($size * 0.04)
    $w = [single]($size - 2 * $m)
    $path = New-Object System.Drawing.Drawing2D.GraphicsPath
    $path.AddArc($m, $m, 2 * $r, 2 * $r, 180, 90)
    $path.AddArc($m + $w - 2 * $r, $m, 2 * $r, 2 * $r, 270, 90)
    $path.AddArc($m + $w - 2 * $r, $m + $w - 2 * $r, 2 * $r, 2 * $r, 0, 90)
    $path.AddArc($m, $m + $w - 2 * $r, 2 * $r, 2 * $r, 90, 90)
    $path.CloseFigure()
    $brush = New-Object System.Drawing.Drawing2D.LinearGradientBrush (
        (New-Object System.Drawing.PointF 0, 0),
        (New-Object System.Drawing.PointF $size, $size),
        [System.Drawing.Color]::FromArgb(255, 79, 70, 229),
        [System.Drawing.Color]::FromArgb(255, 14, 165, 233))
    $g.FillPath($brush, $path)

    $font = New-Object System.Drawing.Font "Segoe UI", ([single]($size * 0.36)), ([System.Drawing.FontStyle]::Bold), ([System.Drawing.GraphicsUnit]::Pixel)
    $format = New-Object System.Drawing.StringFormat
    $format.Alignment = [System.Drawing.StringAlignment]::Center
    $format.LineAlignment = [System.Drawing.StringAlignment]::Center
    $rect = New-Object System.Drawing.RectangleF 0, 0, $size, $size
    $g.DrawString("MD", $font, [System.Drawing.Brushes]::White, $rect, $format)

    $ms = New-Object System.IO.MemoryStream
    $bmp.Save($ms, [System.Drawing.Imaging.ImageFormat]::Png)
    $g.Dispose(); $bmp.Dispose()
    return ,$ms.ToArray()
}

function Write-BE32([System.IO.BinaryWriter]$w, [int]$v) {
    $b = [BitConverter]::GetBytes($v); [Array]::Reverse($b); $w.Write($b)
}

$png = @{}
foreach ($s in 16, 32, 64, 128, 256, 512, 1024) { $png[$s] = New-IconPng $s }

[IO.File]::WriteAllBytes((Join-Path $out "32x32.png"), $png[32])
[IO.File]::WriteAllBytes((Join-Path $out "128x128.png"), $png[128])
[IO.File]::WriteAllBytes((Join-Path $out "128x128@2x.png"), $png[256])
[IO.File]::WriteAllBytes((Join-Path $out "icon.png"), $png[1024])

# ICO with PNG-compressed entries.
$icoSizes = 16, 32, 64, 128, 256
$ms = New-Object System.IO.MemoryStream
$w = New-Object System.IO.BinaryWriter $ms
$w.Write([uint16]0); $w.Write([uint16]1); $w.Write([uint16]$icoSizes.Count)
$offset = 6 + 16 * $icoSizes.Count
foreach ($s in $icoSizes) {
    $dim = if ($s -ge 256) { 0 } else { $s }
    $w.Write([byte]$dim); $w.Write([byte]$dim); $w.Write([byte]0); $w.Write([byte]0)
    $w.Write([uint16]1); $w.Write([uint16]32)
    $w.Write([uint32]$png[$s].Length); $w.Write([uint32]$offset)
    $offset += $png[$s].Length
}
foreach ($s in $icoSizes) { $w.Write($png[$s]) }
$w.Flush()
[IO.File]::WriteAllBytes((Join-Path $out "icon.ico"), $ms.ToArray())

# ICNS with PNG entries (ic07 128, ic08 256, ic09 512, ic10 1024).
$entries = @(@("ic07", 128), @("ic08", 256), @("ic09", 512), @("ic10", 1024))
$total = 8
foreach ($e in $entries) { $total += 8 + $png[$e[1]].Length }
$ms = New-Object System.IO.MemoryStream
$w = New-Object System.IO.BinaryWriter $ms
$w.Write([Text.Encoding]::ASCII.GetBytes("icns")); Write-BE32 $w $total
foreach ($e in $entries) {
    $w.Write([Text.Encoding]::ASCII.GetBytes($e[0])); Write-BE32 $w (8 + $png[$e[1]].Length); $w.Write($png[$e[1]])
}
$w.Flush()
[IO.File]::WriteAllBytes((Join-Path $out "icon.icns"), $ms.ToArray())

Get-ChildItem $out | Select-Object Name, Length
