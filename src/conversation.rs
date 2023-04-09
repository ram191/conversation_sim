use crate::participant;

pub struct Conversation {
    pub SourceParticipant: participant::Participant,
    pub TargetParticipant: participant::Participant,
    Topic: String,
    Interest: i32,
    State: ConversationState,
}

pub enum ConversationState {
    Start,
    Waiting,
    SourceSpeaking,
    TargetSpeaking,
    End,
}
