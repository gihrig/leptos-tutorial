use csscolorparser::Color;

pub fn get_theme() -> Result<Theme, csscolorparser::ParseColorError> {
    let theme = Theme {
        teal: Colors {
            main: Color::from_html("#6FDDDB")?,
            darker: Color::from_html("#2BB4B2")?,
            lighter: Color::from_html("#7EE1DF")?,
            lightest: Color::from_html("#B2EDEC")?,
        },
        pink: Colors {
            main: Color::from_html("#E93EF5")?,
            darker: Color::from_html("#C70BD4")?,
            lighter: Color::from_html("#F5A4FA")?,
            lightest: Color::from_html("#FCE1FD")?,
        },
        green: Colors {
            main: Color::from_html("#54D072")?,
            darker: Color::from_html("#30AF4F")?,
            lighter: Color::from_html("#82DD98")?,
            lightest: Color::from_html("#B4EAC1")?,
        },
        purple: Colors {
            main: Color::from_html("#8C18FB")?,
            darker: Color::from_html("#7204DB")?,
            lighter: Color::from_html("#B162FC")?,
            lightest: Color::from_html("#D0A1FD")?,
        },
        yellow: Colors {
            main: Color::from_html("#E1E862")?,
            darker: Color::from_html("#BAC31D")?,
            lighter: Color::from_html("#EFF3AC")?,
            lightest: Color::from_html("#FAFBE3")?,
        },
        gray: Colors {
            main: Color::from_html("#4a4a4a")?,
            darker: Color::from_html("#3d3d3d")?,
            lighter: Color::from_html("#939393")?,
            lightest: Color::from_html("#c4c4c4")?,
        },
        red: Color::from_html("#FF5854")?,
        black: Color::from_html("#000000")?,
        white: Color::from_html("#FFFFFF")?,
        transparent: Color::from_html("transparent")?,
    };

    Ok(theme)
}

pub struct Theme {
    pub teal: Colors,
    pub pink: Colors,
    pub green: Colors,
    pub purple: Colors,
    pub yellow: Colors,
    pub gray: Colors,
    pub red: Color,
    pub black: Color,
    pub white: Color,
    pub transparent: Color,
}

pub struct Colors {
    pub main: Color,
    pub darker: Color,
    pub lighter: Color,
    pub lightest: Color,
}

impl Colors {
    pub fn main(&self) -> String {
        self.main.to_hex_string()
    }
    pub fn darker(&self) -> String {
        self.darker.to_hex_string()
    }
    pub fn lighter(&self) -> String {
        self.lighter.to_hex_string()
    }
    pub fn lightest(&self) -> String {
        self.lightest.to_hex_string()
    }
}

impl Theme {
    pub fn red(&self) -> String {
        self.red.to_hex_string()
    }
    pub fn green(&self) -> String {
        self.green.darker.to_hex_string()
    }
    pub fn purple(&self) -> String {
        self.purple.main.to_hex_string()
    }
    pub fn teal(&self) -> String {
        self.teal.darker.to_hex_string()
    }
    pub fn black(&self) -> String {
        self.black.to_hex_string()
    }
    pub fn white(&self) -> String {
        self.white.to_hex_string()
    }
    pub fn transparent(&self) -> String {
        self.transparent.to_hex_string()
    }
}
