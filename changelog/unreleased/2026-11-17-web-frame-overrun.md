- **Web and embedded hosts keep motion through a slow frame.** A macOS window holds the motion a
  frame starts for that frame's overrun (`FrameClock::presented`), so a commit that takes 100 ms
  doesn't eat 100 ms of the spring it started. The web and embedded (iOS) hosts never called it,
  so a slow commit charged its time to the new motion and a 280 ms slide could land in a frame
  or two. Both now call it after presenting, as a window does.
