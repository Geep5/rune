- **Web: Backspace at a field's very start reaches the app.** With the caret at offset 0 and
  nothing selected, Backspace deletes nothing, so the browser fires no `input` event and the
  app never heard the key. The bridge now sends it as a `Backspace` key, as native platforms
  do, so an editor can merge the line into the one above or drop its style.
