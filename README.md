
# Backup Box

A portable device to backup your daily pictures from an SD card to a portable drive. 

## Scope

Every time I go on vacation with my girlfriend, she asks me to backup the daily picture she has taken with her DLSR camera to my laptop "just to be safe".
I figured I could put together a small portable device that would be able to automatically backup new pictures in a few simple steps.

The whole thing would need to be:

- Simple:
    it needs to do one thing, and has to do it well, while adhering to the [workflow].
    No useless customization or options.
    Hopefully it should be straightforward to use even for someone with limited technical skills;

- Portable and compact: 
    it needs to be small enough so we toss it in the luggage and forget about it.
    Also it should be a single object, and not a bunch of components to be put together every time.
    No additional input device other than a touch screen;

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

A RaspberryPi 3 should be enough for the job.

A touchscreen HAT with a custom interface should be used as input device.

### Storage system

In this case a mini thumb drive (~128GB) should be enough for the build and will make the final product a bit more portable.

### Custom software GUI

The custom software interface should be written in Rust.

https://gtk-rs.org/
https://wiki.gnome.org/Apps/Glade

https://discourse.gnome.org/t/onscreen-keyboard-integration-with-gtk-3-application/1626
https://askubuntu.com/questions/903937/how-can-i-programmatically-call-ubuntus-on-screen-keyboard-in-gtk-python

Limitations on a small screen.

On screen power off options will be also available.

### Powering the RPi3

Ideally the RPi3 should be powered with a suitable powerbank [1].

### Custom enclosure

It would be nice to have a custom 3D printed enclosure to hold the RPi3, TFT touchscreen, powerbank, SD card reader and HDD.

## References

[1]: https://www.reddit.com/r/raspberry_pi/comments/fvfn4w/raspberry_pi_powered_from_a_powerbank_part_two/
