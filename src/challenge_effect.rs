use konnektoren_core::challenges::challenge_config::ChallengeVariant;
use konnektoren_core::prelude::{Challenge, ChallengeResult};
use konnektoren_yew::components::challenge::ChallengeEvent;
use konnektoren_yew::components::{ChallengeComponent, MusicComponent};
use konnektoren_yew::effects::BlinkAnimation;
use std::time::Duration;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub challenge: Challenge,
    #[prop_or_default]
    pub variant: Option<ChallengeVariant>,
    #[prop_or_default]
    pub on_finish: Option<Callback<ChallengeResult>>,
}

#[function_component(ChallengeEffectComponent)]
pub fn challenge_effect_component(props: &Props) -> Html {
    let challenge = props.challenge.clone();
    let effect_ref = use_state(|| html! { <div></div> });
    let counter = use_state(|| 0);

    let handle_finish = {
        let on_finish = props.on_finish.clone();
        Callback::from(move |result: ChallengeResult| {
            if let Some(on_finish) = on_finish.as_ref() {
                on_finish.emit(result.clone());
            }
        })
    };

    let handle_event = {
        let effect_ref = effect_ref.clone();
        let counter = counter.clone();
        Callback::from(move |event: ChallengeEvent| match event {
            ChallengeEvent::Finish(..) => {}
            ChallengeEvent::NextTask(_index) => {}
            ChallengeEvent::PreviousTask(_index) => {}
            ChallengeEvent::SolvedCorrect(_index) => {
                let c = *counter + 1;
                counter.set(c);
                effect_ref.set(html! {
                    <>
                    <MusicComponent id={format!("music-effect-{}", c)} url="assets/music/UI Positive Signal 002.wav" repeat={false} />
                    <BlinkAnimation target_id={format!("challenge-effect-{}", c)} duration={Duration::from_millis(800)} color={"green"} />
                    </>
                });
            }
            ChallengeEvent::SolvedIncorrect(_index) => {
                let c = *counter + 1;
                counter.set(c);
                effect_ref.set(html! {
                    <>
                    <MusicComponent id={format!("music-effect-{}", c)} url="assets/music/UI Negative Signal 003.wav" repeat={false} />
                    <BlinkAnimation target_id={format!("challenge-effect-{}", c)} duration={Duration::from_millis(800)} color={"red"} />
                    </>
                });
            }
        })
    };

    html! {
        <div id={format!("challenge-effect-{}", *counter)}>
        {(*effect_ref).clone()}
        <ChallengeComponent challenge={challenge.clone()} variant={props.variant.clone()} on_finish={handle_finish} on_event={handle_event} />
        </div>
    }
}
