# logitech_modkey_rs

This is still a work in progress.
But this is a program to configure logitech gaming keyboards like the G15 or G19,
you can easily configure the modkeys using a toml file.
 
The keyboard starts in Normal mode, you can switch between modes with the M1, M2, M3 buttons
you can reset to Normal mode by pressing MR

## Starting the server
The driver part of the app needs to run as root,
`sudo logitech_modkey_rs -m server -k G15`
after that you can run the client.
`logitech_modkey_rs -m client -k G15 -c path/to/config.toml`

## Help menu

```
Usage: logitech_mod_keys_rs [OPTIONS] -m <MODE>

Options:
  -m <MODE>          Operation mode: [server, client]
  -k <KEYBOARD>      Logitch keyboard model: [G15, G19]
  -c <CONFIG>        The path to the mapping config.toml
  -h, --help         Print help
  -V, --version      Print version

```


## Example Config for G15 Keyboards

```toml
[Normal]
G1 = "<Super>+1"
G2 = "<Super>+2"
G3 = "<Super>+3"
G4 = "<Super>+4"
G5 = "<Super>+5"
G6 = "<Super>+6"

[M1]
G1 = "<Super>+<Shift>+1"
G2 = "<Super>+<Shift>+2"
G3 = "<Super>+<Shift>+3"
G4 = "<Super>+<Shift>+4"
G5 = "<Super>+<Shift>+5"
G6 = "<Super>+<Shift>+6"

[M2]
G1 = "<Super>+<Shift>+1"
G2 = "<Super>+<Shift>+2"
G3 = "<Super>+<Shift>+3"
G4 = "<Super>+<Shift>+4"
G5 = "<Super>+<Shift>+5"
G6 = "<Super>+<Shift>+6"

[M3]
G1 = "<Super>+<Shift>+<Ctrl>+1"
G2 = "<Super>+<Shift>+<Ctrl>+2"
G3 = "<Super>+<Shift>+<Ctrl>+3"
G4 = "<Super>+<Shift>+<Ctrl>+4"
G5 = "<Super>+<Shift>+<Ctrl>+5"
G6 = "<Super>+<Shift>+<Ctrl>+6"

```



## Example Config for G19 Keyboards


```toml
[Normal]
G1 = "<Super>+1"
G2 = "<Super>+2"
G3 = "<Super>+3"
G4 = "<Super>+4"
G5 = "<Super>+5"
G6 = "<Super>+6"
G7 = "<Super>+7"
G8 = "<Super>+8"
G9 = "<Super>+9"
G10 = "<Super>+0"
G11 = "<Super>+1"
G12 = "<Super>+2"


[M1]
G1 = "<Super>+1"
G2 = "<Super>+2"
G3 = "<Super>+3"
G4 = "<Super>+4"
G5 = "<Super>+5"
G6 = "<Super>+6"
G7 = "<Super>+7"
G8 = "<Super>+8"
G9 = "<Super>+9"
G10 = "<Super>+0"
G11 = "<Super>+1"
G12 = "<Super>+2"

[M2]
G1 = "<Super>+1"
G2 = "<Super>+2"
G3 = "<Super>+3"
G4 = "<Super>+4"
G5 = "<Super>+5"
G6 = "<Super>+6"
G7 = "<Super>+7"
G8 = "<Super>+8"
G9 = "<Super>+9"
G10 = "<Super>+0"
G11 = "<Super>+1"
G12 = "<Super>+2"


[M3]
G1 = "<Super>+1"
G2 = "<Super>+2"
G3 = "<Super>+3"
G4 = "<Super>+4"
G5 = "<Super>+5"
G6 = "<Super>+6"
G7 = "<Super>+7"
G8 = "<Super>+8"
G9 = "<Super>+9"
G10 = "<Super>+0"
G11 = "<Super>+1"
G12 = "<Super>+2"


```
