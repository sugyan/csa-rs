use std::fmt;
use std::time::Duration;
use time::{Date as NativeDate, Time as NativeTime};

#[derive(Default, Debug, PartialEq, Eq)]
pub struct GameRecord {
    pub black_player: Option<String>,
    pub white_player: Option<String>,
    pub event: Option<String>,
    pub site: Option<String>,
    pub start_time: Option<Time>,
    pub end_time: Option<Time>,
    pub time_limit: Option<TimeLimit>,
    pub opening: Option<String>,
    pub start_pos: Position,
    pub moves: Vec<MoveRecord>,
}

impl fmt::Display for GameRecord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "V2.2")?;

        // Metadata
        let metadata = [
            ("N+", self.black_player.as_ref().map(|x| x.to_string())),
            ("N-", self.white_player.as_ref().map(|x| x.to_string())),
            ("$EVENT:", self.event.as_ref().map(|x| x.to_string())),
            ("$SITE:", self.site.as_ref().map(|x| x.to_string())),
            (
                "$START_TIME:",
                self.start_time.as_ref().map(|x| x.to_string()),
            ),
            ("$END_TIME:", self.end_time.as_ref().map(|x| x.to_string())),
            (
                "$TIME_LIMIT:",
                self.time_limit.as_ref().map(|x| x.to_string()),
            ),
            ("$OPENING:", self.opening.as_ref().map(|x| x.to_string())),
        ];
        for &(ref key, ref value) in &metadata {
            if let Some(ref value) = *value {
                writeln!(f, "{}{}", key, value)?;
            }
        }

        // Position
        write!(f, "{}", self.start_pos)?;

        // Move records
        for record in &self.moves {
            write!(f, "{}", record)?;
        }

        Ok(())
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct Time {
    pub date: NativeDate,
    pub time: Option<NativeTime>,
}

impl Time {
    pub fn now() -> Self {
        let now = time::OffsetDateTime::now_utc();

        Time {
            date: now.date(),
            time: Some(now.time()),
        }
    }
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}/{:02}/{:02}",
            self.date.year(),
            self.date.month() as u8,
            self.date.day()
        )?;
        if let Some(time) = self.time {
            write!(
                f,
                " {}:{:02}:{:02}",
                time.hour(),
                time.minute(),
                time.second()
            )?;
        }

        Ok(())
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TimeLimit {
    pub main_time: Duration,
    pub byoyomi: Duration,
}

impl fmt::Display for TimeLimit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let secs = self.main_time.as_secs();
        let hours = secs / 3600;
        let minutes = (secs % 3600) / 60;

        write!(
            f,
            "{:02}:{:02}+{:02}",
            hours,
            minutes,
            self.byoyomi.as_secs()
        )
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, PartialEq, Eq)]
pub enum GameAttribute {
    Time(Time),
    TimeLimit(TimeLimit),
    Str(String),
}

impl fmt::Display for GameAttribute {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GameAttribute::Time(ref time) => write!(f, "{}", time),
            GameAttribute::TimeLimit(ref time_limit) => write!(f, "{}", time_limit),
            GameAttribute::Str(ref s) => write!(f, "{}", s),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Color {
    Black,
    White,
}

impl Default for Color {
    fn default() -> Self {
        Color::Black
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Color::Black => write!(f, "+"),
            Color::White => write!(f, "-"),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Square {
    pub file: u8,
    pub rank: u8,
}

impl Square {
    pub fn new(file: u8, rank: u8) -> Square {
        Square { file, rank }
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.file, self.rank)
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PieceType {
    Pawn,
    Lance,
    Knight,
    Silver,
    Gold,
    Bishop,
    Rook,
    King,
    ProPawn,
    ProLance,
    ProKnight,
    ProSilver,
    Horse,
    Dragon,
    All,
}

impl fmt::Display for PieceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let pt = match *self {
            PieceType::Pawn => "FU",
            PieceType::Lance => "KY",
            PieceType::Knight => "KE",
            PieceType::Silver => "GI",
            PieceType::Gold => "KI",
            PieceType::Bishop => "KA",
            PieceType::Rook => "HI",
            PieceType::King => "OU",
            PieceType::ProPawn => "TO",
            PieceType::ProLance => "NY",
            PieceType::ProKnight => "NK",
            PieceType::ProSilver => "NG",
            PieceType::Horse => "UM",
            PieceType::Dragon => "RY",
            PieceType::All => "AL",
        };
        write!(f, "{}", pt)
    }
}

////////////////////////////////////////////////////////////////////////////////

type Board = [[Option<(Color, PieceType)>; 9]; 9];

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Position {
    pub drop_pieces: Vec<(Square, PieceType)>,
    pub bulk: Option<Board>,
    pub add_pieces: Vec<(Color, Square, PieceType)>,
    pub side_to_move: Color,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(ref bulk) = self.bulk {
            for (i, row) in bulk.iter().enumerate() {
                write!(f, "P{}", i + 1)?;

                for pc in row.iter() {
                    match *pc {
                        Some((ref color, ref pt)) => write!(f, "{}{}", color, pt)?,
                        None => write!(f, " * ")?,
                    }
                }

                writeln!(f)?;
            }
        } else {
            write!(f, "PI")?;
            for &(ref sq, ref pt) in &self.drop_pieces {
                write!(f, "{}{}", sq, pt)?;
            }
            writeln!(f)?;
        }

        for &(ref color, ref sq, ref pt) in &self.add_pieces {
            writeln!(f, "P{}{}{}", color, sq, pt)?;
        }

        writeln!(f, "{}", self.side_to_move)?;

        Ok(())
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Action {
    Move(Color, Square, Square, PieceType),
    Toryo,
    Chudan,
    Sennichite,
    TimeUp,
    IllegalMove,
    IllegalAction(Color),
    Jishogi,
    Kachi,
    Hikiwake,
    Matta,
    Tsumi,
    Fuzumi,
    Error,
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Action::Move(ref color, ref from, ref to, ref pt) => {
                write!(f, "{}{}{}{}", color, from, to, pt)
            }
            Action::Toryo => write!(f, "%TORYO"),
            Action::Chudan => write!(f, "%CHUDAN"),
            Action::Sennichite => write!(f, "%SENNICHITE"),
            Action::TimeUp => write!(f, "%TIME_UP"),
            Action::IllegalMove => write!(f, "%ILLEGAL_MOVE"),
            Action::IllegalAction(ref color) => write!(f, "%{}ILLEGAL_ACTION", color),
            Action::Jishogi => write!(f, "%JISHOGI"),
            Action::Kachi => write!(f, "%KACHI"),
            Action::Hikiwake => write!(f, "%HIKIWAKE"),
            Action::Matta => write!(f, "%MATTA"),
            Action::Tsumi => write!(f, "%TSUMI"),
            Action::Fuzumi => write!(f, "%FUZUMI"),
            Action::Error => write!(f, "%ERROR"),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, PartialEq, Eq)]
pub struct MoveRecord {
    pub action: Action,
    pub time: Option<Duration>,
}

impl fmt::Display for MoveRecord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.action)?;

        if let Some(ref time) = self.time {
            writeln!(f, "T{}", time.as_secs())?;
        }

        Ok(())
    }
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn piece_type() {
        assert_eq!(&PieceType::Pawn.to_string(), "FU");
        assert_eq!(&PieceType::Lance.to_string(), "KY");
        assert_eq!(&PieceType::Knight.to_string(), "KE");
        assert_eq!(&PieceType::Silver.to_string(), "GI");
        assert_eq!(&PieceType::Gold.to_string(), "KI");
        assert_eq!(&PieceType::Bishop.to_string(), "KA");
        assert_eq!(&PieceType::Rook.to_string(), "HI");
        assert_eq!(&PieceType::King.to_string(), "OU");
        assert_eq!(&PieceType::ProPawn.to_string(), "TO");
        assert_eq!(&PieceType::ProLance.to_string(), "NY");
        assert_eq!(&PieceType::ProKnight.to_string(), "NK");
        assert_eq!(&PieceType::ProSilver.to_string(), "NG");
        assert_eq!(&PieceType::Horse.to_string(), "UM");
        assert_eq!(&PieceType::Dragon.to_string(), "RY");
        assert_eq!(&PieceType::All.to_string(), "AL");
    }

    #[test]
    fn action() {
        assert_eq!(
            &Action::Move(
                Color::Black,
                Square::new(7, 7),
                Square::new(7, 6),
                PieceType::Pawn,
            )
            .to_string(),
            "+7776FU"
        );

        assert_eq!(&Action::Toryo.to_string(), "%TORYO");
        assert_eq!(&Action::Chudan.to_string(), "%CHUDAN");
        assert_eq!(&Action::Sennichite.to_string(), "%SENNICHITE");
        assert_eq!(&Action::TimeUp.to_string(), "%TIME_UP");
        assert_eq!(&Action::IllegalMove.to_string(), "%ILLEGAL_MOVE");
        assert_eq!(
            &Action::IllegalAction(Color::Black).to_string(),
            "%+ILLEGAL_ACTION"
        );
        assert_eq!(
            &Action::IllegalAction(Color::White).to_string(),
            "%-ILLEGAL_ACTION"
        );
        assert_eq!(&Action::Jishogi.to_string(), "%JISHOGI");
        assert_eq!(&Action::Kachi.to_string(), "%KACHI");
        assert_eq!(&Action::Hikiwake.to_string(), "%HIKIWAKE");
        assert_eq!(&Action::Matta.to_string(), "%MATTA");
        assert_eq!(&Action::Tsumi.to_string(), "%TSUMI");
        assert_eq!(&Action::Fuzumi.to_string(), "%FUZUMI");
        assert_eq!(&Action::Error.to_string(), "%ERROR");
    }

    #[test]
    fn game_record() {
        let mut g = GameRecord {
            black_player: Some("NAKAHARA".to_string()),
            white_player: Some("YONENAGA".to_string()),
            event: Some("13th World Computer Shogi Championship".to_string()),
            site: Some("KAZUSA ARC".to_string()),
            start_time: Some(Time {
                date: time::Date::from_calendar_date(2003, time::Month::May, 3).unwrap(),
                time: Some(time::Time::from_hms(10, 30, 0).unwrap()),
            }),
            end_time: Some(Time {
                date: time::Date::from_calendar_date(2003, time::Month::May, 3).unwrap(),
                time: Some(time::Time::from_hms(11, 11, 5).unwrap()),
            }),
            time_limit: Some(TimeLimit {
                main_time: Duration::from_secs(1500),
                byoyomi: Duration::from_secs(0),
            }),
            opening: Some("YAGURA".to_string()),
            ..GameRecord::default()
        };
        g.moves.push(MoveRecord {
            action: Action::Move(
                Color::Black,
                Square::new(8, 7),
                Square::new(8, 6),
                PieceType::Pawn,
            ),
            time: Some(Duration::from_secs(5)),
        });
        g.moves.push(MoveRecord {
            action: Action::Toryo,
            time: None,
        });

        let csa = "\
V2.2
N+NAKAHARA
N-YONENAGA
$EVENT:13th World Computer Shogi Championship
$SITE:KAZUSA ARC
$START_TIME:2003/05/03 10:30:00
$END_TIME:2003/05/03 11:11:05
$TIME_LIMIT:00:25+00
$OPENING:YAGURA
PI
+
+8786FU
T5
%TORYO
";

        assert_eq!(csa, g.to_string());
    }
}
