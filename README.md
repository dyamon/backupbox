
# Backup Box

A portable device to backup your daily pictures from an SD card to a portable drive. 

## Motivation & Scope

> Every time I go on vacation with my girlfriend, she asks me to backup the daily picture she has taken with her DLSR camera to my laptop "just to be safe".
> I figured I could put together a small portable device that would be able to automatically backup new pictures in a few simple steps, and spare my laptop the daily torture.

The whole thing needs to be:

- Simple:
    it needs to do one thing, and has to do it well, while adhering to the [workflow].
    No useless customization or options.
    Hopefully it should be straightforward to use even for someone with limited technical skills;

- Portable and compact: 
    it needs to be small enough so we toss it in the luggage and forget about it.
    Also it should be a single object, and not a bunch of components to be put together every time.
    No additional input device other than a touch screen should be required;

- Incremental:
    Backup should be incremental and duplicate backups should be avoided;

## Details

### Workflow

The entire project is built around the following workflow.
Since the final product needs to be **simple**, the entire backup process should be straightforward.
Options, customization and branches in the workflow should be avoided as much as possible.

1. Power on the RPi
1. Connect external microSD card
1. Start backup from the welcome screen
1. Select device to backup
1. Select the destination of the backup
1. Start the backup
1. Power off the RPi when the process is complete
1. Remove external microSD card

The requirements might change with time and use, and as a consequence the workflow might change as well.

### Main board and touchscreen

A [Raspberry Pi 3 Model B] is used for the build.
This turned out to be more than enough.

A 3.5" 320x480 [touchscreen] is used as input, configured according to the [wiki guide].

### Storage system

In this case a [mini thumb drive] (~128GB) should be enough.

### Custom software GUI

The custom software interface is written in Rust using the [`gtk-rs` library][gtk-rs].
The design was partially created using [Glade].

To install GTK 3:
```{#install_gtk3 .sh}
sudo apt-get install libgtk-3-dev
```

To install Rust (note the [`RUSTUP_UNPACK_RAM` configuration][rustup ram issue]):
```{#install_rust .sh}
export RUSTUP_UNPACK_RAM=16777216
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

To compile and run the project:
```{#install_project .sh}
mkdir -p ~/git && cd ~/git
git clone https://git.dyamon.me/backupbox/
cd backupbox
cargo build
```

### Powering the RPi3

Ideally the RPi3 can be powered with a [suitable powerbank].
The prototype is being tested with a [2000000mAh powerbank] capable of outputting 5V/3A.

An alternative route is to use a Li-Po battery attached directly to the RPi.
While this solution might be more compact, a generic powerbank can be used for other things as well.

### Custom enclosure

I'm working on a simple prototype for a custom 3D printed enclosure to hold the RPi3, touchscreen, powerbank and SD card reader.
The design was developed with [OpenSCAD] and is as parametrized as possible to allow variations is the size of the different components.

Here is an early prototype for the case:
<p align="center">
  <img src="https://git.dyamon.me/backupbox/plain/resources/enclosure.png" alt="A screenshot of an early version of the custom 3D enclosure."/>
</p>


[Raspberry Pi 3 Model B]: https://www.raspberrypi.org/products/raspberry-pi-3-model-b/
[touchscreen]: https://thepihut.com/products/spi-3-5-320x480-touch-screen-gpio
[wiki guide]: https://www.waveshare.com/wiki/3.5inch_RPi_LCD_(A)
[mini thumb drive]: https://www.ebay.co.uk/itm/353470846385
[gtk-rs]: https://gtk-rs.org/
[Glade]: https://wiki.gnome.org/Apps/Glade
[rustup ram issue]: https://github.com/rust-lang/rustup/issues/2128
[suitable powerbank]: https://www.reddit.com/r/raspberry_pi/comments/fvfn4w/raspberry_pi_powered_from_a_powerbank_part_two/
[2000000mAh powerbank]: https://www.ebay.co.uk/itm/223949142966
[OpenSCAD]: 

