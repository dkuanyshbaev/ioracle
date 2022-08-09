// ---------------------------------------
// IOracle iching
// ---------------------------------------
pub struct Trigram(u8, u8, u8);
pub struct Hexagram(Trigram, Trigram);
impl Hexagram {
    pub fn new(first: (u8, u8, u8), second: (u8, u8, u8)) -> Self {
        let t1: Trigram = Trigram(first.0, first.1, first.2);
        let t2: Trigram = Trigram(second.0, second.1, second.2);
        Self(t1, t2)
    }
}

// pub type Hexagram = String;

pub struct Answer {
    pub question: String,
    pub answer: String,
    pub hexagram: Hexagram,
}

impl Answer {
    pub fn new(hexagram: Hexagram, question: String) -> Self {
        // ---------------------------------------
        // TODO: generate answer to question!
        // ---------------------------------------
        let answer = "42".to_string();
        // ---------------------------------------

        Answer {
            question,
            answer,
            hexagram,
        }
    }
    pub fn get_by_id(_id: u64) -> Self {
        // ---------------------------------------
        // TODO: db search, return answer
        // ---------------------------------------

        Answer {
            question: "question".to_string(),
            answer: "42".to_string(),
            hexagram: Hexagram(Trigram(1, 1, 1), Trigram(0, 0, 0)),
        }
    }
    pub fn save(self) -> u64 {
        // ---------------------------------------
        // TODO: save to db, return id
        // ---------------------------------------

        42
    }
}
