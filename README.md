# wex  : WINE EXECUTABLE

**Switched to Linux and can't open your `.exe`? wex is being built to fix that.**

Click a Windows app or game and it runs, with no setup, no guides, and no
hours lost to configuration.

> **Status: early development** (started 2 October 2026). wex is not ready to
> download yet. Follow along or help shape it.

## The idea
- **Zero setup:** each app gets its own Wine environment automatically.
- **Honest errors:** if something can't run, wex tells you why, so you don't
  waste an evening guessing.
- **Built for newcomers:** a simple click-to-play interface is the goal,
  especially for people (and kids) who have never touched a terminal.
- **Better over time:** regular updates and per-app configurations for the
  programs people actually use.

## Where it is today(Last Updated on 3rd October 2026)
- Core library in Rust: finds the data folder and works out each app's own
  prefix location.
- Next: creating prefixes, then launching apps through Wine, then a GUI.

## Contributing
Ideas and feedback are welcome. Open an issue and say hello.

## License
GPL-3.0


