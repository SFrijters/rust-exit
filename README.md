# rust-exit

Even more minimal replacement for [bl-exit](https://github.com/BunsenLabs/bunsen-exit), using GTK4.

The Shutdown and Reboot buttons use DBus. The Logout button assumes that openbox is the window manager.

## Building

```
$ nix build
```

or

```
$ nix develop
...
$ cargo build
```

or take care of the external dependencies on your own (see `default.nix`).
