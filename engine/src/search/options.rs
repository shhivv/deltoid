use std::str::SplitAsciiWhitespace;

#[allow(clippy::module_name_repetitions)]
pub enum SearchOptions {
    Depth(u8),
    MoveTime(u32),
    Nodes(u128),
    Time(u32, u32, Option<u32>, Option<u32>, Option<u32>),
    Infinite,
}

impl SearchOptions {
    #[allow(clippy::similar_names)]
    pub fn parse(input: &mut SplitAsciiWhitespace) -> Self {
        let mut wtime = None;
        let mut btime = None;
        let mut winc = None;
        let mut binc = None;
        let mut movestogo = None;
        let mut depth = None;
        let mut nodes = None;
        let mut movetime = None;
        let mut infinite = None;

        loop {
            let (k, v) = (
                input.next().unwrap_or_default(),
                input.next().unwrap_or_default(),
            );
            match (k, v) {
                ("wtime", n) => wtime = n.parse::<u32>().ok(),
                ("btime", n) => btime = n.parse::<u32>().ok(),
                ("winc", n) => winc = n.parse::<u32>().ok(),
                ("binc", n) => binc = n.parse::<u32>().ok(),
                ("movestogo", n) => movestogo = n.parse::<u32>().ok(),
                ("depth", n) => depth = n.parse::<u8>().ok(),
                ("nodes", n) => nodes = n.parse::<u128>().ok(),
                ("movetime", n) => movetime = n.parse::<u32>().ok(),
                ("infinite", _) => infinite = Some(true),
                ("", _) => break,
                (_, _) => {}
            }
        }

        match (wtime, btime, depth, nodes, movetime, infinite) {
            (Some(wt), Some(bt), None, None, None, None) => {
                Self::Time(wt, bt, winc, binc, movestogo)
            }
            (None, None, Some(d), None, None, None) => Self::Depth(d),
            (None, None, None, Some(n), None, None) => Self::Nodes(n),
            (None, None, None, None, Some(m), None) => Self::MoveTime(m),
            _ => Self::Infinite,
        }
    }
}
