- **A link in a text field opens on a click, and shows the hand.** A rich `TextEdit` with a
  `Mark::Link` drew its link, but the field took every press, so `on_link` never fired and the
  pointer stayed an I-beam over it. A plain click on a field's link now opens it, as one in
  read-only text does, while a press that drags, selects or double-clicks stays an edit; and
  the pointer is a hand over the link's glyphs.
- **`Tx::take_press` for a range that acts as a button in a field.** In a text field's
  `on_link`, `tx.take_press()` says the click was the link's (a task box ticked): the field
  keeps the caret and selection it had before the press, and the click doesn't count toward a
  double click.
