"""Generate a skills radar SVG for the portfolio."""
import math

categories = [
    ("NGFW Config", 4.5),
    ("Log Analysis", 4.5),
    ("Threat Intel", 3.5),
    ("SIEM / SOC", 3.5),
    ("Endpoint Security", 4.0),
    ("Cloud Security", 2.5),
    ("Container Security", 3.0),
    ("Software Dev", 4.0),
]

n = len(categories)
cx, cy = 250, 250
max_r = 180
levels = 5

svg = [
    '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 500 500" width="500" height="500">',
    "<style>",
    '  .grid-line { stroke: #cbd5e0; stroke-width: 0.5; fill: none; }',
    '  .axis-line { stroke: #a0aec0; stroke-width: 0.8; }',
    '  .skill-area { fill: rgba(66, 153, 225, 0.25); stroke: #3182ce; stroke-width: 2; }',
    '  .skill-dot { fill: #2b6cb0; }',
    '  .label { font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;'
    '    font-size: 11px; fill: #2d3748; text-anchor: middle; }',
    '  .level-label { font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;'
    '    font-size: 9px; fill: #a0aec0; text-anchor: middle; }',
    '  .title { font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;'
    '    font-size: 16px; font-weight: 600; fill: #1a202c; text-anchor: middle; }',
    '  .subtitle { font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;'
    '    font-size: 11px; fill: #718096; text-anchor: middle; }',
    "</style>",
]

svg.append(f'<text x="{cx}" y="25" class="title">Skills Proficiency Radar</text>')
svg.append(
    f'<text x="{cx}" y="42" class="subtitle">'
    "CSC-7308 SysOps and Cloud Security — Winter 2025</text>"
)

# Grid polygons
for level in range(1, levels + 1):
    r = max_r * level / levels
    pts = []
    for i in range(n):
        angle = (2 * math.pi * i / n) - math.pi / 2
        pts.append(f"{cx + r * math.cos(angle):.1f},{cy + r * math.sin(angle):.1f}")
    svg.append(f'<polygon points="{" ".join(pts)}" class="grid-line" />')

# Level labels
for level in range(1, levels + 1):
    r = max_r * level / levels
    angle = -math.pi / 2
    x = cx + r * math.cos(angle) + 14
    y = cy + r * math.sin(angle) + 4
    svg.append(f'<text x="{x:.1f}" y="{y:.1f}" class="level-label">{level}</text>')

# Axis lines + labels
for i, (label, _) in enumerate(categories):
    angle = (2 * math.pi * i / n) - math.pi / 2
    x_end = cx + max_r * math.cos(angle)
    y_end = cy + max_r * math.sin(angle)
    svg.append(
        f'<line x1="{cx}" y1="{cy}" x2="{x_end:.1f}" y2="{y_end:.1f}" class="axis-line" />'
    )
    label_r = max_r + 32
    lx = cx + label_r * math.cos(angle)
    ly = cy + label_r * math.sin(angle) + 4
    svg.append(f'<text x="{lx:.1f}" y="{ly:.1f}" class="label">{label}</text>')

# Data polygon
data_points = []
for i, (_, score) in enumerate(categories):
    angle = (2 * math.pi * i / n) - math.pi / 2
    r = max_r * score / levels
    data_points.append((cx + r * math.cos(angle), cy + r * math.sin(angle)))

pts_str = " ".join(f"{x:.1f},{y:.1f}" for x, y in data_points)
svg.append(f'<polygon points="{pts_str}" class="skill-area" />')

for x, y in data_points:
    svg.append(f'<circle cx="{x:.1f}" cy="{y:.1f}" r="4" class="skill-dot" />')

# Legend
legend_items = [
    ("Hands-on (4-5)", "#38a169"),
    ("Applied (3-4)", "#d69e2e"),
    ("Conceptual (1-3)", "#4a90d9"),
]
for i, (text, color) in enumerate(legend_items):
    x_off = 110 + i * 130
    svg.append(
        f'<rect x="{x_off}" y="472" width="10" height="10" rx="2" fill="{color}" />'
    )
    svg.append(
        f'<text x="{x_off + 14}" y="481" class="label" text-anchor="start">{text}</text>'
    )

svg.append("</svg>")

with open(r"D:\pilots\406-SysOps-Cloud-Security\docs\skills-radar.svg", "w") as f:
    f.write("\n".join(svg))
print("SVG generated: docs/skills-radar.svg")
