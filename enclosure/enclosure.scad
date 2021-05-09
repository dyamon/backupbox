
$fn = 50;

// Tollerance
tollerance = 1;
epsilon = 0.01;

// Wall
wall_t = 4;     // wall thickness
wall_r = 5;     // wall magnet housing radius

// Powerbank measures
pbank_w = 140;
pbank_d = 72;
pbank_h = 15;
pbank_r = 7.5;  // pbank corner radius (just an approximation)

// RPi measures
rpi_w = 85;
rpi_d = 56;
rpi_h = 25;

// USB/SDcard reader measures
usb_w = 124;
usb_d = 23;
usb_h = 20;
usb_r = 5;

// Magnets
magnet_r = 3;
magnet_h = 4;

*cube([278, 156 ,300]);

module round_cube_poles(w, d, h, r) {
    linear_extrude(height = h) {
        translate([r, r, 0]) circle(r);
        translate([r, d - r, 0]) circle(r);
        translate([w - r, r, 0]) circle(r);
        translate([w - r, d - r, 0]) circle(r);
    }
}
module round_cube(w, d, h, r) {
    hull() round_cube_poles(w, d, h, r);
}

module hexagon(radius, round) {
    points = 6;
    hull() for (i = [0:points-1]) {
            angle = i * 360 / points;
            translate([(radius - round) * cos(angle), (radius - round) * sin(angle), 0])
                circle(round);
    }
}

module honeycomb(size, hexagon_r, round = 2, spacing = 5, offset = [0, 0]) {
    I = size.x / (2 * hexagon_r + spacing / cos(30)) + 1;
    J = size.y / (2 * hexagon_r * cos(30) + spacing) + 1;
    intersection() {
        square(size);
        translate([-offset.x, -offset.y, 0])
            for (i = [0:I], j = [0:J]) {
                translate([
                    i * (3 * hexagon_r + 2 * spacing / cos(30)),
                    j * (2 * hexagon_r * cos(30) + spacing),
                    0
                ]) hexagon(hexagon_r, round);
                translate([
                    i * (3 * hexagon_r + 2 * spacing / cos(30)) + 1.5 * hexagon_r + spacing / cos(30),
                    j * (2 * hexagon_r * cos(30) + spacing) + hexagon_r * cos(30) + spacing / 2,
                    0
                ]) hexagon(hexagon_r, round);
            }
    }
}

module housing(height, bot_magnets = true, top_magnets = true) {
    difference() {
        union() {
            // Main housing
            difference() {
                round_cube(
                        pbank_w + 2 * wall_t + tollerance,
                        pbank_d + 4 * wall_r + tollerance,
                        height,
                        wall_r
                        );
                translate([wall_t, wall_t, wall_t])
                    round_cube(
                            pbank_w + tollerance,
                            pbank_d + 4 * wall_r - 2 * wall_t + tollerance,
                            height,
                            wall_r
                            );
            }
            // Magnets housing
            round_cube_poles(
                    pbank_w + 2 * wall_t + tollerance,
                    pbank_d + 4 * wall_r + tollerance,
                    height,
                    wall_r
                    );
        }

        // Bottom magnet holes
        if (bot_magnets) {
            translate([wall_r - magnet_r, wall_r - magnet_r, -tollerance / 2])
                round_cube_poles(
                    pbank_w + 2 * wall_t + tollerance - 2 * (wall_r - magnet_r),
                    pbank_d + 2 * wall_r + 2 * magnet_r + tollerance,
                    magnet_h + tollerance,
                    magnet_r
                );
        }
        // Top magnet holes
        if (top_magnets) {
            translate([wall_r - magnet_r, wall_r - magnet_r, height - tollerance / 2 - magnet_h])
                round_cube_poles(
                    pbank_w + 2 * wall_t + tollerance - 2 * (wall_r - magnet_r),
                    pbank_d + 2 * wall_r + 2 * magnet_r + tollerance,
                    magnet_h + tollerance,
                    magnet_r
                );
        }
    }
}

module notched(height) {
    difference() {
        translate([epsilon, epsilon, 0])
            round_cube(
                pbank_w + 2 * wall_t + tollerance - 2 * epsilon,
                pbank_d + 4 * wall_r + tollerance - 2 * epsilon,
                height,
                wall_r - epsilon
            );
        translate([0, 0, -height])
            scale([1, 1, 2])
            union() {
                difference() {
                    round_cube(
                        pbank_w + 2 * wall_t + tollerance,
                        pbank_d + 4 * wall_r + tollerance,
                        height,
                        wall_r
                    );
                    translate([wall_t, wall_t, -epsilon])
                        scale([1, 1, 2])
                        round_cube(
                            pbank_w + tollerance,
                            pbank_d + 4 * wall_r - 2 * wall_t + tollerance,
                            height,
                            wall_r
                        );
                }
                round_cube_poles(
                    pbank_w + 2 * wall_t + tollerance,
                    pbank_d + 4 * wall_r + tollerance,
                    height,
                    wall_r
                );
            }
    }

    translate([0, 0, height]) children();
}

// Powerbank housing
translate([pbank_d + 4 * wall_r + tollerance, 0, 0])
rotate([0, 0, 90])
union() {
    // Housing
    difference() {
        housing(pbank_h + 1.5 * wall_t + tollerance, bot_magnets = false);
        // Cut-outs
        cutouts_notch = 2;
        cutouts_r = 2;
        translate([1.5 * wall_t, 2 * wall_r + tollerance / 2 + cutouts_notch, wall_t + tollerance / 2 + cutouts_notch])
            rotate([0, -90, 0])
            round_cube(pbank_h - 2 * cutouts_notch, pbank_d - 2 * cutouts_notch, 2 * wall_t, cutouts_r);
        translate([2 * wall_r + tollerance / 2, 1.5 * wall_t,  wall_t + tollerance / 2])
            rotate([90, 0, 0])
            round_cube(pbank_w / 5, pbank_h, 2 * wall_t, cutouts_r);
        
    // Bottom honeycomb patterns
    // TODO: put this code in a module
    translate([0, 0, - wall_t / 2])
        linear_extrude(height = 2 * wall_t)
        intersection() {
            honeycomb(size = [pbank_w + 2 * wall_t + tollerance, pbank_d + 4 * wall_r + tollerance], hexagon_r = 8, round = 2, spacing = 3, offset = [3, 3]);
            translate([wall_t + tollerance / 2, wall_t + tollerance / 2, 0])
                offset(r = -2)
                difference() {
                    width = pbank_w;
                    depth = pbank_d + 4 * wall_r - 2 * wall_t;
                    color("green")
                        square([width, depth]);
                    circle(wall_r);
                    translate([0, depth, 0]) circle(wall_r);
                    translate([width, 0, 0]) circle(wall_r);
                    translate([width, depth, 0]) circle(wall_r);
                }
        }
    }
    // Powerbank ghost
    %translate([
        wall_t + tollerance / 2,
        2 * wall_r + tollerance / 2,
        wall_t + tollerance / 2
    ]) round_cube(pbank_w, pbank_d, pbank_h, pbank_r);
}

// RPi housing
//translate([0, 0, 3 * pbank_h])
translate([2 * (pbank_d + 4 * wall_r + tollerance) + 10, 0, 0])
rotate([0, 0, 90])
union() {
    // Housing
    notched(wall_t / 2) difference() {
        union() {
            housing(rpi_h + wall_t + tollerance);
            // Divider (Rpi/USB adapter)
            opening = 20;
            translate([wall_t + opening, usb_h + wall_t + tollerance, 0])
                cube([pbank_w + tollerance + wall_t / 2 - opening, wall_t / 2, rpi_h + tollerance / 2]);
        }
        // USB openings
        usb_offset = 2;
        usb_cutout = [99, 17];
        translate([
            pbank_w + 2 * wall_t + tollerance / 2 - usb_cutout.x - 2 * wall_r - usb_offset,
            1.5 * wall_t,
            wall_t + tollerance / 2 + usb_offset
        ]) rotate([90, 0, 0]) round_cube(usb_cutout.x, usb_cutout.y, 2 * wall_t, 2);
        // RPi power/hdmi/jack opening
        rpi_cutout = [60, rpi_h / 3];
        translate([
            pbank_w + 2 * wall_t + tollerance / 2 - rpi_cutout.x - 2 * wall_r,
            pbank_d + 4 * wall_r + tollerance + wall_t / 2,
            wall_t + tollerance / 2 + 2
        ]) rotate([90, 0, 0]) round_cube(rpi_cutout.x, rpi_cutout.y, 2 * wall_t, 2);
    }
    // RPi mounts and ghost
    adj = 2; // Ports in a RPi3 go a bit over the edge (check lid as well)
    translate([
        pbank_w + 2 * wall_t + tollerance / 2 - 2 * wall_r - rpi_w, 
        pbank_d + 4 * wall_r + tollerance / 2 - wall_t - rpi_d - adj,
        1.5 * wall_t
    ]) union() {
        mounts = [[23.5, 3.5], [23.5, 3.5 + 49], [23.5 + 58, 3.5], [23.5 + 58, 3.5 + 49]];
        mbase_d = 6;
        mbase_h = 2;
        mount_d = 2.75 - 0.05;
        mount_h = 10;
        for (pos = mounts) {
            translate([pos.x, pos.y, 0]) cylinder(d = mbase_d, h = mbase_h);
            translate([pos.x, pos.y, mbase_h]) cylinder(d = mount_d, h = mount_h);
        }
        %cube([rpi_w, rpi_d, rpi_h]);
    }
    // USB adapter ghost
    %translate([
        pbank_w + 2 * wall_t + tollerance / 2 - usb_w - 2 * wall_r,
        usb_h + wall_t + tollerance / 2,
        wall_t + tollerance / 2,
    ]) rotate([90, 0, 0]) round_cube(usb_w, usb_d, usb_h, usb_r);
}

// RPi lid
//translate([0, 0, 7 * pbank_h])
translate([3 * (pbank_d + 4 * wall_r + tollerance) + 20, 0, 0])
rotate([0, 0, 90])
union() {
    difference() {
        notched(wall_t / 2)
        difference() {
            round_cube(
                pbank_w + 2 * wall_t + tollerance - 2 * epsilon,
                pbank_d + 4 * wall_r + tollerance - 2 * epsilon,
                wall_t / 2 + magnet_h,
                wall_r - epsilon
            );
            // Magnet holes
            translate([wall_r - magnet_r, wall_r - magnet_r, -tollerance / 2])
                round_cube_poles(
                    pbank_w + 2 * wall_t + tollerance - 2 * (wall_r - magnet_r),
                    pbank_d + 2 * wall_r + 2 * magnet_r + tollerance,
                    magnet_h + tollerance,
                    magnet_r
                );
            // Upper LCD cutout
            adj = 2; // Ports in a RPi3 go a bit over the edge
            translate([
                pbank_w + 2 * wall_t + tollerance / 2 - 2 * wall_r - rpi_w + 4, 
                pbank_d + 4 * wall_r + tollerance / 2 - wall_t - rpi_d + 3 - adj,
                -rpi_h / 2,
            ]) cube([rpi_w - 7, rpi_d - 6, rpi_h]);
        }
        // Lower LCD cutout
        translate([
            pbank_w + 2 * wall_t + tollerance / 2 - 2 * wall_r - rpi_w - 1.5, 
            pbank_d + 4 * wall_r + tollerance / 2 - wall_t - rpi_d - 4,
            0,
        ]) cube([rpi_w + 3, rpi_d + 6, wall_t / 2]);
        // Top honeycomb patterns
        translate([0, 0, - wall_t / 2])
            linear_extrude(height = 4 * wall_t)
            intersection() {
                honeycomb(size = [pbank_w + 2 * wall_t + tollerance, pbank_d + 4 * wall_r + tollerance], hexagon_r = 8, round = 2, spacing = 3, offset = [3, 3]);
                translate([wall_t + tollerance / 2, wall_t + tollerance / 2, 0])
                    offset(r = -2)
                    difference() {
                        width = pbank_w;
                        depth = pbank_d + 4 * wall_r - 2 * wall_t;
                        square([width, depth]);
                        circle(wall_r);
                        translate([0, depth, 0]) circle(wall_r);
                        translate([width, 0, 0]) circle(wall_r);
                        translate([width, depth, 0]) circle(wall_r);
                        translate([
                            pbank_w + tollerance / 2 - 2 * wall_r - rpi_w + 3 + 2.5, 
                            pbank_d + 4 * wall_r + tollerance / 2 - wall_t - rpi_d - 5,
                            0
                        ]) square([rpi_w - 3, rpi_d - 2]);
                    }
            }
    }
}

