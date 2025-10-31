use iced::Color;

use crate::theme::tokens::{ColorScale, Tokens};
/// Converts HSL color values to RGB Color
/// h: hue in degrees (0-360)
/// s: saturation as percentage (0-100)
/// l: lightness as percentage (0-100)
const fn hsl(h: f32, s: f32, l: f32) -> Color {
    let h = h / 360.0;
    let s = s / 100.0;
    let l = l / 100.0;

    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h * 6.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;

    let (r, g, b) = if h < 1.0 / 6.0 {
        (c, x, 0.0)
    } else if h < 2.0 / 6.0 {
        (x, c, 0.0)
    } else if h < 3.0 / 6.0 {
        (0.0, c, x)
    } else if h < 4.0 / 6.0 {
        (0.0, x, c)
    } else if h < 5.0 / 6.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    Color::from_rgb(r + m, g + m, b + m)
}

pub mod dark {
    use iced::Color;

    use crate::theme::{pallete::hsl, tokens::ColorScale};

    // Gray colors
    pub const GRAY: ColorScale = ColorScale {
        c50: hsl(240.0, 5.1, 15.0),
        c100: hsl(240.0, 5.7, 18.2),
        c200: hsl(240.0, 4.6, 22.0),
        c300: hsl(240.0, 5.0, 27.6),
        c400: hsl(240.0, 5.0, 35.5),
        c500: hsl(240.0, 3.7, 44.0),
        c600: hsl(240.0, 5.3, 58.0),
        c700: hsl(240.0, 5.6, 73.0),
        c800: hsl(240.0, 7.3, 84.0),
        c900: hsl(240.0, 9.1, 91.8),
        c950: hsl(0.0, 0.0, 95.0),
    };

    // Red colors
    pub const RED: ColorScale = ColorScale {
        c50: hsl(0.0, 56.0, 23.9),
        c100: hsl(0.6, 60.0, 33.9),
        c200: hsl(0.9, 67.2, 37.1),
        c300: hsl(1.1, 71.3, 43.7),
        c400: hsl(1.0, 76.0, 52.5),
        c500: hsl(0.7, 89.6, 57.2),
        c600: hsl(0.0, 98.6, 67.9),
        c700: hsl(0.0, 100.0, 72.3),
        c800: hsl(0.0, 100.0, 85.6),
        c900: hsl(0.0, 100.0, 90.3),
        c950: hsl(0.0, 100.0, 95.9),
    };

    // Orange colors
    pub const ORANGE: ColorScale = ColorScale {
        c50: hsl(15.0, 64.2, 23.3),
        c100: hsl(15.1, 70.9, 31.1),
        c200: hsl(15.3, 75.7, 35.5),
        c300: hsl(17.1, 83.5, 42.7),
        c400: hsl(20.1, 88.0, 50.8),
        c500: hsl(24.3, 100.0, 50.5),
        c600: hsl(27.2, 100.0, 57.7),
        c700: hsl(31.3, 100.0, 68.7),
        c800: hsl(33.8, 100.0, 79.3),
        c900: hsl(38.9, 100.0, 87.7),
        c950: hsl(46.2, 100.0, 95.0),
    };

    // Amber colors
    pub const AMBER: ColorScale = ColorScale {
        c50: hsl(21.9, 66.3, 21.1),
        c100: hsl(21.5, 73.6, 29.7),
        c200: hsl(22.3, 77.6, 33.3),
        c300: hsl(25.4, 84.2, 39.6),
        c400: hsl(31.4, 87.4, 46.7),
        c500: hsl(37.0, 96.6, 48.3),
        c600: hsl(43.3, 100.0, 53.4),
        c700: hsl(46.5, 100.0, 61.1),
        c800: hsl(49.3, 100.0, 73.0),
        c900: hsl(51.8, 100.0, 85.0),
        c950: hsl(60.0, 100.0, 94.6),
    };

    // Yellow colors
    pub const YELLOW: ColorScale = ColorScale {
        c50: hsl(32.5, 60.0, 18.2),
        c100: hsl(28.1, 68.6, 29.0),
        c200: hsl(31.3, 75.8, 30.8),
        c300: hsl(34.7, 84.4, 35.3),
        c400: hsl(40.1, 87.3, 43.3),
        c500: hsl(44.7, 88.0, 46.0),
        c600: hsl(47.7, 100.0, 50.9),
        c700: hsl(51.3, 100.0, 59.9),
        c800: hsl(54.6, 100.0, 73.0),
        c900: hsl(58.9, 100.0, 84.2),
        c950: hsl(60.0, 100.0, 94.0),
    };

    // Lime colors
    pub const LIME: ColorScale = ColorScale {
        c50: hsl(86.5, 54.4, 18.0),
        c100: hsl(87.6, 56.8, 23.3),
        c200: hsl(85.8, 63.2, 24.5),
        c300: hsl(86.1, 72.0, 29.4),
        c400: hsl(85.5, 76.8, 37.3),
        c500: hsl(84.3, 74.2, 42.1),
        c600: hsl(82.8, 81.5, 52.6),
        c700: hsl(82.0, 89.9, 64.0),
        c800: hsl(80.9, 97.9, 76.6),
        c900: hsl(77.9, 100.0, 85.8),
        c950: hsl(69.5, 100.0, 93.8),
    };

    // Green colors
    pub const GREEN: ColorScale = ColorScale {
        c50: hsl(144.3, 53.6, 16.0),
        c100: hsl(143.2, 55.4, 23.5),
        c200: hsl(141.5, 58.2, 26.3),
        c300: hsl(140.8, 64.2, 31.8),
        c400: hsl(140.3, 68.0, 39.2),
        c500: hsl(141.1, 64.9, 43.0),
        c600: hsl(141.6, 72.4, 55.2),
        c700: hsl(141.7, 82.7, 70.1),
        c800: hsl(141.0, 90.9, 82.1),
        c900: hsl(142.0, 100.0, 89.1),
        c950: hsl(144.0, 100.0, 95.5),
    };

    // Emerald colors
    pub const EMERALD: ColorScale = ColorScale {
        c50: hsl(164.3, 75.0, 13.5),
        c100: hsl(163.5, 72.6, 20.1),
        c200: hsl(162.1, 73.7, 22.4),
        c300: hsl(161.3, 77.3, 27.6),
        c400: hsl(159.6, 77.1, 34.3),
        c500: hsl(159.1, 73.5, 37.9),
        c600: hsl(157.8, 66.8, 48.9),
        c700: hsl(156.2, 76.1, 63.8),
        c800: hsl(152.4, 84.4, 77.4),
        c900: hsl(149.3, 100.0, 87.0),
        c950: hsl(158.6, 100.0, 94.8),
    };

    // Teal colors
    pub const TEAL: ColorScale = ColorScale {
        c50: hsl(176.5, 51.5, 15.4),
        c100: hsl(175.9, 54.7, 22.3),
        c200: hsl(175.9, 60.7, 23.9),
        c300: hsl(174.5, 67.3, 28.8),
        c400: hsl(174.4, 71.9, 34.9),
        c500: hsl(173.1, 71.0, 38.3),
        c600: hsl(172.3, 68.2, 48.1),
        c700: hsl(170.5, 81.3, 61.5),
        c800: hsl(168.4, 92.1, 75.2),
        c900: hsl(168.3, 100.0, 86.0),
        c950: hsl(180.0, 100.0, 95.5),
    };

    // Cyan colors
    pub const CYAN: ColorScale = ColorScale {
        c50: hsl(197.1, 53.8, 20.3),
        c100: hsl(196.8, 57.3, 27.2),
        c200: hsl(195.3, 62.7, 29.4),
        c300: hsl(193.5, 71.3, 34.1),
        c400: hsl(192.5, 76.8, 40.6),
        c500: hsl(189.4, 78.6, 42.6),
        c600: hsl(188.2, 89.1, 51.7),
        c700: hsl(187.0, 98.6, 66.2),
        c800: hsl(184.9, 100.0, 78.3),
        c900: hsl(180.0, 100.0, 86.6),
        c950: hsl(180.0, 100.0, 94.8),
    };

    // Sky colors
    pub const SKY: ColorScale = ColorScale {
        c50: hsl(203.0, 63.8, 20.9),
        c100: hsl(203.4, 70.4, 28.0),
        c200: hsl(202.7, 75.8, 30.8),
        c300: hsl(203.1, 80.4, 36.1),
        c400: hsl(202.1, 80.5, 44.3),
        c500: hsl(199.7, 85.9, 47.7),
        c600: hsl(198.7, 97.9, 57.2),
        c700: hsl(198.7, 100.0, 70.5),
        c800: hsl(198.8, 100.0, 82.5),
        c900: hsl(198.5, 100.0, 89.9),
        c950: hsl(186.0, 100.0, 95.5),
    };

    // Blue colors
    pub const BLUE: ColorScale = ColorScale {
        c50: hsl(227.1, 49.5, 22.7),
        c100: hsl(225.8, 58.9, 36.8),
        c200: hsl(227.7, 64.4, 42.9),
        c300: hsl(226.1, 72.7, 51.2),
        c400: hsl(222.6, 86.5, 56.3),
        c500: hsl(217.8, 95.8, 57.4),
        c600: hsl(213.3, 100.0, 65.0),
        c700: hsl(210.9, 100.0, 74.8),
        c800: hsl(211.5, 100.0, 83.4),
        c900: hsl(211.0, 100.0, 88.9),
        c950: hsl(201.8, 100.0, 95.3),
    };

    // Indigo colors
    pub const INDIGO: ColorScale = ColorScale {
        c50: hsl(243.5, 40.8, 27.0),
        c100: hsl(242.9, 45.7, 37.6),
        c200: hsl(244.7, 52.7, 43.1),
        c300: hsl(245.3, 60.5, 52.4),
        c400: hsl(244.1, 79.2, 60.4),
        c500: hsl(239.6, 88.7, 63.8),
        c600: hsl(234.5, 96.7, 70.9),
        c700: hsl(229.4, 100.0, 78.3),
        c800: hsl(227.1, 100.0, 85.0),
        c900: hsl(223.8, 100.0, 89.9),
        c950: hsl(220.0, 100.0, 95.1),
    };

    // Violet colors
    pub const VIOLET: ColorScale = ColorScale {
        c50: hsl(265.1, 57.3, 25.4),
        c100: hsl(263.5, 63.8, 39.4),
        c200: hsl(263.4, 66.2, 44.1),
        c300: hsl(263.7, 72.8, 52.4),
        c400: hsl(262.5, 87.3, 59.8),
        c500: hsl(258.3, 95.1, 63.2),
        c600: hsl(255.1, 100.0, 67.2),
        c700: hsl(253.0, 100.0, 81.5),
        c800: hsl(251.7, 100.0, 87.9),
        c900: hsl(254.1, 100.0, 91.7),
        c950: hsl(257.1, 100.0, 96.1),
    };

    // Purple colors
    pub const PURPLE: ColorScale = ColorScale {
        c50: hsl(276.0, 54.3, 20.5),
        c100: hsl(273.6, 61.8, 35.4),
        c200: hsl(272.9, 64.0, 41.4),
        c300: hsl(271.9, 68.1, 49.2),
        c400: hsl(271.5, 85.1, 57.8),
        c500: hsl(270.7, 96.4, 62.1),
        c600: hsl(270.5, 100.0, 71.9),
        c700: hsl(270.9, 100.0, 81.3),
        c800: hsl(272.4, 100.0, 87.7),
        c900: hsl(276.7, 100.0, 91.5),
        c950: hsl(300.0, 100.0, 96.5),
    };

    // Fuchsia colors
    pub const FUCHSIA: ColorScale = ColorScale {
        c50: hsl(297.1, 51.2, 18.6),
        c100: hsl(296.7, 59.5, 31.5),
        c200: hsl(295.4, 65.4, 35.1),
        c300: hsl(294.6, 67.4, 42.2),
        c400: hsl(293.3, 68.7, 51.2),
        c500: hsl(292.1, 88.4, 57.7),
        c600: hsl(292.0, 98.5, 59.5),
        c700: hsl(292.4, 100.0, 79.5),
        c800: hsl(292.9, 100.0, 86.8),
        c900: hsl(300.0, 100.0, 91.5),
        c950: hsl(300.0, 100.0, 96.3),
    };

    // Pink colors
    pub const PINK: ColorScale = ColorScale {
        c50: hsl(336.2, 59.6, 20.0),
        c100: hsl(336.8, 63.9, 34.0),
        c200: hsl(336.8, 68.7, 37.6),
        c300: hsl(336.1, 71.8, 44.5),
        c400: hsl(333.9, 74.9, 53.1),
        c500: hsl(330.7, 86.3, 57.7),
        c600: hsl(328.6, 91.5, 67.2),
        c700: hsl(327.4, 97.6, 78.7),
        c800: hsl(325.1, 100.0, 86.6),
        c900: hsl(322.1, 100.0, 91.3),
        c950: hsl(315.0, 100.0, 95.9),
    };

    // Rose colors
    pub const ROSE: ColorScale = ColorScale {
        c50: hsl(342.3, 62.9, 21.5),
        c100: hsl(342.8, 68.9, 34.2),
        c200: hsl(344.8, 72.6, 37.3),
        c300: hsl(346.9, 75.8, 43.7),
        c400: hsl(348.2, 80.1, 52.7),
        c500: hsl(350.4, 94.8, 57.5),
        c600: hsl(351.2, 100.0, 58.1),
        c700: hsl(352.3, 100.0, 78.1),
        c800: hsl(352.0, 100.0, 86.2),
        c900: hsl(354.5, 100.0, 90.7),
        c950: hsl(353.3, 100.0, 95.7),
    };

    // Special neutral colors
    pub const NEUTRAL_0: Color = hsl(240.0, 5.9, 11.0);
    pub const NEUTRAL_1000: Color = hsl(0.0, 0.0, 100.0);
}

pub mod light {
    use iced::Color;

    use crate::theme::{pallete::hsl, tokens::ColorScale};

    // Gray colors
    pub const GRAY: ColorScale = ColorScale {
        c50: hsl(0.0, 0.0, 97.5),
        c100: hsl(240.0, 4.8, 95.9),
        c200: hsl(240.0, 5.9, 90.0),
        c300: hsl(240.0, 4.9, 83.9),
        c400: hsl(240.0, 5.0, 64.9),
        c500: hsl(240.0, 3.8, 46.1),
        c600: hsl(240.0, 5.2, 33.9),
        c700: hsl(240.0, 5.3, 26.1),
        c800: hsl(240.0, 3.7, 15.9),
        c900: hsl(240.0, 5.9, 10.0),
        c950: hsl(240.0, 7.3, 8.0),
    };

    // Red colors
    pub const RED: ColorScale = ColorScale {
        c50: hsl(0.0, 85.7, 97.3),
        c100: hsl(0.0, 93.3, 94.1),
        c200: hsl(0.0, 96.3, 89.4),
        c300: hsl(0.0, 93.5, 81.8),
        c400: hsl(0.0, 90.6, 70.8),
        c500: hsl(0.0, 84.2, 60.2),
        c600: hsl(0.0, 72.2, 50.6),
        c700: hsl(0.0, 73.7, 41.8),
        c800: hsl(0.0, 70.0, 35.3),
        c900: hsl(0.0, 62.8, 30.6),
        c950: hsl(0.0, 60.0, 19.6),
    };

    // Orange colors
    pub const ORANGE: ColorScale = ColorScale {
        c50: hsl(33.3, 100.0, 96.5),
        c100: hsl(34.3, 100.0, 91.8),
        c200: hsl(32.1, 97.7, 83.1),
        c300: hsl(30.7, 97.2, 72.4),
        c400: hsl(27.0, 96.0, 61.0),
        c500: hsl(24.6, 95.0, 53.1),
        c600: hsl(20.5, 90.2, 48.2),
        c700: hsl(17.5, 88.3, 40.4),
        c800: hsl(15.0, 79.1, 33.7),
        c900: hsl(15.3, 74.6, 27.8),
        c950: hsl(15.2, 69.1, 19.0),
    };

    // Amber colors
    pub const AMBER: ColorScale = ColorScale {
        c50: hsl(48.0, 100.0, 96.1),
        c100: hsl(48.0, 96.5, 88.8),
        c200: hsl(48.0, 96.6, 76.7),
        c300: hsl(45.9, 96.7, 64.5),
        c400: hsl(43.3, 96.4, 56.3),
        c500: hsl(37.7, 92.1, 50.2),
        c600: hsl(32.1, 94.6, 43.7),
        c700: hsl(26.0, 90.5, 37.1),
        c800: hsl(22.7, 82.5, 31.4),
        c900: hsl(21.7, 77.8, 26.5),
        c950: hsl(22.9, 74.1, 16.7),
    };

    // Yellow colors
    pub const YELLOW: ColorScale = ColorScale {
        c50: hsl(54.5, 91.7, 95.3),
        c100: hsl(54.9, 96.7, 88.0),
        c200: hsl(52.8, 98.3, 76.9),
        c300: hsl(50.4, 97.8, 63.5),
        c400: hsl(47.9, 95.8, 53.1),
        c500: hsl(45.4, 93.4, 47.5),
        c600: hsl(40.6, 96.1, 40.4),
        c700: hsl(35.5, 91.7, 32.9),
        c800: hsl(31.8, 81.0, 28.8),
        c900: hsl(28.4, 72.5, 25.7),
        c950: hsl(33.1, 69.0, 13.9),
    };

    // Lime colors
    pub const LIME: ColorScale = ColorScale {
        c50: hsl(78.3, 92.0, 95.1),
        c100: hsl(79.6, 89.1, 89.2),
        c200: hsl(80.9, 88.5, 79.6),
        c300: hsl(82.0, 84.5, 67.1),
        c400: hsl(82.7, 78.0, 55.5),
        c500: hsl(83.7, 80.5, 44.3),
        c600: hsl(84.8, 85.2, 34.5),
        c700: hsl(85.9, 78.4, 27.3),
        c800: hsl(86.3, 69.0, 22.7),
        c900: hsl(87.6, 61.2, 20.2),
        c950: hsl(86.5, 60.6, 13.9),
    };

    // Green colors
    pub const GREEN: ColorScale = ColorScale {
        c50: hsl(138.5, 76.5, 96.7),
        c100: hsl(140.6, 84.2, 92.5),
        c200: hsl(141.0, 78.9, 85.1),
        c300: hsl(141.7, 76.6, 73.1),
        c400: hsl(141.9, 69.2, 58.0),
        c500: hsl(142.1, 70.6, 45.3),
        c600: hsl(142.1, 76.2, 36.3),
        c700: hsl(142.4, 71.8, 29.2),
        c800: hsl(142.8, 64.2, 24.1),
        c900: hsl(143.8, 61.2, 20.2),
        c950: hsl(144.3, 60.7, 12.0),
    };

    // Emerald colors
    pub const EMERALD: ColorScale = ColorScale {
        c50: hsl(151.8, 81.0, 95.9),
        c100: hsl(149.3, 80.4, 90.0),
        c200: hsl(152.4, 76.0, 80.4),
        c300: hsl(156.2, 71.6, 66.9),
        c400: hsl(158.1, 64.4, 51.6),
        c500: hsl(160.1, 84.1, 39.4),
        c600: hsl(161.4, 93.5, 30.4),
        c700: hsl(162.9, 93.5, 24.3),
        c800: hsl(163.1, 88.1, 19.8),
        c900: hsl(164.2, 85.7, 16.5),
        c950: hsl(164.3, 87.5, 9.4),
    };

    // Teal colors
    pub const TEAL: ColorScale = ColorScale {
        c50: hsl(166.2, 76.5, 96.7),
        c100: hsl(167.2, 85.5, 89.2),
        c200: hsl(168.4, 83.8, 78.2),
        c300: hsl(170.6, 76.9, 64.3),
        c400: hsl(172.5, 66.0, 50.4),
        c500: hsl(173.4, 80.4, 40.0),
        c600: hsl(174.7, 83.9, 31.6),
        c700: hsl(175.3, 77.4, 26.1),
        c800: hsl(176.1, 69.4, 21.8),
        c900: hsl(175.9, 60.8, 19.0),
        c950: hsl(176.5, 58.6, 11.4),
    };

    // Cyan colors
    pub const CYAN: ColorScale = ColorScale {
        c50: hsl(183.2, 100.0, 96.3),
        c100: hsl(185.1, 95.9, 90.4),
        c200: hsl(186.2, 93.5, 81.8),
        c300: hsl(187.0, 92.4, 69.0),
        c400: hsl(187.9, 85.7, 53.3),
        c500: hsl(188.7, 94.5, 42.7),
        c600: hsl(191.6, 91.4, 36.5),
        c700: hsl(192.9, 82.3, 31.0),
        c800: hsl(194.4, 69.6, 27.1),
        c900: hsl(196.4, 63.6, 23.7),
        c950: hsl(196.8, 61.0, 16.1),
    };

    // Sky colors
    pub const SKY: ColorScale = ColorScale {
        c50: hsl(204.0, 100.0, 97.1),
        c100: hsl(204.0, 93.8, 93.7),
        c200: hsl(200.6, 94.4, 86.1),
        c300: hsl(199.4, 95.5, 73.9),
        c400: hsl(198.4, 93.2, 59.6),
        c500: hsl(198.6, 88.7, 48.4),
        c600: hsl(200.4, 98.0, 39.4),
        c700: hsl(201.3, 96.3, 32.2),
        c800: hsl(201.0, 90.0, 27.5),
        c900: hsl(202.0, 80.3, 23.9),
        c950: hsl(202.3, 73.8, 16.5),
    };

    // Blue colors
    pub const BLUE: ColorScale = ColorScale {
        c50: hsl(213.8, 100.0, 96.9),
        c100: hsl(214.3, 94.6, 92.7),
        c200: hsl(213.3, 96.9, 87.3),
        c300: hsl(211.7, 96.4, 78.4),
        c400: hsl(213.1, 93.9, 67.8),
        c500: hsl(217.2, 91.2, 59.8),
        c600: hsl(221.2, 83.2, 53.3),
        c700: hsl(224.3, 76.3, 48.0),
        c800: hsl(225.9, 70.7, 40.2),
        c900: hsl(224.4, 64.3, 32.9),
        c950: hsl(226.2, 55.3, 18.4),
    };

    // Indigo colors
    pub const INDIGO: ColorScale = ColorScale {
        c50: hsl(225.9, 100.0, 96.7),
        c100: hsl(226.5, 100.0, 93.9),
        c200: hsl(228.0, 96.5, 88.8),
        c300: hsl(229.7, 93.5, 81.8),
        c400: hsl(234.5, 89.5, 73.9),
        c500: hsl(238.7, 83.5, 66.7),
        c600: hsl(243.4, 75.4, 58.6),
        c700: hsl(244.5, 57.9, 50.6),
        c800: hsl(243.7, 54.5, 41.4),
        c900: hsl(242.2, 47.4, 34.3),
        c950: hsl(243.5, 43.6, 22.9),
    };

    // Violet colors
    pub const VIOLET: ColorScale = ColorScale {
        c50: hsl(250.0, 100.0, 97.6),
        c100: hsl(251.4, 91.3, 95.5),
        c200: hsl(250.5, 95.2, 91.8),
        c300: hsl(252.5, 94.7, 85.1),
        c400: hsl(255.1, 91.7, 76.3),
        c500: hsl(258.3, 89.5, 66.3),
        c600: hsl(262.1, 83.3, 57.8),
        c700: hsl(263.4, 70.0, 50.4),
        c800: hsl(263.4, 69.3, 42.2),
        c900: hsl(263.5, 67.4, 34.9),
        c950: hsl(265.1, 61.5, 21.4),
    };

    // Purple colors
    pub const PURPLE: ColorScale = ColorScale {
        c50: hsl(270.0, 100.0, 98.0),
        c100: hsl(268.7, 100.0, 95.5),
        c200: hsl(268.6, 100.0, 91.8),
        c300: hsl(269.2, 97.4, 85.1),
        c400: hsl(270.0, 95.2, 75.3),
        c500: hsl(270.7, 91.0, 65.1),
        c600: hsl(271.5, 81.3, 55.9),
        c700: hsl(272.1, 71.7, 47.1),
        c800: hsl(272.9, 67.2, 39.4),
        c900: hsl(273.6, 65.6, 32.0),
        c950: hsl(276.0, 59.5, 16.5),
    };

    // Fuchsia colors
    pub const FUCHSIA: ColorScale = ColorScale {
        c50: hsl(289.1, 100.0, 97.8),
        c100: hsl(287.0, 100.0, 95.5),
        c200: hsl(288.3, 95.8, 90.6),
        c300: hsl(291.1, 93.1, 82.9),
        c400: hsl(292.0, 91.4, 72.5),
        c500: hsl(292.2, 84.1, 60.6),
        c600: hsl(293.4, 69.5, 48.8),
        c700: hsl(294.7, 72.4, 39.8),
        c800: hsl(295.4, 70.2, 32.9),
        c900: hsl(296.7, 63.6, 28.0),
        c950: hsl(297.1, 56.8, 14.5),
    };

    // Pink colors
    pub const PINK: ColorScale = ColorScale {
        c50: hsl(327.3, 73.3, 97.1),
        c100: hsl(325.7, 77.8, 94.7),
        c200: hsl(325.9, 84.6, 89.8),
        c300: hsl(327.4, 87.1, 81.8),
        c400: hsl(328.6, 85.5, 70.2),
        c500: hsl(330.4, 81.2, 60.4),
        c600: hsl(333.3, 71.4, 50.6),
        c700: hsl(335.1, 77.6, 42.0),
        c800: hsl(335.8, 74.4, 35.3),
        c900: hsl(335.9, 69.0, 30.4),
        c950: hsl(336.2, 65.4, 15.9),
    };

    // Rose colors
    pub const ROSE: ColorScale = ColorScale {
        c50: hsl(355.7, 100.0, 97.3),
        c100: hsl(355.6, 100.0, 94.7),
        c200: hsl(352.7, 96.1, 90.0),
        c300: hsl(352.6, 95.7, 81.8),
        c400: hsl(351.3, 94.5, 71.4),
        c500: hsl(349.7, 89.2, 60.2),
        c600: hsl(346.8, 77.2, 49.8),
        c700: hsl(345.3, 82.7, 40.8),
        c800: hsl(343.4, 79.7, 34.7),
        c900: hsl(341.5, 75.5, 30.4),
        c950: hsl(341.3, 70.1, 17.1),
    };

    // Special neutral colors
    pub const NEUTRAL_0: Color = hsl(0.0, 0.0, 100.0);
    pub const NEUTRAL_1000: Color = hsl(0.0, 0.0, 0.0);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorValue {
    C50,
    C100,
    C200,
    C300,
    C400,
    C500,
    C600,
    C700,
    C800,
    C900,
    C950,
}

impl ColorValue {
    pub fn get_color(self, color: &ColorScale) -> Color {
        match self {
            ColorValue::C50 => color.c50,
            ColorValue::C100 => color.c100,
            ColorValue::C200 => color.c200,
            ColorValue::C300 => color.c300,
            ColorValue::C400 => color.c400,
            ColorValue::C500 => color.c500,
            ColorValue::C600 => color.c600,
            ColorValue::C700 => color.c700,
            ColorValue::C800 => color.c800,
            ColorValue::C900 => color.c900,
            ColorValue::C950 => color.c950,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorVariant {
    Gray,
    Red,
    Orange,
    Amber,
    Yellow,
    Lime,
    Green,
    Emerald,
    Teal,
    Cyan,
    Sky,
    Blue,
    Indigo,
    Violet,
    Purple,
    Fuchsia,
    Pink,
    Rose,
    Neutral,
    NeutralBase,
    NeutralDark,
}

impl ColorVariant {
    pub fn get_color(self, color: Tokens, value: ColorValue) -> Color {
        let color_scale = match self {
            ColorVariant::Gray => color.gray,
            ColorVariant::Red => color.red,
            ColorVariant::Orange => color.orange,
            ColorVariant::Amber => color.amber,
            ColorVariant::Yellow => color.yellow,
            ColorVariant::Lime => color.lime,
            ColorVariant::Green => color.green,
            ColorVariant::Emerald => color.emerald,
            ColorVariant::Teal => color.teal,
            ColorVariant::Cyan => color.cyan,
            ColorVariant::Sky => color.sky,
            ColorVariant::Blue => color.blue,
            ColorVariant::Indigo => color.indigo,
            ColorVariant::Violet => color.violet,
            ColorVariant::Purple => color.purple,
            ColorVariant::Fuchsia => color.fuchsia,
            ColorVariant::Pink => color.pink,
            ColorVariant::Rose => color.rose,
            ColorVariant::Neutral => color.neutral,
            ColorVariant::NeutralBase => return color.neutral_0,
            ColorVariant::NeutralDark => return color.neutral_1000,
        };

        value.get_color(&color_scale)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColorToken {
    variant: ColorVariant,
    value: ColorValue,
}

impl ColorToken {
    pub fn new(variant: ColorVariant, value: ColorValue) -> Self {
        Self { variant, value }
    }

    pub fn get_color(self, color: Tokens) -> Color {
        self.variant.get_color(color, self.value)
    }
}

pub(crate) const DARK: Tokens = Tokens {
    // Color scales
    gray: dark::GRAY,
    red: dark::RED,
    orange: dark::ORANGE,
    amber: dark::AMBER,
    yellow: dark::YELLOW,
    lime: dark::LIME,
    green: dark::GREEN,
    emerald: dark::EMERALD,
    teal: dark::TEAL,
    cyan: dark::CYAN,
    sky: dark::SKY,
    blue: dark::BLUE,
    indigo: dark::INDIGO,
    violet: dark::VIOLET,
    purple: dark::PURPLE,
    fuchsia: dark::FUCHSIA,
    pink: dark::PINK,
    rose: dark::ROSE,

    // Semantic colors
    primary: dark::SKY,
    success: dark::GREEN,
    warning: dark::AMBER,
    danger: dark::RED,
    neutral: dark::GRAY,

    // Special neutral colors
    neutral_0: dark::NEUTRAL_0,
    neutral_1000: dark::NEUTRAL_1000,
};

pub(crate) const LIGHT: Tokens = Tokens {
    // Color scales
    gray: light::GRAY,
    red: light::RED,
    orange: light::ORANGE,
    amber: light::AMBER,
    yellow: light::YELLOW,
    lime: light::LIME,
    green: light::GREEN,
    emerald: light::EMERALD,
    teal: light::TEAL,
    cyan: light::CYAN,
    sky: light::SKY,
    blue: light::BLUE,
    indigo: light::INDIGO,
    violet: light::VIOLET,
    purple: light::PURPLE,
    fuchsia: light::FUCHSIA,
    pink: light::PINK,
    rose: light::ROSE,

    // Semantic colors
    primary: light::SKY,
    success: light::GREEN,
    warning: light::AMBER,
    danger: light::RED,
    neutral: light::GRAY,

    // Special neutral colors
    neutral_0: light::NEUTRAL_0,
    neutral_1000: light::NEUTRAL_1000,
};
