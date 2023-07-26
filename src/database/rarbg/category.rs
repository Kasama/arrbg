use std::fmt::Display;
use std::marker::PhantomData;
use std::str::FromStr;

use serde::de::Visitor;
use serde::Deserialize;
use tracing::debug;

#[derive(Debug, Clone)]
#[allow(clippy::upper_case_acronyms)]
pub enum Category {
    XXX = 4,
    MoviesX264 = 17,
    MoviesX264_1080 = 44,
    MoviesX264_720 = 45,
    MoviesX264_3d = 47,
    MoviesX264_4k = 50,
    MoviesX265_4k = 51,
    MoviesX265_4kHdr = 52,
    MoviesXvid = 14,
    MoviesXvid720 = 48,
    MoviesFullBd = 42,
    MoviesBdRemux = 46,
    MoviesX265_1080 = 54,
    TvEpisodes = 18,
    TvHdEpisodes = 41,
    TvUhdEpisodes = 49,
    MusicMp3 = 23,
    MusicFlac = 25,
    GamesPcIso = 27,
    GamesPcRip = 28,
    SoftwarePcIso = 33,
    GamesPs4 = 53,
}

impl Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.name())
    }
}

impl<'de> Deserialize<'de> for Category {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct StringOrInt<T>(PhantomData<T>);

        impl<'de> Visitor<'de> for StringOrInt<Category> {
            type Value = Category;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("string or integer")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                debug!("Visiting string");
                let o: Result<i64, _> = v.parse();
                match o {
                    Ok(i) => self.visit_i64(i),
                    Err(_) => FromStr::from_str(v).map_err(serde::de::Error::custom),
                }
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                debug!("Visiting i64");
                v.try_into().map_err(serde::de::Error::custom)
            }
        }

        debug!("Got to the deserializer");
        deserializer.deserialize_any(StringOrInt(PhantomData))
    }
}

impl TryFrom<i64> for Category {
    type Error = anyhow::Error;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            4 => Ok(Self::XXX),
            14 => Ok(Self::MoviesXvid),
            48 => Ok(Self::MoviesXvid720),
            17 => Ok(Self::MoviesX264),
            44 => Ok(Self::MoviesX264_1080),
            45 => Ok(Self::MoviesX264_720),
            47 => Ok(Self::MoviesX264_3d),
            50 => Ok(Self::MoviesX264_4k),
            51 => Ok(Self::MoviesX265_4k),
            52 => Ok(Self::MoviesX265_4kHdr),
            42 => Ok(Self::MoviesFullBd),
            46 => Ok(Self::MoviesBdRemux),
            54 => Ok(Self::MoviesX265_1080),
            18 => Ok(Self::TvEpisodes),
            41 => Ok(Self::TvHdEpisodes),
            49 => Ok(Self::TvUhdEpisodes),
            23 => Ok(Self::MusicMp3),
            25 => Ok(Self::MusicFlac),
            27 => Ok(Self::GamesPcIso),
            28 => Ok(Self::GamesPcRip),
            33 => Ok(Self::SoftwarePcIso),
            53 => Ok(Self::GamesPs4),
            _ => Err(anyhow::anyhow!("Invalid category: {}", value)),
        }
    }
}

impl TryFrom<&str> for Category {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Ok(i) = value.parse::<i64>() {
            return Self::try_from(i);
        };
        match value {
            "xxx" => Ok(Category::XXX),
            "movies_x264" => Ok(Category::MoviesX264),
            "movies_x264_1080" => Ok(Category::MoviesX264_1080),
            "movies_x264_720" => Ok(Category::MoviesX264_720),
            "movies_x264_3d" => Ok(Category::MoviesX264_3d),
            "movies_x264_4k" => Ok(Category::MoviesX264_4k),
            "movies_x265_4k" => Ok(Category::MoviesX265_4k),
            "movies_x265_4k_hdr" => Ok(Category::MoviesX265_4kHdr),
            "movies_full_bd" => Ok(Category::MoviesFullBd),
            "movies_bd_remux" => Ok(Category::MoviesBdRemux),
            "movies_x265_1080" => Ok(Category::MoviesX265_1080),
            "movies_xvid" => Ok(Category::MoviesXvid),
            "movies_xvid_720" => Ok(Category::MoviesXvid720),
            "tv" => Ok(Category::TvEpisodes),
            "tv_sd" => Ok(Category::TvHdEpisodes),
            "tv_uhd" => Ok(Category::TvUhdEpisodes),
            "music_mp3" => Ok(Category::MusicMp3),
            "music_flac" => Ok(Category::MusicFlac),
            "games_pc_iso" => Ok(Category::GamesPcIso),
            "games_pc_rip" => Ok(Category::GamesPcRip),
            "software_pc_iso" => Ok(Category::SoftwarePcIso),
            "games_ps4" => Ok(Category::GamesPs4),
            _ => Err(anyhow::anyhow!("Invalid category: '{}'", value)),
        }
    }
}

impl FromStr for Category {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.try_into()
    }
}

impl Category {
    pub fn name(&self) -> String {
        match self {
            Category::XXX => "xxx",
            Category::MoviesX264 => "movies_x264",
            Category::MoviesX264_1080 => "movies_x264_1080",
            Category::MoviesX264_720 => "movies_x264_720",
            Category::MoviesX264_3d => "movies_x264_3d",
            Category::MoviesX264_4k => "movies_x264_4k",
            Category::MoviesX265_4k => "movies_x265_4k",
            Category::MoviesX265_4kHdr => "movies_x265_4k_hdr",
            Category::MoviesFullBd => "movies_full_bd",
            Category::MoviesBdRemux => "movies_bd_remux",
            Category::MoviesX265_1080 => "movies_x265_1080",
            Category::MoviesXvid => "movies_xvid",
            Category::MoviesXvid720 => "movies_xvid_720",
            Category::TvEpisodes => "tv",
            Category::TvHdEpisodes => "tv_sd",
            Category::TvUhdEpisodes => "tv_uhd",
            Category::MusicMp3 => "music_mp3",
            Category::MusicFlac => "music_flac",
            Category::GamesPcIso => "games_pc_iso",
            Category::GamesPcRip => "games_pc_rip",
            Category::SoftwarePcIso => "software_pc_iso",
            Category::GamesPs4 => "games_ps4",
        }
        .to_string()
    }
}
