/// TODO:
#[macro_export]
macro_rules! lift_effect {
    ($entity:expr, $effect_expr:expr) => {{
        match $effect_expr {
            ::eventsourced::CommandEffect::EmitAndReply(events, reply) => {
                let result = reply($entity);
                ::eventsourced::CommandEffect::EmitAndReply(events, Box::new(move |_| result))
            }
            ::eventsourced::CommandEffect::Reply(reply) => {
                ::eventsourced::CommandEffect::Reply(reply)
            }
            ::eventsourced::CommandEffect::Reject(err) => {
                ::eventsourced::CommandEffect::Reject(err)
            }
        }
    }};
}
