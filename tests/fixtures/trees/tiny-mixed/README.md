# tiny-mixed — placeholder

Top-level files only (organize does not recurse into category folders).

Suggested contents (phase 03):

- `photo.jpg` → `Imagens/JPG/` (if `.jpg` enabled under imagens)
- `notes.txt` → matching category or `OUTROS/TXT/`
- `weird.xyz` → `OUTROS/XYZ/` (unknown / disabled ext)
- `.hidden` → skipped (dotfile)
- `noext` → `OUTROS/OUTROS/`

Collision case: pre-create destination `Imagens/JPG/photo.jpg` → expect
`photo_1.jpg` (extension organize rename rule).
