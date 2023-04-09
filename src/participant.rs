use crate::conversation;
use crate::occupation;

pub struct Participant {
    pub name: String,
    pub age: i32,
    mood: i32,
    occupation: occupation::OccupationEnum,
    communication: i32,
    traits: Vec<String>,
}

impl Participant {
    pub fn new(
        name: String,
        age: i32,
        occupation: occupation::OccupationEnum,
        traits: Vec<String>,
    ) -> Participant {
        Participant {
            name,
            age,
            mood: 100,
            occupation,
            communication: 50,
            traits,
        }
    }

    pub fn greet(&self, c: &mut conversation::Conversation, s: Participant, t: Participant) {
        c.SourceParticipant = s;
        c.TargetParticipant = t;
    }
}
