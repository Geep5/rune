- **A link in a text field opens on a click, and shows the hand.** A rich `TextEdit` with a
  `Mark::Link` drew its link, but the field took every press, so `on_link` never fired and the
  pointer stayed an I-beam over it. A plain click on a field's link now opens it, as one in
  read-only text does, while a press that drags, selects or double-clicks stays an edit; and
  the pointer is a hand over the link's glyphs.
